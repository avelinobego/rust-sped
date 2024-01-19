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

use crate::eventos::eventos::Eventos;
use crate::toxml::toxml::ToElement;

use crate::identificadores::ide::IdeEmpregador;
use crate::identificadores::ide::IdeTransmissor;

pub struct EnvioLotesEventos<'a, E: ToElement> {
    grupo: i32,
    ide_empregador: IdeEmpregador<'a>,
    ide_transmissor: IdeTransmissor<'a>,
    eventos: Vec<Eventos<E>>,
}

impl<'a, E: ToElement> EnvioLotesEventos<'a, E> {
    pub fn new(
        grupo: i32,
        ide_empregador: IdeEmpregador<'a>,
        ide_transmissor: IdeTransmissor<'a>,
    ) -> Self {
        EnvioLotesEventos {
            grupo: grupo,
            ide_empregador: ide_empregador,
            ide_transmissor: ide_transmissor,
            eventos: Vec::default(),
        }
    }

    pub fn add_evento(&mut self, evento: Eventos<E>) {
        self.eventos.push(evento);
    }
}

impl<E: ToElement> ToElement for EnvioLotesEventos<'_, E> {
    fn to_element(&self) -> Element {
        let mut root = Element::new("envioLotesEventos");
        root.set_attr("grupo", self.grupo.to_string());
        root.append_child(self.ide_empregador.to_element());
        root.append_child(self.ide_transmissor.to_element());

        let evnts = root.append_new_child("eventos");

        self.eventos
            .iter()
            .map(|e| {
                let mut result = Element::new("evento");
                result.set_attr("Id", e.id().to_string());
                result
            })
            .for_each(|e| {
                evnts.append_child(e);
            });

        root
    }
}
// --------------------------------------------------
