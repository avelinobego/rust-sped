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

pub fn validar(valor: String) -> bool {
    let mut somente_digitos = valor.chars().filter(|c| c.is_digit(10)).collect::<String>();
    let digito_verificador = String::from(&somente_digitos[somente_digitos.len() - 2..]);

    let mut primeiro_digito = calcular_digito(&somente_digitos);
    somente_digitos.push_str(&primeiro_digito);
    let segundo_digito = calcular_digito(&somente_digitos);

    primeiro_digito.push_str(&segundo_digito);

    digito_verificador.eq(&primeiro_digito)
}

fn calcular_digito(valor: &String) -> String {
    let verificar = &valor[..valor.len() - 2];

    let mut peso = Peso::new(11);
    // Primeiro dígito
    let soma_pesos = verificar
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap() * peso.next().unwrap())
        .sum::<u32>();

    let resto = soma_pesos % 11u32;
    if resto < 2 {
        0.to_string()
    } else {
        (11 - resto).to_string()
    }
}

pub struct Peso {
    limite: u32,
    atual: u32,
}

impl Iterator for Peso {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.atual;
        if self.atual < self.limite {
            self.atual = self.atual + 1;
        } else {
            self.atual = 2;
        }
        Some(result)
    }
}

impl Peso {
    pub fn new(limite: u32) -> Self {
        Peso { limite, atual: 2 }
    }
}

#[test]
#[allow(dead_code)]
fn test_validar() {
    let t = String::from("916.059.640-69");
    assert_eq!(validar(t), true);
}
