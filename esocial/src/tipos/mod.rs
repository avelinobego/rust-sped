mod tests;

use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use chrono::{DateTime, Local, NaiveDate, NaiveDateTime};
use pad::{Alignment, PadStr};
use serde::{Deserialize, Serialize, ser::SerializeMap};

pub trait Validate {
    fn validate(&self) -> Result<(), String>;
}

//----------------------------------------------------------------------------------------

pub struct Varchar(pub String, pub usize, pub usize);

impl From<Varchar> for String {
    fn from(value: Varchar) -> Self {
        varchar_to_string(&value)
    }
}

impl From<&Varchar> for String {
    fn from(value: &Varchar) -> Self {
        varchar_to_string(value)
    }
}

fn varchar_to_string(value: &Varchar) -> String {
    let result: String = value.0.chars().take(value.2).collect();
    if result.len() < value.1 {
        return "tamanho invalido".into();
    }
    result
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TiposIdentificadores {
    CNPJ(String),
    CPF(String),
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TipoCNPJ {
    CNPJ(String),
}

impl TipoCNPJ {
    pub fn sem_mascara(&self) -> String {
        match self {
            TipoCNPJ::CNPJ(cnpj) => TiposIdentificadores::CNPJ(cnpj.clone()).sem_mascara(),
        }
    }

    pub fn tipo_inscricao(&self) -> u32 {
        match self {
            TipoCNPJ::CNPJ(_) => 1,
        }
    }
}

impl FromStr for TipoCNPJ {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 14 {
            Ok(TipoCNPJ::CNPJ(s.into()))
        } else {
            Err("Tamanho inválido".into())
        }
    }
}

impl From<String> for TipoCNPJ {
    fn from(value: String) -> Self {
        TipoCNPJ::CNPJ(value)
    }
}

impl From<&str> for TipoCNPJ {
    fn from(value: &str) -> Self {
        TipoCNPJ::CNPJ(value.into())
    }
}

impl From<TipoCNPJ> for u32 {
    fn from(value: TipoCNPJ) -> Self {
        match value {
            TipoCNPJ::CNPJ(_) => 1,
        }
    }
}

impl Validate for TipoCNPJ {
    fn validate(&self) -> Result<(), String> {
        match self {
            TipoCNPJ::CNPJ(cnpj) => validate_cnpj(cnpj.clone()),
        }
    }
}

impl Serialize for TipoCNPJ {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TipoCNPJ::CNPJ(cnpj) => cnpj.clone(),
        };
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for TipoCNPJ {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let result = TipoCNPJ::CNPJ(value);
        result.validate().map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TipoCPF {
    CPF(String),
}

impl TipoCPF {
    pub fn sem_mascara(&self) -> String {
        match self {
            TipoCPF::CPF(cpf) => TiposIdentificadores::CPF(cpf.clone()).sem_mascara(),
        }
    }

    pub fn tipo_inscricao(&self) -> u32 {
        match self {
            TipoCPF::CPF(_) => 2,
        }
    }
}

impl FromStr for TipoCPF {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 11 {
            Ok(TipoCPF::CPF(s.into()))
        } else {
            Err("Tamanho inválido".into())
        }
    }
}

impl From<String> for TipoCPF {
    fn from(value: String) -> Self {
        TipoCPF::CPF(value)
    }
}

impl From<&str> for TipoCPF {
    fn from(value: &str) -> Self {
        TipoCPF::CPF(value.into())
    }
}

impl From<TipoCPF> for u32 {
    fn from(value: TipoCPF) -> Self {
        match value {
            TipoCPF::CPF(_) => 2,
        }
    }
}

impl Validate for TipoCPF {
    fn validate(&self) -> Result<(), String> {
        match self {
            TipoCPF::CPF(cpf) => validate_cpf(cpf.clone()),
        }
    }
}

impl Serialize for TipoCPF {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TipoCPF::CPF(cpf) => cpf.clone(),
        };
        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for TipoCPF {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let result = TipoCPF::CPF(value);
        result.validate().map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}

//----------------------------------------------------------------------------------------

impl TiposIdentificadores {
    pub fn sem_mascara(&self) -> String {
        match self {
            TiposIdentificadores::CNPJ(cnpj) => {
                cnpj.replace(".", "").replace("/", "").replace("-", "")
            }
            TiposIdentificadores::CPF(cpf) => cpf.replace(".", "").replace("-", ""),
        }
    }

    pub fn descricao(&self) -> String {
        match self {
            TiposIdentificadores::CNPJ(_) => "CNPJ".into(),
            TiposIdentificadores::CPF(_) => "CPF".into(),
        }
    }
}

impl Serialize for TiposIdentificadores {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (tipo_inscricao, numero_incricao) = match self {
            TiposIdentificadores::CNPJ(cnpj) => (1, cnpj.clone()),
            TiposIdentificadores::CPF(cpf) => (2, cpf.clone()),
        };

        let mut map = serializer.serialize_map(Some(2))?;

        map.serialize_key("tpInsc")?;
        map.serialize_value(&tipo_inscricao)?;
        map.serialize_key("nrInsc")?;
        map.serialize_value(&numero_incricao)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for TiposIdentificadores {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer
            .deserialize_map(TiposIdentificadoresVisitor)
            .map_err(serde::de::Error::custom)
    }
}

struct TiposIdentificadoresVisitor;

impl<'de> serde::de::Visitor<'de> for TiposIdentificadoresVisitor {
    type Value = TiposIdentificadores;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_fmt(format_args!("a struct with fields tp_insc and nr_insc"))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut result: Option<TiposIdentificadores> = None;

        while let Some(key) = map.next_key::<String>()? {
            match key.as_str() {
                "nrInsc" => {
                    let nr_insc: String = map.next_value()?;
                    if nr_insc.len() == 14 {
                        result = Some(TiposIdentificadores::CNPJ(nr_insc));
                        break;
                    } else if nr_insc.len() == 11 {
                        result = Some(TiposIdentificadores::CPF(nr_insc));
                        break;
                    }
                }
                _ => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
            }
        }

        if let Some(id) = result {
            Ok(id)
        } else {
            Err(serde::de::Error::custom("Identificador não encontrado"))
        }
    }
}

impl FromStr for TiposIdentificadores {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() == 14 {
            Ok(TiposIdentificadores::CNPJ(s.into()))
        } else if s.len() == 11 {
            Ok(TiposIdentificadores::CPF(s.into()))
        } else {
            Err("Tamanho inválido".into())
        }
    }
}

impl From<String> for TiposIdentificadores {
    fn from(value: String) -> Self {
        TiposIdentificadores::CNPJ(value)
    }
}

impl From<&str> for TiposIdentificadores {
    fn from(value: &str) -> Self {
        TiposIdentificadores::CNPJ(value.into())
    }
}

impl From<TiposIdentificadores> for String {
    fn from(value: TiposIdentificadores) -> Self {
        match value {
            TiposIdentificadores::CNPJ(cnpj) => cnpj,
            TiposIdentificadores::CPF(cpf) => cpf,
        }
    }
}

impl From<&TiposIdentificadores> for String {
    fn from(value: &TiposIdentificadores) -> Self {
        match value {
            TiposIdentificadores::CNPJ(cnpj) => cnpj.clone(),
            TiposIdentificadores::CPF(cpf) => cpf.clone(),
        }
    }
}

impl From<TiposIdentificadores> for u32 {
    fn from(value: TiposIdentificadores) -> Self {
        match value {
            TiposIdentificadores::CNPJ(_) => 1,
            TiposIdentificadores::CPF(_) => 2,
        }
    }
}
impl Validate for TiposIdentificadores {
    fn validate(&self) -> Result<(), String> {
        match self {
            TiposIdentificadores::CNPJ(cnpj) => validate_cnpj(cnpj.clone()),
            TiposIdentificadores::CPF(cpf) => validate_cpf(cpf.clone()),
        }
    }
}

fn validate_cpf(value: String) -> Result<(), String> {
    let cpf_ascii: Vec<usize> = value.chars().map(|c| c as usize - 48).collect();

    if cpf_ascii.len() != 11 {
        return Err("quantidade de dígitos inválido".into());
    }

    let peso1 = [10, 9, 8, 7, 6, 5, 4, 3, 2];
    let peso2 = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];

    let soma1: usize = peso1.iter().zip(cpf_ascii.iter()).map(|(p, n)| p * n).sum();

    let digito1 = if soma1 % 11 < 2 { 0 } else { 11 - (soma1 % 11) };

    let soma2: usize = peso2
        .iter()
        .zip(cpf_ascii.iter().chain(std::iter::once(&digito1)))
        .map(|(p, n)| p * n)
        .sum();

    let digito2 = if soma2 % 11 < 2 { 0 } else { 11 - (soma2 % 11) };

    if cpf_ascii[9] == digito1 && cpf_ascii[10] == digito2 {
        return Ok(());
    }

    Err(format!("cpf inválido: {value}"))
}

fn validate_cnpj(value: String) -> Result<(), String> {
    let cnpj_ascii: Vec<usize> = value.chars().map(|c| c as usize - 48).collect();

    if cnpj_ascii.len() != 14 {
        return Err("quantidade de dígitos inválido".into());
    }

    let peso1 = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
    let peso2 = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

    let soma1: usize = peso1
        .iter()
        .zip(cnpj_ascii.iter())
        .map(|(p, n)| p * n)
        .sum();

    let digito1 = if soma1 % 11 < 2 { 0 } else { 11 - (soma1 % 11) };

    let soma2: usize = peso2
        .iter()
        .zip(cnpj_ascii.iter().chain(std::iter::once(&digito1)))
        .map(|(p, n)| p * n)
        .sum();

    let digito2 = if soma2 % 11 < 2 { 0 } else { 11 - (soma2 % 11) };

    if cnpj_ascii[12] == digito1 && cnpj_ascii[13] == digito2 {
        return Ok(());
    }

    Err(format!("cnpj inválido: {value}"))
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Id(pub TiposIdentificadores, pub NaiveDateTime, pub u32);

impl From<(TiposIdentificadores, NaiveDateTime, u32)> for Id {
    fn from(value: (TiposIdentificadores, NaiveDateTime, u32)) -> Self {
        Id(value.0, value.1, value.2)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let (codigo, doc): (u32, String) = (self.0.clone().into(), self.0.clone().into());

        let temp = format!(
            "ID{}{}{}{}",
            codigo,
            doc.pad(14, '0', Alignment::Left, true),
            self.1.format("%Y%m%d%H%M%S"),
            self.2.to_string().pad(5, '0', Alignment::Right, true)
        );

        serializer.serialize_str(&temp)
    }
}

impl<'de> Deserialize<'de> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if value.len() < 36 {
            return Err(serde::de::Error::invalid_length(value.len(), &"36"));
        }

        let doc = value[3..17].to_string();
        let identi = TiposIdentificadores::from(doc);
        identi.validate().map_err(serde::de::Error::custom)?;
        let datastr = value[17..31].to_string();
        let data = NaiveDateTime::parse_from_str(&datastr, "%Y%m%d%H%M%S").unwrap();

        let sequencia = value[33..].to_string();
        let sequencia = sequencia
            .parse::<u32>()
            .map_err(|_| serde::de::Error::custom("parsing ID error => \"sequência inválida\""))?;

        Ok(Id(identi, data, sequencia))
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum EnumSexo {
    MASCULINO,
    FEMININO,
}

impl Serialize for EnumSexo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            EnumSexo::MASCULINO => "M",
            EnumSexo::FEMININO => "F",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for EnumSexo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "M" => Ok(EnumSexo::MASCULINO),
            "F" => Ok(EnumSexo::FEMININO),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para EnumSexo, deve ser 'M' ou 'F'",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum EnumRacaCor {
    BRANCA,
    PRETA,
    PARDA,
    AMARELA,
    INDIGENA,
    NAOINFORMADO,
}

impl Serialize for EnumRacaCor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            EnumRacaCor::BRANCA => "1",
            EnumRacaCor::PRETA => "2",
            EnumRacaCor::PARDA => "3",
            EnumRacaCor::AMARELA => "4",
            EnumRacaCor::INDIGENA => "5",
            EnumRacaCor::NAOINFORMADO => "6",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for EnumRacaCor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(EnumRacaCor::BRANCA),
            "2" => Ok(EnumRacaCor::PRETA),
            "3" => Ok(EnumRacaCor::PARDA),
            "4" => Ok(EnumRacaCor::AMARELA),
            "5" => Ok(EnumRacaCor::INDIGENA),
            "6" => Ok(EnumRacaCor::NAOINFORMADO),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para EnumRacaCor, deve ser '1', '2', '3', '4', '5' ou '6'",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TipoSimNao {
    SIM,
    NAO,
}

impl Serialize for TipoSimNao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TipoSimNao::SIM => "S",
            TipoSimNao::NAO => "N",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for TipoSimNao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "S" => Ok(TipoSimNao::SIM),
            "N" => Ok(TipoSimNao::NAO),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para TipoSimNao, deve ser 'S' ou 'N'",
            )),
        }
    }
}

impl Display for TipoSimNao {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TipoSimNao::SIM => write!(f, "S"),
            TipoSimNao::NAO => write!(f, "N"),
        }
    }
}
impl From<TipoSimNao> for String {
    fn from(value: TipoSimNao) -> Self {
        match value {
            TipoSimNao::SIM => "S".into(),
            TipoSimNao::NAO => "N".into(),
        }
    }
}
impl From<&TipoSimNao> for String {
    fn from(value: &TipoSimNao) -> Self {
        match value {
            TipoSimNao::SIM => "S".into(),
            TipoSimNao::NAO => "N".into(),
        }
    }
}

impl From<bool> for TipoSimNao {
    fn from(value: bool) -> Self {
        if value {
            TipoSimNao::SIM
        } else {
            TipoSimNao::NAO
        }
    }
}

impl FromStr for TipoSimNao {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(TipoSimNao::SIM),
            "N" => Ok(TipoSimNao::NAO),
            _ => Err("Valor inválido para TipoSimNao, deve ser 'S' ou 'N'".into()),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct Texto<const MIN: usize, const MAX: usize>(pub String);
pub type Nome = Texto<1, 70>;
pub type Matricula = Texto<1, 30>;
pub type CodigoEsocial = Texto<1, 30>;
pub type Descricao = Texto<1, 300>;
pub type LocalTrabalhoGeral = Texto<1, 80>;
pub type Observacao = Texto<1, 1024>;
pub type Texto255 = Texto<1, 255>;
pub type CBO = Texto<6, 6>;
pub type Nome100 = Texto<1, 100>;
pub type DescricaoSalario = Texto<1, 1000>;

impl<const MIN: usize, const MAX: usize> Serialize for Texto<MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let truncated = if self.0.len() > MAX {
            self.0.chars().take(MAX).collect::<String>()
        } else {
            self.0.clone()
        };
        serializer.serialize_str(&truncated)
    }
}

impl<const MIN: usize, const MAX: usize> Validate for Texto<MIN, MAX> {
    fn validate(&self) -> Result<(), String> {
        if self.0.len() < MIN {
            return Err(format!("Texto deve conter no mínimo {MIN} caracteres"));
        }
        if self.0.len() > MAX {
            return Err(format!("Texto deve conter no máximo {MAX} caracteres"));
        }
        Ok(())
    }
}

impl<'de, const MIN: usize, const MAX: usize> Deserialize<'de> for Texto<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let result = Texto::<MIN, MAX>(value);
        result.validate().map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}
impl<const MIN: usize, const MAX: usize> From<Texto<MIN, MAX>> for String {
    fn from(value: Texto<MIN, MAX>) -> Self {
        value.0
    }
}
impl<const MIN: usize, const MAX: usize> From<&Texto<MIN, MAX>> for String {
    fn from(value: &Texto<MIN, MAX>) -> Self {
        value.0.clone()
    }
}
impl<const MIN: usize, const MAX: usize> From<String> for Texto<MIN, MAX> {
    fn from(value: String) -> Self {
        Texto::<MIN, MAX>(value)
    }
}
impl<const MIN: usize, const MAX: usize> From<&str> for Texto<MIN, MAX> {
    fn from(value: &str) -> Self {
        Texto::<MIN, MAX>(value.into())
    }
}
impl<const MIN: usize, const MAX: usize> From<&String> for Texto<MIN, MAX> {
    fn from(value: &String) -> Self {
        Texto::<MIN, MAX>(value.clone())
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct Numero<const MIN: usize, const MAX: usize>(pub String);
pub type TipoDependente = Numero<2, 2>;
pub type Contato = Numero<8, 13>;
pub type CodTreinoCapacitacao = Numero<1, 4>;

impl<const MIN: usize, const MAX: usize> Validate for Numero<MIN, MAX> {
    fn validate(&self) -> Result<(), String> {
        let re = regex::Regex::new(&format!(r"^\d{{{MIN},{MAX}}}$")).unwrap();
        if !re.is_match(&self.0) {
            return Err(format!(
                "Código ({}) deveria conter entre {} e {} dígitos numéricos",
                self.0, MIN, MAX
            ));
        }
        Ok(())
    }
}

impl<'de, const MIN: usize, const MAX: usize> Deserialize<'de> for Numero<MIN, MAX> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let result = Numero::<MIN, MAX>(value);
        result.validate().map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}

impl<const MIN: usize, const MAX: usize> Serialize for Numero<MIN, MAX> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.validate().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&self.0)
    }
}

impl<const MIN: usize, const MAX: usize> From<Numero<MIN, MAX>> for String {
    fn from(value: Numero<MIN, MAX>) -> Self {
        value.0
    }
}

impl<const MIN: usize, const MAX: usize> From<&Numero<MIN, MAX>> for String {
    fn from(value: &Numero<MIN, MAX>) -> Self {
        value.0.clone()
    }
}
impl<const MIN: usize, const MAX: usize> From<String> for Numero<MIN, MAX> {
    fn from(value: String) -> Self {
        Numero::<MIN, MAX>(value)
    }
}
impl<const MIN: usize, const MAX: usize> From<&str> for Numero<MIN, MAX> {
    fn from(value: &str) -> Self {
        Numero::<MIN, MAX>(value.into())
    }
}

impl<const MIN: usize, const MAX: usize> From<&String> for Numero<MIN, MAX> {
    fn from(value: &String) -> Self {
        Numero::<MIN, MAX>(value.clone())
    }
}

impl<const MIN: usize, const MAX: usize> From<u32> for Numero<MIN, MAX> {
    fn from(value: u32) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}
impl<const MIN: usize, const MAX: usize> From<i32> for Numero<MIN, MAX> {
    fn from(value: i32) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}

impl<const MIN: usize, const MAX: usize> From<u64> for Numero<MIN, MAX> {
    fn from(value: u64) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}
impl<const MIN: usize, const MAX: usize> From<i64> for Numero<MIN, MAX> {
    fn from(value: i64) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}

impl<const MIN: usize, const MAX: usize> From<u128> for Numero<MIN, MAX> {
    fn from(value: u128) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}
impl<const MIN: usize, const MAX: usize> From<i128> for Numero<MIN, MAX> {
    fn from(value: i128) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}

impl<const MIN: usize, const MAX: usize> From<usize> for Numero<MIN, MAX> {
    fn from(value: usize) -> Self {
        Numero::<MIN, MAX>(value.to_string())
    }
}

impl<const MIN: usize, const MAX: usize> Display for Numero<MIN, MAX> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Default, Clone)]
pub struct Decimal<const MIN: usize, const MAX: usize, const FRACTION: usize>(pub String);
pub type ValorMonetario = Decimal<1, 14, 2>;

impl<const MIN: usize, const MAX: usize, const FRACTION: usize> Serialize
    for Decimal<MIN, MAX, FRACTION>
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.validate().map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&self.0)
    }
}

impl<'de, const MIN: usize, const MAX: usize, const FRACTION: usize> Deserialize<'de>
    for Decimal<MIN, MAX, FRACTION>
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        let result = Decimal::<MIN, MAX, FRACTION>(value);
        result.validate().map_err(serde::de::Error::custom)?;
        Ok(result)
    }
}

impl<const MIN: usize, const MAX: usize, const FRACTION: usize> Validate
    for Decimal<MIN, MAX, FRACTION>
{
    fn validate(&self) -> Result<(), String> {
        let re = regex::Regex::new(&format!(r"^\d{{{MIN},{MAX}}}\.\d{{{FRACTION}}}$")).unwrap();
        if !re.is_match(&self.0) {
            return Err(format!(
                "Decimal ({}) deveria conter entre {} e {} dígitos numéricos com {} casas decimais",
                self.0, MIN, MAX, FRACTION
            ));
        }
        Ok(())
    }
}

impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<Decimal<MIN, MAX, FRACTION>>
    for String
{
    fn from(value: Decimal<MIN, MAX, FRACTION>) -> Self {
        value.0
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<&Decimal<MIN, MAX, FRACTION>>
    for String
{
    fn from(value: &Decimal<MIN, MAX, FRACTION>) -> Self {
        value.0.clone()
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<String>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: String) -> Self {
        Decimal::<MIN, MAX, FRACTION>(value)
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<&str>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: &str) -> Self {
        Decimal::<MIN, MAX, FRACTION>(value.into())
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<&String>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: &String) -> Self {
        Decimal::<MIN, MAX, FRACTION>(value.clone())
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<usize>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: usize) -> Self {
        let temp = format!("{:.FRACTION$}", value as f32);
        Decimal::<MIN, MAX, FRACTION>(temp)
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<i32>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: i32) -> Self {
        let temp = format!("{:.FRACTION$}", value as f32);
        Decimal::<MIN, MAX, FRACTION>(temp)
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<f32>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: f32) -> Self {
        let temp = format!("{value:.FRACTION$}");
        Decimal::<MIN, MAX, FRACTION>(temp)
    }
}
impl<const MIN: usize, const MAX: usize, const FRACTION: usize> From<f64>
    for Decimal<MIN, MAX, FRACTION>
{
    fn from(value: f64) -> Self {
        let temp = format!("{value:.FRACTION$}");
        Decimal::<MIN, MAX, FRACTION>(temp)
    }
}

//----------------------------------------------------------------------------------------

type NumeroSequencial = Numero<1, 19>;

#[derive(Debug)]
pub struct NumeroRecibo(pub AgentProcessamento, pub Ambiente, pub NumeroSequencial);

impl Serialize for NumeroRecibo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let agente = u32::from(self.0.clone());
        let ambiente = u32::from(self.1.clone());

        let value = format!(
            "{}{}{}",
            agente,
            ambiente,
            self.2.to_string().pad(19, '0', Alignment::Right, true)
        );

        serializer.serialize_str(&value)
    }
}

impl<'de> Deserialize<'de> for NumeroRecibo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;

        if value.len() < 21 {
            return Err(serde::de::Error::invalid_length(value.len(), &"21"));
        }

        let agente = u32::from_str(&value[0..1]).map_err(serde::de::Error::custom)?;
        let ambiente = u32::from_str(&value[1..2]).map_err(serde::de::Error::custom)?;
        let sequencial = NumeroSequencial::from(value[2..].to_string());

        Ok(NumeroRecibo(
            AgentProcessamento::from(agente),
            Ambiente::from(ambiente),
            sequencial,
        ))
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum AgentProcessamento {
    Serpro,
}

impl Serialize for AgentProcessamento {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            AgentProcessamento::Serpro => 1,
        };
        serializer.serialize_u32(value)
    }
}

impl<'de> Deserialize<'de> for AgentProcessamento {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(AgentProcessamento::Serpro),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para AgentProcessamento, deve ser 'SERPRO'",
            )),
        }
    }
}

impl From<u32> for AgentProcessamento {
    fn from(value: u32) -> Self {
        match value {
            1 => AgentProcessamento::Serpro,
            _ => panic!("Valor inválido para AgentProcessamento"),
        }
    }
}

impl From<&u32> for AgentProcessamento {
    fn from(value: &u32) -> Self {
        match *value {
            1 => AgentProcessamento::Serpro,
            _ => panic!("Valor inválido para AgentProcessamento"),
        }
    }
}

impl From<AgentProcessamento> for u32 {
    fn from(value: AgentProcessamento) -> Self {
        match value {
            AgentProcessamento::Serpro => 1,
        }
    }
}

impl From<&AgentProcessamento> for u32 {
    fn from(value: &AgentProcessamento) -> Self {
        match value {
            AgentProcessamento::Serpro => 1,
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum GrauInstrucao {
    Analfabeto,
    Ate5Ano,
    Completo5Ano,
    Do6ao9Ano,
    FundamentalCompleto,
    EnsinoMedioIncompleto,
    EducacaoSuperiorIncompleta,
    EducacaoSuperiorCompleta,
    PosGraduacaoCompleta,
    MestradoCompleto,
    DoutoradoCompleto,
}

impl Serialize for GrauInstrucao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            GrauInstrucao::Analfabeto => "01",
            GrauInstrucao::Ate5Ano => "02",
            GrauInstrucao::Completo5Ano => "03",
            GrauInstrucao::Do6ao9Ano => "04",
            GrauInstrucao::FundamentalCompleto => "05",
            GrauInstrucao::EnsinoMedioIncompleto => "06",
            GrauInstrucao::EducacaoSuperiorIncompleta => "07",
            GrauInstrucao::EducacaoSuperiorCompleta => "08",
            GrauInstrucao::PosGraduacaoCompleta => "09",
            GrauInstrucao::MestradoCompleto => "10",
            GrauInstrucao::DoutoradoCompleto => "11",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for GrauInstrucao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "01" => Ok(GrauInstrucao::Analfabeto),
            "02" => Ok(GrauInstrucao::Ate5Ano),
            "03" => Ok(GrauInstrucao::Completo5Ano),
            "04" => Ok(GrauInstrucao::Do6ao9Ano),
            "05" => Ok(GrauInstrucao::FundamentalCompleto),
            "06" => Ok(GrauInstrucao::EnsinoMedioIncompleto),
            "07" => Ok(GrauInstrucao::EducacaoSuperiorIncompleta),
            "08" => Ok(GrauInstrucao::EducacaoSuperiorCompleta),
            "09" => Ok(GrauInstrucao::PosGraduacaoCompleta),
            "10" => Ok(GrauInstrucao::MestradoCompleto),
            "11" => Ok(GrauInstrucao::DoutoradoCompleto),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para GrauInstrucao",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum EnumUf {
    AC,
    AL,
    AP,
    AM,
    BA,
    CE,
    DF,
    ES,
    GO,
    MA,
    MT,
    MS,
    MG,
    PA,
    PB,
    PR,
    PE,
    PI,
    RJ,
    RN,
    RS,
    RO,
    RR,
    SC,
    SP,
    SE,
    TO,
}

impl Serialize for EnumUf {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            EnumUf::AC => "AC",
            EnumUf::AL => "AL",
            EnumUf::AP => "AP",
            EnumUf::AM => "AM",
            EnumUf::BA => "BA",
            EnumUf::CE => "CE",
            EnumUf::DF => "DF",
            EnumUf::ES => "ES",
            EnumUf::GO => "GO",
            EnumUf::MA => "MA",
            EnumUf::MT => "MT",
            EnumUf::MS => "MS",
            EnumUf::MG => "MG",
            EnumUf::PA => "PA",
            EnumUf::PB => "PB",
            EnumUf::PR => "PR",
            EnumUf::PE => "PE",
            EnumUf::PI => "PI",
            EnumUf::RJ => "RJ",
            EnumUf::RN => "RN",
            EnumUf::RS => "RS",
            EnumUf::RO => "RO",
            EnumUf::RR => "RR",
            EnumUf::SC => "SC",
            EnumUf::SP => "SP",
            EnumUf::SE => "SE",
            EnumUf::TO => "TO",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for EnumUf {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "AC" => Ok(EnumUf::AC),
            "AL" => Ok(EnumUf::AL),
            "AP" => Ok(EnumUf::AP),
            "AM" => Ok(EnumUf::AM),
            "BA" => Ok(EnumUf::BA),
            "CE" => Ok(EnumUf::CE),
            "DF" => Ok(EnumUf::DF),
            "ES" => Ok(EnumUf::ES),
            "GO" => Ok(EnumUf::GO),
            "MA" => Ok(EnumUf::MA),
            "MT" => Ok(EnumUf::MT),
            "MS" => Ok(EnumUf::MS),
            "MG" => Ok(EnumUf::MG),
            "PA" => Ok(EnumUf::PA),
            "PB" => Ok(EnumUf::PB),
            "PR" => Ok(EnumUf::PR),
            "PE" => Ok(EnumUf::PE),
            "PI" => Ok(EnumUf::PI),
            "RJ" => Ok(EnumUf::RJ),
            "RN" => Ok(EnumUf::RN),
            "RS" => Ok(EnumUf::RS),
            "RO" => Ok(EnumUf::RO),
            "RR" => Ok(EnumUf::RR),
            "SC" => Ok(EnumUf::SC),
            "SP" => Ok(EnumUf::SP),
            "SE" => Ok(EnumUf::SE),
            "TO" => Ok(EnumUf::TO),
            _ => Err(serde::de::Error::custom("Valor inválido para EnumUf")),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum RegimeTrabalhista {
    CLT,
    Estatutario,
}

impl Serialize for RegimeTrabalhista {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            RegimeTrabalhista::CLT => "1",
            RegimeTrabalhista::Estatutario => "2",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for RegimeTrabalhista {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(RegimeTrabalhista::CLT),
            "2" => Ok(RegimeTrabalhista::Estatutario),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para RegimeTrabalhista",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum RegimePrevidenciario {
    RegimeGeral,
    RegimeProprio,
    RegimeExterior,
    SistemaProtecaoMilitares,
}

impl Serialize for RegimePrevidenciario {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            RegimePrevidenciario::RegimeGeral => "1",
            RegimePrevidenciario::RegimeProprio => "2",
            RegimePrevidenciario::RegimeExterior => "3",
            RegimePrevidenciario::SistemaProtecaoMilitares => "4",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for RegimePrevidenciario {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(RegimePrevidenciario::RegimeGeral),
            "2" => Ok(RegimePrevidenciario::RegimeProprio),
            "3" => Ok(RegimePrevidenciario::RegimeExterior),
            "4" => Ok(RegimePrevidenciario::SistemaProtecaoMilitares),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para RegimePrevidenciario",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum EnumEstadoCivil {
    Solteiro,
    Casado,
    Divorciado,
    Separado,
    Viuvo,
}

impl Serialize for EnumEstadoCivil {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            EnumEstadoCivil::Solteiro => "1",
            EnumEstadoCivil::Casado => "2",
            EnumEstadoCivil::Divorciado => "3",
            EnumEstadoCivil::Separado => "4",
            EnumEstadoCivil::Viuvo => "5",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for EnumEstadoCivil {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(EnumEstadoCivil::Solteiro),
            "2" => Ok(EnumEstadoCivil::Casado),
            "3" => Ok(EnumEstadoCivil::Divorciado),
            "4" => Ok(EnumEstadoCivil::Separado),
            "5" => Ok(EnumEstadoCivil::Viuvo),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para EnumEstadoCivil",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(PartialEq, Eq, Debug, Deserialize)]
pub struct Periodo(pub DateTime<Local>, pub Option<DateTime<Local>>);

impl Display for Periodo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = format!("{}", self.0.format("%Y-%m"));

        if let Some(data) = self.1 {
            let _ = result.write_str(format!(",{}", data.format("%Y-%m")).as_str());
        }

        f.write_str(result.as_str())
    }
}

impl Serialize for Periodo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TipoAdmissao {
    Admissao,
    TransferenciaEmpresaMesmoGrupo,
    TransferenciaEmpresaConsorciada,
    TransferenciaPorSucessao,
    TransferenciaEmpregadoDomestico,
    MudancaCPF,
    TransferenciaEmpresaInapta,
}

impl Serialize for TipoAdmissao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TipoAdmissao::Admissao => "1",
            TipoAdmissao::TransferenciaEmpresaMesmoGrupo => "2",
            TipoAdmissao::TransferenciaEmpresaConsorciada => "3",
            TipoAdmissao::TransferenciaPorSucessao => "4",
            TipoAdmissao::TransferenciaEmpregadoDomestico => "5",
            TipoAdmissao::MudancaCPF => "6",
            TipoAdmissao::TransferenciaEmpresaInapta => "7",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for TipoAdmissao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(TipoAdmissao::Admissao),
            "2" => Ok(TipoAdmissao::TransferenciaEmpresaMesmoGrupo),
            "3" => Ok(TipoAdmissao::TransferenciaEmpresaConsorciada),
            "4" => Ok(TipoAdmissao::TransferenciaPorSucessao),
            "5" => Ok(TipoAdmissao::TransferenciaEmpregadoDomestico),
            "6" => Ok(TipoAdmissao::MudancaCPF),
            "7" => Ok(TipoAdmissao::TransferenciaEmpresaInapta),
            _ => Err(serde::de::Error::custom("Valor inválido para TipoAdmissao")),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum IndicativoAdmissao {
    Normal,
    AçãoFiscal,
    DecisãoJudicial,
}

impl Serialize for IndicativoAdmissao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            IndicativoAdmissao::Normal => "1",
            IndicativoAdmissao::AçãoFiscal => "2",
            IndicativoAdmissao::DecisãoJudicial => "3",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for IndicativoAdmissao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(IndicativoAdmissao::Normal),
            "2" => Ok(IndicativoAdmissao::AçãoFiscal),
            "3" => Ok(IndicativoAdmissao::DecisãoJudicial),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para IndicativoAdmissao",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum RegimeJornada {
    SubmetidoHorarioTrabalho,
    AtividadeExternaIncisoI,
    FuncaoIncisoII,
    Teletrabalho,
}

impl Serialize for RegimeJornada {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            RegimeJornada::SubmetidoHorarioTrabalho => "1",
            RegimeJornada::AtividadeExternaIncisoI => "2",
            RegimeJornada::FuncaoIncisoII => "3",
            RegimeJornada::Teletrabalho => "4",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for RegimeJornada {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(RegimeJornada::SubmetidoHorarioTrabalho),
            "2" => Ok(RegimeJornada::AtividadeExternaIncisoI),
            "3" => Ok(RegimeJornada::FuncaoIncisoII),
            "4" => Ok(RegimeJornada::Teletrabalho),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para RegimeJornada",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum NaturezaAtividade {
    TrabalhoUrbano,
    TrabalhoRural,
}

impl Serialize for NaturezaAtividade {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            NaturezaAtividade::TrabalhoUrbano => "1",
            NaturezaAtividade::TrabalhoRural => "2",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for NaturezaAtividade {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(NaturezaAtividade::TrabalhoUrbano),
            "2" => Ok(NaturezaAtividade::TrabalhoRural),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para NaturezaAtividade",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum DataMes {
    Janeiro,
    Fevereiro,
    Março,
    Abril,
    Maio,
    Junho,
    Julho,
    Agosto,
    Setembro,
    Outubro,
    Novembro,
    Dezembro,
}

impl Serialize for DataMes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            DataMes::Janeiro => "01",
            DataMes::Fevereiro => "02",
            DataMes::Março => "03",
            DataMes::Abril => "04",
            DataMes::Maio => "05",
            DataMes::Junho => "06",
            DataMes::Julho => "07",
            DataMes::Agosto => "08",
            DataMes::Setembro => "09",
            DataMes::Outubro => "10",
            DataMes::Novembro => "11",
            DataMes::Dezembro => "12",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for DataMes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "01" => Ok(DataMes::Janeiro),
            "02" => Ok(DataMes::Fevereiro),
            "03" => Ok(DataMes::Março),
            "04" => Ok(DataMes::Abril),
            "05" => Ok(DataMes::Maio),
            "06" => Ok(DataMes::Junho),
            "07" => Ok(DataMes::Julho),
            "08" => Ok(DataMes::Agosto),
            "09" => Ok(DataMes::Setembro),
            "10" => Ok(DataMes::Outubro),
            "11" => Ok(DataMes::Novembro),
            "12" => Ok(DataMes::Dezembro),
            _ => Err(serde::de::Error::custom("Valor inválido para DataMes")),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Data(pub NaiveDate);

impl<'de> Deserialize<'de> for Data {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = match NaiveDate::parse_from_str(&s, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => {
                return Err(serde::de::Error::custom(format!(
                    "Invalid date format: {s}"
                )));
            }
        };
        Ok(Data(date))
    }
}
impl Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.0.format("%Y-%m-%d")).as_str())
    }
}

impl Serialize for Data {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}
impl From<NaiveDateTime> for Data {
    fn from(date: NaiveDateTime) -> Self {
        Data(date.into())
    }
}

impl From<&NaiveDateTime> for Data {
    fn from(date: &NaiveDateTime) -> Self {
        Data(date.date())
    }
}

impl From<Data> for NaiveDateTime {
    fn from(data: Data) -> Self {
        data.0.into()
    }
}

impl From<&Data> for NaiveDateTime {
    fn from(data: &Data) -> Self {
        data.0.into()
    }
}

impl From<NaiveDate> for Data {
    fn from(date: NaiveDate) -> Self {
        Data(date)
    }
}

impl From<&NaiveDate> for Data {
    fn from(date: &NaiveDate) -> Self {
        Data(*date)
    }
}

impl From<Data> for NaiveDate {
    fn from(data: Data) -> Self {
        data.0
    }
}

impl From<&Data> for NaiveDate {
    fn from(data: &Data) -> Self {
        data.0
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum HipoteseContratacaoTemporario {
    SubstituicaoTransitoria,
    DemandaComplementar,
}
impl Serialize for HipoteseContratacaoTemporario {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            HipoteseContratacaoTemporario::SubstituicaoTransitoria => "1",
            HipoteseContratacaoTemporario::DemandaComplementar => "2",
        };
        serializer.serialize_str(value)
    }
}
impl<'de> Deserialize<'de> for HipoteseContratacaoTemporario {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1" => Ok(HipoteseContratacaoTemporario::SubstituicaoTransitoria),
            "2" => Ok(HipoteseContratacaoTemporario::DemandaComplementar),
            _ => Err(serde::de::Error::custom(
                "Valor inválido para HipoteseContratacaoTemporario",
            )),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub enum TipoPlanoRP {
    SemSegregacaoMassa,
    FundoCapitalizacao,
    FundoReparticao,
    MantidoTesouro,
}

impl Serialize for TipoPlanoRP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            TipoPlanoRP::SemSegregacaoMassa => "0",
            TipoPlanoRP::FundoCapitalizacao => "1",
            TipoPlanoRP::FundoReparticao => "2",
            TipoPlanoRP::MantidoTesouro => "3",
        };
        serializer.serialize_str(value)
    }
}

impl<'de> Deserialize<'de> for TipoPlanoRP {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "0" => Ok(TipoPlanoRP::SemSegregacaoMassa),
            "1" => Ok(TipoPlanoRP::FundoCapitalizacao),
            "2" => Ok(TipoPlanoRP::FundoReparticao),
            "3" => Ok(TipoPlanoRP::MantidoTesouro),
            _ => Err(serde::de::Error::custom("Valor inválido para TipoPlanoRP")),
        }
    }
}

impl FromStr for TipoPlanoRP {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(TipoPlanoRP::SemSegregacaoMassa),
            "1" => Ok(TipoPlanoRP::FundoCapitalizacao),
            "2" => Ok(TipoPlanoRP::FundoReparticao),
            "3" => Ok(TipoPlanoRP::MantidoTesouro),
            _ => Err(format!("TipoPlanoRP inválido: {s}")),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug)]
pub enum IndRetificacao {
    Original,
    Retificacao,
}

impl Serialize for IndRetificacao {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let value = match self {
            IndRetificacao::Original => 1,
            IndRetificacao::Retificacao => 2,
        };
        serializer.serialize_u32(value)
    }
}
impl<'de> Deserialize<'de> for IndRetificacao {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(IndRetificacao::Original),
            2 => Ok(IndRetificacao::Retificacao),
            _ => Err(serde::de::Error::custom(format!(
                "Valor inválido para IndRetificacao: {value}"
            ))),
        }
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone)]
#[repr(u32)]
pub enum Ambiente {
    Producao = 1,
    PreProducaoDadosReais = 2,
    PreProducaoDadosFicticios = 3,
    Homologacao = 6,
    Validacao = 7,
    Testes = 8,
    Desenvolvimento = 9,
}
impl Serialize for Ambiente {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.into())
    }
}
impl<'de> Deserialize<'de> for Ambiente {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(Ambiente::Producao),
            2 => Ok(Ambiente::PreProducaoDadosReais),
            3 => Ok(Ambiente::PreProducaoDadosFicticios),
            6 => Ok(Ambiente::Homologacao),
            7 => Ok(Ambiente::Validacao),
            8 => Ok(Ambiente::Testes),
            9 => Ok(Ambiente::Desenvolvimento),
            _ => Err(serde::de::Error::custom(format!(
                "Valor inválido para Ambiente: {value}"
            ))),
        }
    }
}
impl From<u32> for Ambiente {
    fn from(value: u32) -> Self {
        match value {
            1 => Ambiente::Producao,
            2 => Ambiente::PreProducaoDadosReais,
            3 => Ambiente::PreProducaoDadosFicticios,
            6 => Ambiente::Homologacao,
            7 => Ambiente::Validacao,
            8 => Ambiente::Testes,
            9 => Ambiente::Desenvolvimento,
            _ => panic!("Ambiente inválido: {value}"),
        }
    }
}
impl From<&u32> for Ambiente {
    fn from(value: &u32) -> Self {
        match value {
            1 => Ambiente::Producao,
            2 => Ambiente::PreProducaoDadosReais,
            3 => Ambiente::PreProducaoDadosFicticios,
            6 => Ambiente::Homologacao,
            7 => Ambiente::Validacao,
            8 => Ambiente::Testes,
            9 => Ambiente::Desenvolvimento,
            _ => panic!("Ambiente inválido: {value}"),
        }
    }
}

impl From<Ambiente> for u32 {
    fn from(value: Ambiente) -> Self {
        value as u32
    }
}

impl From<&Ambiente> for u32 {
    fn from(value: &Ambiente) -> Self {
        value.clone() as u32
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum OrigemEvento {
    AplicativoEmpregador = 1,
    AplicativoGovernamentalSimplificadoPF = 2,
    AplicativoGovernamentalWebGeral = 3,
    AplicativoGovernamentalSimplificadoPJ = 4,
    AplicativoGovernamentalJudiciario = 8,
    AplicativoGovernamentalIntegracaoJuntaComercial = 9,
    AplicativoGovernamentalMovelEmpregadorDomestico = 22,
}

impl Serialize for OrigemEvento {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(self.into())
    }
}

impl<'de> Deserialize<'de> for OrigemEvento {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        match value {
            1 => Ok(OrigemEvento::AplicativoEmpregador),
            2 => Ok(OrigemEvento::AplicativoGovernamentalSimplificadoPF),
            3 => Ok(OrigemEvento::AplicativoGovernamentalWebGeral),
            4 => Ok(OrigemEvento::AplicativoGovernamentalSimplificadoPJ),
            8 => Ok(OrigemEvento::AplicativoGovernamentalJudiciario),
            9 => Ok(OrigemEvento::AplicativoGovernamentalIntegracaoJuntaComercial),
            22 => Ok(OrigemEvento::AplicativoGovernamentalMovelEmpregadorDomestico),
            _ => Err(serde::de::Error::custom(format!(
                "Valor inválido para OrigemEvento: {value}"
            ))),
        }
    }
}

impl From<u32> for OrigemEvento {
    fn from(value: u32) -> Self {
        match value {
            1 => OrigemEvento::AplicativoEmpregador,
            2 => OrigemEvento::AplicativoGovernamentalSimplificadoPF,
            3 => OrigemEvento::AplicativoGovernamentalWebGeral,
            4 => OrigemEvento::AplicativoGovernamentalSimplificadoPJ,
            8 => OrigemEvento::AplicativoGovernamentalJudiciario,
            9 => OrigemEvento::AplicativoGovernamentalIntegracaoJuntaComercial,
            22 => OrigemEvento::AplicativoGovernamentalMovelEmpregadorDomestico,
            _ => panic!("OrigemEvento inválido: {value}"),
        }
    }
}

impl From<&u32> for OrigemEvento {
    fn from(value: &u32) -> Self {
        match value {
            1 => OrigemEvento::AplicativoEmpregador,
            2 => OrigemEvento::AplicativoGovernamentalSimplificadoPF,
            3 => OrigemEvento::AplicativoGovernamentalWebGeral,
            4 => OrigemEvento::AplicativoGovernamentalSimplificadoPJ,
            8 => OrigemEvento::AplicativoGovernamentalJudiciario,
            9 => OrigemEvento::AplicativoGovernamentalIntegracaoJuntaComercial,
            22 => OrigemEvento::AplicativoGovernamentalMovelEmpregadorDomestico,
            _ => panic!("OrigemEvento inválido: {value}"),
        }
    }
}

impl From<OrigemEvento> for u32 {
    fn from(value: OrigemEvento) -> Self {
        value as u32
    }
}

impl From<&OrigemEvento> for u32 {
    fn from(value: &OrigemEvento) -> Self {
        value.clone() as u32
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentEvento {
    pub tp_amb: Ambiente,
    pub proc_emi: OrigemEvento,
    pub ver_proc: u32,
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TIdePeriodo {
    pub ini_validade: String,
    pub fim_validade: String,
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DadosInsencao {
    pub ide_min_lei: u8,
    pub nr_certif: usize,
    pub dt_emis_certif: Data,
    pub dt_venc_certif: Data,
    pub nr_prot_renov: String,
    pub dt_prot_renov: Data,
    pub dt_dou: Data,
    pub pag_dou: usize,
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InfoOrgInternacional {
    pub ind_acordo_isen_multa: String,
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TInfoCadasro {
    pub class_trib: u8,
    pub ind_coop: u8,
    pub ind_constr: u8,
    pub ind_des_folha: u8,
    pub ind_opc_cp: u8,
    pub ind_port: u8,
    pub ind_opt_reg_eletron: u8,
    pub cnpj_efr: TipoCNPJ,
    pub dt_trans_1096: Data,
    pub ind_tribo_folha_pis_pasep: u8,
    pub dados_isencao: DadosInsencao,
    pub info_org_internacional: InfoOrgInternacional,
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inclusao {
    ide_periodo: TIdePeriodo,
    info_cadastro: TInfoCadasro,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alteracao {
    pub ide_periodo: TIdePeriodo,
    pub info_cadastro: TInfoCadasro,
    pub nova_validade: TIdePeriodo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exclusao {
    ide_periodo: TIdePeriodo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EnumInfoEmpregador {
    Inclusao(Inclusao),
    Alteracao(Alteracao),
    Exclusao(Exclusao),
}

impl From<Inclusao> for EnumInfoEmpregador {
    fn from(value: Inclusao) -> Self {
        EnumInfoEmpregador::Inclusao(value)
    }
}

impl From<Alteracao> for EnumInfoEmpregador {
    fn from(value: Alteracao) -> Self {
        EnumInfoEmpregador::Alteracao(value)
    }
}

impl From<Exclusao> for EnumInfoEmpregador {
    fn from(value: Exclusao) -> Self {
        EnumInfoEmpregador::Exclusao(value)
    }
}

//----------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipoInfoEmpregador {
    #[serde(rename = "$value")]
    pub value: EnumInfoEmpregador,
}
