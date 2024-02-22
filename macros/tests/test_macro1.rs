// Copyright (C) 2024 Avelino Bego
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

use macros::builder_elements;
use elementtree::Element;

#[derive(builder_elements)]
pub struct EvtMinhaEstrutura {
    pub nome: &'static str,
}

pub fn validate_evtminhaestrutura(e: &EvtMinhaEstrutura) -> Result<(), &'static str>{
    // Err("Deu zica!")
    Ok(())
}
#[test]
pub fn test_macro1() {
    let s = EvtMinhaEstrutura { nome: "Avelino" };
    let els =  s.build_elements();

    match els {
        Ok(e) => println!("{}", e.to_string().unwrap()),
        Err(err) => println!("{}", err),
    }
}
