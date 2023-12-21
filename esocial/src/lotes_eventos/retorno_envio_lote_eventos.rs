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

//TODO: Tratar o retorno com propriedades/métodos públicos.

use crate::{
    identificadores::ide::{IdeEmpregador, IdeTransmissor},
    toxml::toxml::ToElement,
};

use super::status::Status;

//TODO: Incluir status
pub struct RetornoEnvioLoteEventos<'a> {
    ide_empregador: IdeEmpregador<'a>,
    ide_transmissor: IdeTransmissor<'a>,
    status: Status<'a>,
}

impl<'a> RetornoEnvioLoteEventos<'a> {
    pub fn new(
        ide_empregador: IdeEmpregador<'a>,
        ide_transmissor: IdeTransmissor<'a>,
        status: Status<'a>,
    ) -> Self {
        RetornoEnvioLoteEventos {
            ide_empregador,
            ide_transmissor,
            status,
        }
    }
}

impl ToElement for RetornoEnvioLoteEventos<'_> {
    fn to_element(&self) -> elementtree::Element {
        todo!()
    }
}
