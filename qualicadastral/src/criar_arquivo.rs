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

use std::{fs::OpenOptions, io::Write};

use simplelog::{SimpleLogger, LevelFilter, Config};

use crate::dto::Lote;

#[allow(dead_code)]
pub fn criar_arquivo_cqc(lote: Lote, file_name: &str) {
    log_panics::init();
    let mut f = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap();
    if let Err(e) = writeln!(f, "{}", lote) {
        eprintln!("Falha! {}", e);
    }
}

#[test]
pub fn test_criar() {
    use chrono::{Local, TimeZone};
    use crate::dto::Lote;

    log_panics::init();
    let _ = SimpleLogger::init(LevelFilter::Error, Config::default());

    for n in 1..2 {
        let mut dto = Lote::default();
        dto.cpf = Some(10000000000 + n);
        dto.nis = Some(10000000000 + n);
        dto.nome = Some(String::from("Avelino de Almeida Bego"));
        dto.nome_mae = Some(String::from("Maria Aparecida de Almeida Navas"));
        dto.dn = Some(Local.with_ymd_and_hms(2023, 12, 15, 0, 0, 0).unwrap());
        dto.uf = Some(String::from("RS"));

        println!("{}", dto);
        assert_eq!("10000000001;10000000001;Avelino de Almeida Bego;15122023;RS;Maria Aparecida de Almeida Navas",
    format!("{}", dto));
    }
}
