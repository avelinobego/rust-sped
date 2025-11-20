use serde::{Deserialize, Serialize};

use crate::tipos::{Id, IdentEvento, TipoInfoEmpregador};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvtInfoEmpregador {
    #[serde(rename = "@Id")]
    pub id: Id,
    pub ide_evento: IdentEvento,
    pub info_empregador: TipoInfoEmpregador,
}
