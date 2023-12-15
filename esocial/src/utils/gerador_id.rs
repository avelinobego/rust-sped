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

use chrono::{DateTime, Local};

pub struct IdEvento<'a> {
    tipo_inscricao: &'a str,
    numero_inscricao: &'a str,
    date_time: DateTime<Local>,
    sequencial: u32,
}

impl<'a> IdEvento<'a> {
    pub fn new(
        tipo_inscricao: &'a str,
        numero_inscricao: &'a str,
        date_time: DateTime<Local>,
        sequencial: u32,
    ) -> Self {
        IdEvento {
            date_time,
            numero_inscricao,
            sequencial,
            tipo_inscricao,
        }
    }
}

impl IdEvento<'_> {
    pub fn to_id(&self) -> String {
        let s_data_time = self.date_time.format("%Y%m%d%H%M%S");
        let s_numero_inscricao = format!("{0:0<14}", self.numero_inscricao);
        let s_sequencial = format!("{0:0>5}", self.sequencial);
        format!(
            "ID{0}{1:0<14}{2}{3:0>5}",
            &self.tipo_inscricao, &s_numero_inscricao, s_data_time, &s_sequencial
        )
    }
}

#[test]
fn test_gerar_id() {
    use chrono::TimeZone;

    let now = Local.with_ymd_and_hms(2023, 12, 10, 18, 37, 59).unwrap();
    let id_evento = IdEvento::new("1", "0426001000138", now, 1);

    let result = id_evento.to_id();
    println!("{result}");
    assert_eq!("ID1042600100013802023121018375900001", result);
}
