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

use chrono::{Local, TimeZone};
use elementtree::Element;

use crate::{
    identificadores::ide::{IdeEmpregador, IdeTransmissor},
    toxml::toxml::ToElement,
};

use super::{dados_repcao_lote::DadosRecepcaoLote, status::Status};

pub struct RetornoEnvioLoteEventos<'a> {
    ide_empregador: IdeEmpregador<'a>,
    ide_transmissor: IdeTransmissor<'a>,
    status: Status<'a>,
    dados_rec_lote: DadosRecepcaoLote<'a>,
}

impl<'a> RetornoEnvioLoteEventos<'a> {
    pub fn new(
        ide_empregador: IdeEmpregador<'a>,
        ide_transmissor: IdeTransmissor<'a>,
        status: Status<'a>,
        dados_rec_lote: DadosRecepcaoLote<'a>,
    ) -> Self {
        RetornoEnvioLoteEventos {
            ide_empregador,
            ide_transmissor,
            status,
            dados_rec_lote,
        }
    }
}

impl ToElement for RetornoEnvioLoteEventos<'_> {
    fn to_element(&self) -> elementtree::Element {
        let mut root = Element::new("retornoEnvioLotes");
        root.append_child(self.ide_empregador.to_element());
        root.append_child(self.ide_transmissor.to_element());
        root.append_child(self.status.to_element());
        root.append_child(self.dados_rec_lote.to_element());

        root
    }
}

#[test]
pub fn test_retorno_envio_lotes() {
    use super::{ocorrencia::Ocorrencia, status::Status};

    let ide_empregador = IdeEmpregador::new(1, "22");
    let ide_transmissor = IdeTransmissor::new(2, "33");
    let dados_recepecao_lote = DadosRecepcaoLote::new(
        Local.with_ymd_and_hms(2023, 12, 20, 0, 0, 0).unwrap(),
        "versao",
        "protocolo",
    );
    let os = vec![
        Ocorrencia::new(1, "Recebido", 1, "<codigo>"),
        Ocorrencia::new(2, "Erro", 1, "<codigo>"),
        Ocorrencia::new(3, "500", 1, "<codigo>"),
    ];
    let st = Status::new(1, "Desc resposta", os);

    let ret_env =
        RetornoEnvioLoteEventos::new(ide_empregador, ide_transmissor, st, dados_recepecao_lote);

    let dados = "<?xml version=\"1.0\" encoding=\"utf-8\"?><retornoEnvioLotes><ideEmpregador><tpInsc>1</tpInsc><nrInsc>22</nrInsc></ideEmpregador><ideTransmissor><tpInsc>2</tpInsc><nrInsc>33</nrInsc></ideTransmissor><status><cdResposta>1</cdResposta><descResposta>Desc resposta</descResposta><ocorrencias><ocorrencia><codigo>1</codigo><descricao>Recebido</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia><ocorrencia><codigo>2</codigo><descricao>Erro</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia><ocorrencia><codigo>3</codigo><descricao>500</descricao><tipo>1</tipo><localizacao>&lt;codigo></localizacao></ocorrencia></ocorrencias></status><dadosRecepcaoLote><dhRecepcao>20231220000000</dhRecepcao><versaoAplicativoRecepcao>versao</versaoAplicativoRecepcao><protocoloEnvio>protocolo</protocoloEnvio></dadosRecepcaoLote></retornoEnvioLotes>";

    assert_eq!(dados, ret_env.to_element().to_string().unwrap());

}
