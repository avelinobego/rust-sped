use chrono::NaiveDate;

use serde::{Deserialize, Serialize, ser::SerializeStruct};
use serde_with::skip_serializing_none;

use crate::tipos::{
    CodTreinoCapacitacao, CodigoEsocial, Contato, Data, DataMes, Decimal, Descricao, DescricaoSalario, EnumEstadoCivil, EnumRacaCor, EnumSexo, EnumUf, GrauInstrucao, HipoteseContratacaoTemporario, Id, IdentEvento, IndicativoAdmissao, LocalTrabalhoGeral, Matricula, NaturezaAtividade, Nome, Nome100, Numero, Observacao, RegimeJornada, RegimePrevidenciario, RegimeTrabalhista, Texto255, TipoAdmissao, TipoCNPJ, TipoCPF, TipoDependente, TipoPlanoRP, TipoSimNao, TiposIdentificadores, ValorMonetario
};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvtAdimissao {
    #[serde(rename = "@Id")]
    pub id: Id,
    pub ide_evento: IdentEvento,
    pub inscricao: TiposIdentificadores,
    pub trabalhador: TipoTrabalhador,
    pub vinculo: Option<Vinculo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TIdeTrabAdmissao {
    pub ind_retif: u8,
    pub nr_recibo: Option<String>,
    pub tp_amb: u8,
    pub proc_emi: u8,
    pub ver_proc: u8,
}

//----------------------------------------------------------------------------------------

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipoTrabalhador {
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: TipoCPF,
    pub nm_trab: Nome,
    pub sexo: EnumSexo,
    pub raca_cor: EnumRacaCor,
    pub est_civ: Option<EnumEstadoCivil>,
    pub grau_instr: GrauInstrucao,
    pub nm_soc: Option<Nome>,
    pub nascimento: Nascimento,
    pub endereco: Option<Endereco>,
    pub trab_imig: Option<TipoTrabalhadorImig>,
    pub info_def: Option<InfoDefComposer>,
    pub dependente: Option<Vec<Dependente>>,
    pub contato: Option<Contato>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dependente {
    pub tp_dep: Option<TipoDependente>,
    pub nm_dep: Nome,
    pub dt_nascto: Data,
    pub cpf_dep: Option<TipoCPF>,
    pub sexo_dep: Option<EnumSexo>,
    #[serde(rename = "depIRRF")]
    pub dep_irrf: TipoSimNao,
    #[serde(rename = "depSF")]
    pub dep_sf: TipoSimNao,
    pub inc_trab: Option<TipoSimNao>,
    pub descr_dep: Option<Descricao>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Endereco {
    #[serde(rename = "$value")]
    pub end: EnumEndereco,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipoTrabalhadorImig {
    #[serde(rename = "$value")]
    pub trab_imig: EnumTrabImig,
}

impl From<TempoResidencia> for TipoTrabalhadorImig {
    fn from(value: TempoResidencia) -> Self {
        TipoTrabalhadorImig {
            trab_imig: EnumTrabImig::TmpResid(value),
        }
    }
}

impl From<CondicaoImigracao> for TipoTrabalhadorImig {
    fn from(value: CondicaoImigracao) -> Self {
        TipoTrabalhadorImig {
            trab_imig: EnumTrabImig::TsCondIng(value),
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnumTrabImig {
    TmpResid(TempoResidencia),
    TsCondIng(CondicaoImigracao),
}

#[derive(Debug)]
pub enum CondicaoImigracao {
    Refugiado,
    SolicitanteRefugio,
    ReuniaoFamiliar,
    BeneficiadoMercosul,
    DepAgenteDiplomatico,
    TratadoAmizade,
    Outro,
}

impl Serialize for CondicaoImigracao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            CondicaoImigracao::Refugiado => "1",
            CondicaoImigracao::SolicitanteRefugio => "2",
            CondicaoImigracao::ReuniaoFamiliar => "3",
            CondicaoImigracao::BeneficiadoMercosul => "4",
            CondicaoImigracao::DepAgenteDiplomatico => "5",
            CondicaoImigracao::TratadoAmizade => "6",
            CondicaoImigracao::Outro => "7",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for CondicaoImigracao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(CondicaoImigracao::Refugiado),
            "2" => Ok(CondicaoImigracao::SolicitanteRefugio),
            "3" => Ok(CondicaoImigracao::ReuniaoFamiliar),
            "4" => Ok(CondicaoImigracao::BeneficiadoMercosul),
            "5" => Ok(CondicaoImigracao::DepAgenteDiplomatico),
            "6" => Ok(CondicaoImigracao::TratadoAmizade),
            "7" => Ok(CondicaoImigracao::Outro),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para CondicaoImigracao",
            )),
        }
    }
}

#[derive(Debug)]
pub enum TempoResidencia {
    Indeterminado,
    Determinado,
}

impl Serialize for TempoResidencia {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TempoResidencia::Indeterminado => "1",
            TempoResidencia::Determinado => "2",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for TempoResidencia {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: String = Deserialize::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(TempoResidencia::Indeterminado),
            "2" => Ok(TempoResidencia::Determinado),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para TempoResidencia",
            )),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum EnumInfoDef {
    Fisica,
    Visual,
    Auditiva,
    Mental,
    Intelectual,
    Reabilitado,
    Cota,
    Observacao(String),
}

#[derive(Debug, Clone, Default)]
pub struct InfoDefComposer {
    info_def: Vec<EnumInfoDef>,
    observacao: Option<String>,
}

impl InfoDefComposer {
    pub fn fisica(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Fisica);
        self.clone()
    }

    pub fn visual(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Visual);
        self.clone()
    }
    pub fn auditiva(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Auditiva);
        self.clone()
    }
    pub fn mental(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Mental);
        self.clone()
    }
    pub fn intelectual(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Intelectual);
        self.clone()
    }
    pub fn reabilitado(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Reabilitado);
        self.clone()
    }
    pub fn cota(&mut self) -> Self {
        self.info_def.push(EnumInfoDef::Cota);
        self.clone()
    }
    pub fn observacao(&mut self, obs: String) -> Self {
        self.observacao = Some(obs);
        self.clone()
    }
}

impl Serialize for InfoDefComposer {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let count = self.info_def.len() + if self.observacao.is_some() { 1 } else { 0 };

        let mut stru = serializer.serialize_struct("InfoDef", count)?;
        self.info_def.iter().for_each(|info| {
            let k = match info {
                EnumInfoDef::Fisica => "defFisica",
                EnumInfoDef::Visual => "defVisual",
                EnumInfoDef::Auditiva => "defAuditiva",
                EnumInfoDef::Mental => "defMental",
                EnumInfoDef::Intelectual => "defIntelectual",
                EnumInfoDef::Reabilitado => "reabReadap",
                EnumInfoDef::Cota => "infoCota",
                _ => unreachable!(),
            };
            stru.serialize_field(k, "S").unwrap();
        });

        if let Some(obs) = &self.observacao {
            stru.serialize_field("observacao", obs).unwrap();
        }
        stru.end()
    }
}

impl<'de> Deserialize<'de> for InfoDefComposer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut info_def = InfoDefComposer::default();
        let map: std::collections::HashMap<String, String> =
            Deserialize::deserialize(deserializer)?;

        for (key, value) in map {
            match key.as_str() {
                "defFisica" => info_def.info_def.push(EnumInfoDef::Fisica),
                "defVisual" => info_def.info_def.push(EnumInfoDef::Visual),
                "defAuditiva" => info_def.info_def.push(EnumInfoDef::Auditiva),
                "defMental" => info_def.info_def.push(EnumInfoDef::Mental),
                "defIntelectual" => info_def.info_def.push(EnumInfoDef::Intelectual),
                "reabReadap" => info_def.info_def.push(EnumInfoDef::Reabilitado),
                "infoCota" => info_def.info_def.push(EnumInfoDef::Cota),
                "observacao" => info_def.observacao = Some(value),
                _ => {}
            }
        }
        Ok(info_def)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndBrasil {
    pub tp_lograd: Option<String>,
    pub dsc_lograd: String,
    pub nr_lograd: String,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub cep: String,
    pub cod_munic: u32,
    pub uf: EnumUf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndExterior {
    pub pais_resid: String,
    pub dsc_lograd: String,
    pub nr_lograd: String,
    pub complemento: Option<String>,
    pub bairro: Option<String>,
    pub nm_cid: String,
    pub cod_postal: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnumEndereco {
    Brasil(EndBrasil),
    Exterior(EndExterior),
}

impl From<EndBrasil> for Endereco {
    fn from(value: EndBrasil) -> Self {
        Endereco {
            end: EnumEndereco::Brasil(value),
        }
    }
}

impl From<&EndBrasil> for Endereco {
    fn from(value: &EndBrasil) -> Self {
        Endereco {
            end: EnumEndereco::Brasil(value.clone()),
        }
    }
}

impl From<EndExterior> for Endereco {
    fn from(value: EndExterior) -> Self {
        Endereco {
            end: EnumEndereco::Exterior(value),
        }
    }
}

impl From<&EndExterior> for Endereco {
    fn from(value: &EndExterior) -> Self {
        Endereco {
            end: EnumEndereco::Exterior(value.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nascimento {
    pub dt_nascto: Data,
    pub pais_nascto: i32,
    pub pais_nac: i32,
}

// infoRegimeTrab
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoRegimeTrab {
    pub info_celetista: Option<InfoCeletista>,
    pub info_estatutario: Option<InfoEstatutario>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoCeletista {
    pub dt_adm: NaiveDate,
    pub tp_admissao: TipoAdmissao,
    pub ind_admissao: IndicativoAdmissao,
    pub nr_proc_trab: Option<String>,
    pub tp_reg_jor: RegimeJornada,
    pub nat_atividade: NaturezaAtividade,
    pub dt_base: Option<DataMes>,
    pub cnpj_sind_categ_prof: TipoCNPJ,
    pub mat_anot_jud: Option<CodigoEsocial>,
    pub fgts: Option<Fgts>,
    pub trab_temporario: Option<TrabTemporario>,
    pub aprend: Option<TAprend>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fgts {
    pub dt_opc_fgts: Data,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrabTemporario {
    pub hip_leg: HipoteseContratacaoTemporario,
    pub just_contr: String,
    pub ide_estab_vinc: TiposIdentificadores,
    pub ide_trab_substituido: Option<Vec<IdeTrabSubstituido>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdeTrabSubstituido {
    pub cpf_trab_subst: TipoCPF,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TAprend {
    pub ind_aprend: u128,
    pub identificador: TiposIdentificadores,
    pub cnpj_ent_qual: TipoCNPJ,
    pub cnpj_prat: TipoCNPJ,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoEstatutario {
    pub tp_prov: u128,
    pub dt_exercicio: Data,
    pub tp_plan_rp: Option<TipoPlanoRP>,
    pub ind_teto_rgps: Option<TipoSimNao>,
    pub ind_abono_perm: Option<TipoSimNao>,
    pub dt_ini_abono: Option<Data>,
}

// infoContrato
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoContrato {
    pub nm_cargo: Option<Nome100>,
    pub cbo_cargo: Option<Numero<6, 6>>,
    pub dt_ingr_cargo: Option<Data>,
    pub nm_funcao: Option<Nome100>,
    pub cbo_funcao: Option<Numero<6, 6>>,
    pub acum_cargo: Option<TipoSimNao>,
    pub cod_categ: Numero<3, 3>,
    pub remuneracao: Option<Remuneracao>,
    pub duracao: Option<Duracao>,
    pub local_trabalho: Option<LocalTrabalho>,
    pub hor_contratual: Option<Decimal<1, 4, 2>>,
    pub alvara_judicial: Option<String>,
    pub observacoes: Option<Vec<ObservacaoContrato>>,
    pub trei_cap: Option<Vec<CodTreinoCapacitacao>>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Remuneracao {
    pub vr_sal_fx: ValorMonetario,
    pub und_sal_fixo: usize,
    pub dsc_sal_var: Option<DescricaoSalario>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Duracao {
    pub tp_contr: u128,
    pub dt_term: Option<Data>,
    pub clau_assec: Option<TipoSimNao>,
    pub obj_det: Option<Texto255>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LocalTrabalho {
    pub local_trab_geral: Option<LocalTrabalhoGeral>,
    pub local_temp_dom: Option<EndBrasil>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ObservacaoContrato {
    pub observacao: Texto255,
}

// sucessaoVinc
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SucessaoVinc {
    pub identificador: TiposIdentificadores,
    pub matric_ant: Option<Matricula>,
    pub dt_transf: Data,
    pub observacao: Option<Observacao>,
}

// transfDom
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransfDom {
    pub cpf_substituido: TipoCPF,
    pub matric_ant: Option<CodigoEsocial>,
    pub dt_transf: Data,
}

// mudancaCPF
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MudancaCpf {
    pub cpf_ant: TipoCPF,
    pub matric_ant: Matricula,
    pub dt_alt_cpf: Data,
    pub observacao: Option<Observacao>,
}

// afastamento
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Afastamento {
    pub dt_ini_afast: Data,
    pub cod_mot_afast: u32,
}

// desligamento
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Desligamento {
    pub dt_deslig: Data,
}

impl From<Data> for Desligamento {
    fn from(dt_deslig: Data) -> Self {
        Desligamento { dt_deslig }
    }
}
// cessao
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cessao {
    pub dt_ini_cessao: Data,
}

impl From<Data> for Cessao {
    fn from(dt_ini_cessao: Data) -> Self {
        Cessao { dt_ini_cessao }
    }
}

// A struct principal "Vinculo"
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vinculo {
    pub matricula: Matricula,
    pub tp_reg_trab: RegimeTrabalhista,
    pub tp_reg_prev: RegimePrevidenciario,
    pub cad_ini: TipoSimNao,
    pub info_regime_trab: InfoRegimeTrab,
    pub info_contrato: InfoContrato,
    pub sucessao_vinc: Option<SucessaoVinc>,
    pub transf_dom: Option<TransfDom>,
    pub mudanca_cpf: Option<MudancaCpf>,
    pub afastamento: Option<Afastamento>,
    pub desligamento: Option<Desligamento>,
    pub cessao: Option<Cessao>,
}
