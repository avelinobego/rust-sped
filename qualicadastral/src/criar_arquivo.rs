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
