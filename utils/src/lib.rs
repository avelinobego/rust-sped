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

/// Trunca um valor pelo **tamanho**, caso contrário retorna o valor **intacto**.
///
/// # Argumentos
///
/// * `value` - Uma referência à uma String.
/// * `size` - O tamanho limte
///
/// # Retorno
///
/// * A String modificada

pub fn trunc<T: ToString>(value: &Option<T>, size: usize) -> Option<String> {
    if let Some(s) = value {
        let mut temp = s.to_string().clone();
        temp.truncate(size);
        Some(temp)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_chars() {
        let s = String::from("Avelino");
        assert_eq!("Ave", trunc(&Some(s), 3).unwrap());
    }
}
