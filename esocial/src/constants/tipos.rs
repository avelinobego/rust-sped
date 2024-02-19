// Copyright (C) 2024 Avelino Bego
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
#[derive(Debug)]
pub enum TipoEventos {
    EvtAdmPrelim,
    EvtAdmissao,
    EvtAfastTemp,
    EvtAltCadastral,
    EvtAltContratual,
    EvtAnotJud,
    EvtBaixa,
    EvtBasesFGTS,
    EvtBasesTrab,
    EvtBenPrRP,
    EvtCAT,
    EvtCS,
    EvtCdBenAlt,
    EvtCdBenIn,
    EvtCdBenTerm,
    EvtCdBenefAlt,
    EvtCdBenefIn,
    EvtCessao,
    EvtComProd,
    EvtContProc,
    EvtContratAvNP,
    EvtDeslig,
    EvtExcProcTrab,
    EvtExclusao,
    EvtExpRisco,
    EvtFGTS,
    EvtFGTSProcTrab,
    EvtFechaEvPer,
    EvtRemun,
    EvtPgtos,
    EvtInfoComplPer,
    EvtInfoEmpregador,
    EvtIrrf,
    EvtIrrfBenef,
    EvtMonit,
    EvtProcTrab,
    EvtReabreEvPer,
    EvtReativBen,
    EvtReintegr,
    EvtRmnRPPS,
    EvtTSVAltContr,
    EvtTSVInicio,
    EvtTSVTermino,
    EvtTabEstab,
    EvtTabLotacao,
    EvtTabProcesso,
    EvtTabRubrica,
    EvtTribProcTrab,
}

impl TipoEventos {
    pub fn nome_evento(&self) -> String {
        let temp = format!("{:?}", self);
        temp.replacen("E", "e", 1)
    }

    pub fn nome_info(&self) -> String {
        let temp = format!("{:?}", self);
        temp.replacen("Evt", "info", 3)
    }
}

#[test]
fn test_to_string() {
    let t = TipoEventos::EvtAdmissao;
    assert_eq!("evtAdmissao", t.nome_evento());
    assert_eq!("infoAdmissao", t.nome_info());
}
