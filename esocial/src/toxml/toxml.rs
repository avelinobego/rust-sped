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

pub trait ToElement {
    fn to_element(&self) -> Element;

    fn validate(&self) -> Result<(), &'static str> {
        Err("validação não implementada!")
    }
}

#[test]
pub fn test_validacao() {
    use structured_logger::Builder;

    // Initialize the logger.
    Builder::new().init();
    log::error!("oi pessoal");
}
