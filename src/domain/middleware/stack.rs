// Copyright (c) 2022 André de Moraes
//
// This file is part of Austenite, a Rust integration framework built by André de Moraes. Austenite is 
// free software; you can redistribute it and/or modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 3 of the License, or (at your option) any later version.
//
// Austenite is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the
// implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General
// Public License for more details. You should have received a copy of the GNU Lesser General Public
// License along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// You should have received a copy of the GNU General Public License along with Austenite. If not, see
// <http://www.gnu.org/licenses/>.

use std::{any::{Any}};

use super::{Middleware};

/// Representaiton for a Middleware Stack
#[derive(Debug)]
pub struct Stack {
    middlewares: Vec<Box<dyn Middleware>>
}

impl<'message> Stack {
    /// Returns a Middleware Stack
    /// 
    /// # Arguments
    /// 
    /// * `middlewares` - An optional vector of boxes around the dynamic implementation of the trait Middleware
    /// 
    /// # Examples
    /// 
    /// '''
    /// let stack = Stack::new(
    ///     Some(
    ///         vec![
    ///             Box::new(ExampleMiddleware::new())
    ///         ]
    ///     )
    /// )
    /// '''
    pub fn new(middlewares: Option<Vec<Box<dyn Middleware>>>) -> Self {
        let middlewares: Vec<Box<dyn Middleware>> = middlewares.unwrap_or(Vec::new());
        Self {
            middlewares
        }
    }

    fn exists(&self, middleware: &Box<dyn Middleware>) -> Option<usize> {
        if self.middlewares.len() > 0 {
            if let Some(idx) = self.middlewares.iter().position(|x| x.type_id() == middleware.type_id()) {
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn attach(&mut self, middleware: Box<dyn Middleware>) -> &mut Self {
        match self.exists(&middleware) {
            Some(_) => println!("Middleware {} already exists in stack", middleware),
            None => self.middlewares.push(middleware)
        };
        self
    }

    pub fn dettach(&mut self, middleware: Box<dyn Middleware>) -> &mut Self {
        match self.exists(&middleware) {
            Some(idx) => { self.middlewares.remove(idx); },
            None => { println!("Middleware {} didn't exist in stack", middleware); }
        };
        self
    }

    pub fn handle(&self, message: &'message &str) -> Result<&'message str, &str> {
        let mut msg: &str = message;
        let mut err: &str = "";
        for middleware in self.middlewares.iter().rev() {
            match middleware.handle(msg) {
                Ok(message) => { msg = message; },
                Err(error) => { err = error; break },
            }
        }
        if err.len() > 0 {
            return Err(err)
        }
        Ok(msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug, Clone, Copy)]
    struct FakeMiddlewareA {}

    impl FakeMiddlewareA {
        fn new() -> Self {
            Self {}
        }
    }

    impl Middleware for FakeMiddlewareA {
        fn handle<'message>(&self, _message: &'message str) -> Result<&'message str, &str> {
            Err("I always fail!")
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    struct FakeMiddlewareB {}

    impl FakeMiddlewareB {
        fn new() -> Self {
            Self {}
        }
    }

    impl Middleware for FakeMiddlewareB {
        fn handle<'message>(&self, message: &'message str) -> Result<&'message str, &str> {
            Ok(message)
        }
    }

    #[test]
    fn stack_initiates_empty() {
        let stack = Stack::new(None);
        assert_eq!(stack.middlewares.len(), 0);
    }

    #[test]
    fn stack_initializes_with_middlewares_list() {
        let stack = Stack::new(
            Some(
                vec![
                    Box::new(FakeMiddlewareA::new()),
                    Box::new(FakeMiddlewareB::new())
                ]
            ));
        assert!(stack.middlewares.len() > 0);
    }

    #[test]
    fn stack_checks_if_middleware_exists() {
        let mut stack = Stack::new(None);
        stack.attach(Box::new(FakeMiddlewareA::new()));
        assert_eq!(stack.middlewares.len(), 1);
    }

    #[test]
    fn stack_disallow_adding_same_middleware() {
        let mut stack = Stack::new(None);
        let middleware_a = Box::new(FakeMiddlewareA::new());
        let middleware_b = Box::new(FakeMiddlewareA::new());

        stack
        .attach(middleware_a)
        .attach(middleware_b);
        assert_eq!(stack.middlewares.len(), 1);
    }

    #[test]
    fn stack_dettaches_existing_middleware() {
        let middleware = Box::new(FakeMiddlewareA::new());
        let mut stack = Stack::new(
            Some(vec![
                Box::new(FakeMiddlewareA::new())
            ]));
        stack.dettach(middleware);
        assert_eq!(stack.middlewares.len(), 0);
    }

    #[test]
    fn stack_results_ok_if_middlewares_succeeds() {
        let stack = Stack::new(
            Some(
                vec![Box::new(FakeMiddlewareB::new())]
            )
        );
        let message = "A message";
        dbg!(message);
        match stack.handle(&message) {
            Ok(msg) => { assert_eq!(msg, message); },
            Err(_) => ()
        }
    }

    #[test]
    fn stack_results_err_if_at_least_one_middleware_fails() {
        let stack = Stack::new(
            Some(
                vec![
                    Box::new(FakeMiddlewareA::new()),
                    Box::new(FakeMiddlewareB::new())
                ]
            )
        );
        match stack.handle(&"A message") {
            Ok(_) => { assert!(false); },
            Err(_) => { assert!(true); }
        }
    }
}