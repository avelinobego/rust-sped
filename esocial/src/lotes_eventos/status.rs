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

use elementtree::Element;

use crate::toxml::toxml::ToElement;

use super::ocorrencia::{Ocorrencia, self};

//TODO: Criar implementações
pub struct Status<'a> {
    cod_resposta: i32,
    desc_resposta: &'static str,
    ocorrencias: Vec<Ocorrencia<'a>>,
}

impl<'a> Status<'a> {
    pub fn new(
        cod_resposta: i32,
        desc_resposta: &'static str,
        ocorrencias: Vec<Ocorrencia<'a>>,
    ) -> Self {
        Status {
            cod_resposta,
            desc_resposta,
            ocorrencias,
        }
    }
}

impl<'a> ToElement for Status<'a> {
    fn to_element(&self) -> elementtree::Element {
        let mut root = Element::new("status");
        root.append_new_child("cdResposta")
            .set_text(self.cod_resposta.to_string());
        root.append_new_child("descResposta")
            .set_text(self.desc_resposta);

        let mut ocorrencias = Element::new("ocorrencias");
        self.ocorrencias.iter().for_each(|o| {
            ocorrencias.append_child(o.to_element());
        });
        root.append_child(ocorrencias);

        root
    }
}

#[test]
pub fn test_status() {
    let os = vec![
        Ocorrencia::new(1, "Recebido", 1, "<codigo>"),
        Ocorrencia::new(2, "Erro", 1, "<codigo>"),
        Ocorrencia::new(3, "500", 1, "<codigo>"),
    ];
    let st = Status::new(1, "Desc resposta", os);
    
    let testar = "<?xml version=\"1.0\" encoding=\"utf-8\"?><status><cdResposta>1</cdResposta><descResposta>Desc resposta</descResposta><ocorrencias><ocorrencia><codigo>1</codigo><descricao>Recebido</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia><ocorrencia><codigo>2</codigo><descricao>Erro</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia><ocorrencia><codigo>3</codigo><descricao>500</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia></ocorrencias></status>";

    assert_eq!(testar, st.to_element().to_string().unwrap());
}
