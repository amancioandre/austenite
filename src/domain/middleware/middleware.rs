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

use std::any::{Any, type_name};
use std::fmt::{Debug, Display};

pub trait Middleware {
    fn message(&mut self) -> Result<String, String>;

    fn next(&mut self) -> &'static dyn Any;

    fn name(&self) -> &'static str {
        type_name::<Self>()
    }
}

impl Display for dyn Middleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        write!(f, "{}", name)
    }
}

impl Debug for dyn Middleware {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name();
        write!(f, "{}", name)
    }
}