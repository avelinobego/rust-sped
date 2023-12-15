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

mod criar_arquivo;
mod dto;

#[cfg(test)]
mod tests {
    use chrono::Local;

    use crate::{dto::Lote, criar_arquivo::criar_arquivo_cqc};

    //TODO: Apagar ou arrumar este teste
    #[test]
    fn test_criar() {
        for n in 1..101 {
            let mut dto = Lote::default();
            dto.cpf = Some(10000000000 + n);
            dto.nis = Some(10000000000 + n);
            dto.nome = Some(String::from("Avelino de Almeida Bego"));
            dto.nome_mae = Some(String::from("Maria Aparecida de Almeida Navas"));
            dto.dn = Some(Local::now());
            dto.uf = Some(String::from("RS"));

            criar_arquivo_cqc(dto, "saida.txt");
        }
    }
}
