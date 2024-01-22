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

use crate::{
    identificadores::ide::IdeEmpregador, toxml::toxml::ToElement, constants::tipos::TipoEventos,
};


pub struct Eventos<'a, E: ToElement> {
    name: String,
    id: String,
    // id_evento: IdEventoEmpregador,
    id_empregador: IdeEmpregador<'a>,
    evento: E,
}

impl<'a, E: ToElement> Eventos<'a, E> {
    pub fn new(tipo: TipoEventos, id: String, id_empregador: IdeEmpregador<'a>, evento: E) -> Self {
        Self {
            name: tipo.to_name(),
            id,
            id_empregador,
            evento,
        }
    }
}

impl<'a, E: ToElement> ToElement for Eventos<'a, E> {
    fn to_element(&self) -> elementtree::Element {
        let mut root = Element::new("evento");
        root.append_child(self.evento.to_element());
        root
    }

    fn validate(&self) -> Result<(), &'static str> {
        todo!()
    }
}

