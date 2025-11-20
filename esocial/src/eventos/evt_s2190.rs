use serde::{Deserialize, Serialize};
use crate::tipos::{Data, Id, Matricula, NaturezaAtividade, TipoCPF, TiposIdentificadores, ValorMonetario};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvtAdmPrelim {
    pub id: Id,
    pub ide_empregador: TiposIdentificadores,
    pub info_reg_prelim: Option<InfoRegPrelim>
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRegPrelim {
    pub cpf_trab: TipoCPF,
    pub dt_nasc: Data,
    pub dt_adm: Data,
    pub matricula: Matricula,
    pub cod_categ: u32,
    pub nat_atividade: Option<NaturezaAtividade>,
    pub info_reg_ctps: Option<InfoRegCTPS>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRegCTPS {
    #[serde(rename = "CBOCargo")]
    pub cbo_cargo: Option<String>,
    pub vr_sal_fx: Option<ValorMonetario>,
    pub und_sal_fixo: Option<u8>,
    pub tp_contr: Option<u8>,
    pub dt_term: Option<Data>,
}


