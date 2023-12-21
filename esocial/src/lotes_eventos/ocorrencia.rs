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

pub struct Ocorrencia<'a> {
    codigo: i32,
    descricao: &'a str,
    tipo: i32,
    localizacao: &'a str,
}

impl<'a> Ocorrencia<'a> {
    pub fn new(codigo: i32, descricao: &'a str, tipo: i32, localizacao: &'a str) -> Self {
        Ocorrencia {
            codigo,
            descricao,
            tipo,
            localizacao,
        }
    }
}

impl ToElement for Ocorrencia<'_> {
    fn to_element(&self) -> Element {
        let mut root = Element::new("ocorrencia");
        root.append_new_child("codigo")
            .set_text(self.codigo.to_string());
        root.append_new_child("descricao").set_text(self.descricao);
        root.append_new_child("tipo")
            .set_text(self.tipo.to_string());
        root.append_new_child("localizacao")
            .set_text(self.localizacao);
        root
    }
}

#[test]
pub fn test_ocorrencia() {
    let o = Ocorrencia::new(1, "Recebido", 1, "<codigo>");
    let el = o.to_element();
    assert_eq!("<?xml version=\"1.0\" encoding=\"utf-8\"?><ocorrencia><codigo>1</codigo><descricao>Recebido</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia>", 
    el.to_string().unwrap());
}
