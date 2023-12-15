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
mod criar_arquivo;
mod dto;

fn main() {
    println!("Qualificação cadastral");
}

#[cfg(test)]
mod tests {
    use chrono::Local;

    use crate::dto::Lote;

    //TODO: Apagar ou arrumar este teste
    #[test]
    fn test_criar() {
        for n in 1..2 {
            let mut dto = Lote::default();
            dto.cpf = Some(10000000000 + n);
            dto.nis = Some(10000000000 + n);
            dto.nome = Some(String::from("Avelino de Almeida Bego"));
            dto.nome_mae = Some(String::from("Maria Aparecida de Almeida Navas"));
            dto.dn = Some(Local::now());
            dto.uf = Some(String::from("RS"));

            println!("{}", dto);
            assert_eq!("10000000001;10000000001;Avelino de Almeida Bego;15122023;RS;Maria Aparecida de Almeida Navas",
        format!("{}", dto));
        }
    }
}
