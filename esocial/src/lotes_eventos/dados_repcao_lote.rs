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

use chrono::{DateTime, Local, TimeZone};
use elementtree::Element;

use crate::toxml::toxml::ToElement;

pub struct DadosRecepcaoLote<'a> {
    dh_recepcao: DateTime<Local>,
    versao_aplicativo_recepcao: &'a str,
    protocolo_envio: &'a str,
}

impl<'a> DadosRecepcaoLote<'a> {
    pub fn new(
        dh_recepcao: DateTime<Local>,
        versao_aplicativo_recepcao: &'a str,
        protocolo_envio: &'a str,
    ) -> Self {
        DadosRecepcaoLote {
            dh_recepcao,
            versao_aplicativo_recepcao,
            protocolo_envio,
        }
    }
}

impl ToElement for DadosRecepcaoLote<'_> {
    fn to_element(&self) -> Element {
        let mut root = Element::new("dadosRecepcaoLote");
        root.append_new_child("dhRecepcao")
            .set_text(self.dh_recepcao.format("%Y%m%d%H%M%S").to_string());
        root.append_new_child("versaoAplicativoRecepcao")
            .set_text(self.versao_aplicativo_recepcao);
        root.append_new_child("protocoloEnvio")
            .set_text(self.protocolo_envio);

        root
    }
}

#[test]
pub fn test_dados_recepcao_lote() {
    let dh_recepcao = Local.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap();
    let versao_aplicativo_recepcao = "1";
    let protocolo_envio = "000001";
    let rcplt = DadosRecepcaoLote::new(dh_recepcao, versao_aplicativo_recepcao, protocolo_envio);

    let dados = "<?xml version=\"1.0\" encoding=\"utf-8\"?><dadosRecepcaoLote><dhRecepcao>20231220000000</dhRecepcao><versaoAplicativoRecepcao>1</versaoAplicativoRecepcao><protocoloEnvio>000001</protocoloEnvio></dadosRecepcaoLote>";
}
