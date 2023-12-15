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

use std::{
    fs::OpenOptions,
    io::Write,
};

use crate::dto::Lote;

#[allow(dead_code)]
pub fn criar_arquivo_cqc(lote: Lote, file_name: &str) {
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
