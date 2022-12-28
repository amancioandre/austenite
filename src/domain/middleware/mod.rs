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

pub use middleware::{Middleware};
pub use stack::{Stack};

pub mod middleware;
pub mod stack;