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

use std::any::Any;

use super::{Middleware};

pub struct Stack {
    middlewares: Vec<Box<dyn Middleware>>
}

impl Stack {
    pub fn new(middlewares: Option<Vec<Box<dyn Middleware>>>) -> Self {
        let middlewares: Vec<Box<dyn Middleware>> = middlewares.unwrap_or(Vec::new());
        Self {
            middlewares
        }
    }

    fn exists(&self, middleware: &Box<dyn Middleware>) -> Option<usize> {
        if let Some(idx) = self.middlewares.iter().position(|x| x.type_id() == middleware.type_id()) {
            Some(idx)
        } else {
            None
        }
    }

    pub fn attach(&mut self, middleware: Box<dyn Middleware>) {
        match self.exists(&middleware) {
            Some(idx) => println!("Middleware already exists"),
            None => self.middlewares.push(middleware)
        }
    }

    pub fn dettach(&mut self) {
        unimplemented!()
    }

    pub fn substitute(&mut self, middleware: Box<dyn Middleware>, other: Box<dyn Middleware>) {
        unimplemented!()
    }

    pub fn run(&self) -> Result<String, String> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Debug)]
    struct FakeMiddlewareA {}

    impl FakeMiddlewareA {
        fn new() -> Self {
            Self {}
        }
    }

    impl Middleware for FakeMiddlewareA {
        fn next(self: &mut FakeMiddlewareA) -> &'static dyn Any {
            unimplemented!()
        }

        fn req(&mut self) -> Result<String, String> {
            unimplemented!()
        }

        fn res(&mut self) -> Result<String, String> {
            unimplemented!()
        }
    }

    #[derive(PartialEq, Debug)]
    struct FakeMiddlewareB {}

    impl FakeMiddlewareB {
        fn new() -> Self {
            Self {}
        }
    }

    impl Middleware for FakeMiddlewareB {
        fn next(self: &mut FakeMiddlewareB) -> &'static dyn Any {
            unimplemented!()
        }

        fn req(&mut self) -> Result<String, String> {
            unimplemented!()
        }

        fn res(&mut self) -> Result<String, String> {
            unimplemented!()
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
        dbg!("{}", middleware_a.type_id());

        let middleware_b = Box::new(FakeMiddlewareA::new());
        dbg!("{} {:?}", middleware_b.type_id(), &middleware_b);
        stack.attach(middleware_a);
        assert_eq!(stack.middlewares.len(), 1);
        stack.attach(middleware_b);
        assert_eq!(stack.middlewares.len(), 1);
    }
}