use crate::eventos::{
    eventos::Eventos,
    s1000::{self, S1000},
};

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
mod esocial;
mod eventos;
mod identificadores;
mod lotes_eventos;
mod toxml;
mod utils;

fn main() {
    println!("e-Social!");
}

#[test]
#[allow(dead_code)]
fn test_criar_xml() {
    use crate::esocial::esocial::ESocial;
    use crate::identificadores::ide::IdeEmpregador;
    use crate::identificadores::ide::IdeTransmissor;
    use crate::lotes_eventos::envio_lotes_eventos::EnvioLotesEventos;
    use crate::toxml::toxml::ToElement;

    let ide_e = IdeEmpregador::new(1, "Teste Empregador");
    let ide_t = IdeTransmissor::new(10, "Teste Transmissor");

    let mut envio_lotes = EnvioLotesEventos::<S1000>::new(1, ide_e, ide_t);

    for i in 1..51 {
        envio_lotes.add_evento(Eventos::new(i, S1000::new()));
    }

    let es = ESocial::new("v1_1_1", envio_lotes);
    println!("{}", es.to_element().to_string().unwrap());
}
