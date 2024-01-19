// Copyright (C) 2023 Avelino Bego
//
// This file is part of SPED.
//
// SPED is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// SPED is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with SPED.  If not, see <http://www.gnu.org/licenses/>.

use elementtree::Element;

use crate::toxml::toxml::ToElement;

pub struct S1000 {}

impl S1000 {
    pub fn new() -> Self {
        S1000 {}
    }
}

impl ToElement for S1000 {
    fn to_element(&self) -> elementtree::Element {
        let root = Element::new("s1000");
        root
    }
}
