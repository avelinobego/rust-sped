// Copyright 2023 Avelino Bego
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chrono::{DateTime, Local};
use std::fmt::Display;
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
