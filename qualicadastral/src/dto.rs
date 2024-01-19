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

use std::fmt::Display;
use chrono::{DateTime, Local};
use utils::trunc;

#[derive(Default)]
pub struct Lote {
    pub cpf: Option<i64>,
    pub nis: Option<i64>,
    pub nome: Option<String>,
    pub dn: Option<DateTime<Local>>,
    pub uf: Option<String>,
    pub municipio: Option<String>,
    pub nome_mae: Option<String>,
}

impl Display for Lote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fields: Vec<Option<String>> = vec![];

        fields.push(trunc(&self.cpf, 11));
        fields.push(trunc(&self.nis, 11));
        fields.push(trunc(&self.nome, 70));

        if let Some(dt) = self.dn {
            let temp = Some(dt.format("%d%m%Y").to_string());
            fields.push(temp);
        }

        fields.push(trunc(&self.uf, 2));
        fields.push(trunc(&self.nome_mae, 60));

        let clean: Vec<String> = fields
            .iter()
            .filter(|e| e.is_some())
            .map(|e| e.to_owned().unwrap())
            .collect();

        write!(f, "{}", clean.join(";"))
    }
}
