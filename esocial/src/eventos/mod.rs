pub mod tests_evt_admissao;
pub mod tests_evt_adm_prelim;
pub mod tests_s1000;
pub mod evt_s2200;
pub mod evt_s2190;
pub mod evt_s1000;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::eventos::evt_s2200::EvtAdimissao;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "eSocial")]
pub struct ESocial {
    #[serde(rename = "@xmlns")]
    pub name_space: String,
    #[serde(rename = "$value")]
    pub evento: EnumEventos,
}

impl From<(EnumEventos, String)> for ESocial {
    fn from(value: (EnumEventos, String)) -> Self {
        ESocial {
            name_space: value.1,
            evento: value.0,
        }
    }
}

impl From<(EnumEventos, &str)> for ESocial {
    fn from(value: (EnumEventos, &str)) -> Self {
        ESocial {
            name_space: value.1.into(),
            evento: value.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum EnumEventos {
    #[serde(rename = "evtAdmissao")]
    Adimissao(EvtAdimissao),
}

impl From<EvtAdimissao> for EnumEventos {
    fn from(value: EvtAdimissao) -> Self {
        EnumEventos::Adimissao(value)
    }
}

