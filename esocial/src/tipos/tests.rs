
#[test]
fn test_periodo() {
    let p1 = crate::tipos::Periodo(chrono::Local::now(), Some(chrono::Local::now()));
    let p2 = crate::tipos::Periodo(chrono::Local::now(), None);
    assert_ne!(p1, p2);
}

#[test]
fn test_cpf() {
    use crate::tipos::Validate;
    let cpf1 = crate::tipos::TipoCPF::CPF("62510507052".into());
    cpf1.validate().unwrap();
    let cpf2 = crate::tipos::TipoCPF::CPF("16944301890".into());
    cpf2.validate().unwrap();
    assert_ne!(cpf1, cpf2)
}

#[test]
fn test_cnpj() {
    use crate::tipos::Validate;
    let cnpj1 = crate::tipos::TipoCNPJ::CNPJ("04126001000138".into());
    let cnpj2 = crate::tipos::TipoCNPJ::CNPJ("04126001000138".into());
    cnpj1.validate().unwrap();
    cnpj2.validate().unwrap();
    assert_eq!(cnpj1, cnpj2);

    let cnpj3 = crate::tipos::TipoCNPJ::CNPJ("12ABC34501DE35".into());
    let cnpj4 = crate::tipos::TipoCNPJ::CNPJ("12ABC34501DE35".into());
    cnpj3.validate().unwrap();
    cnpj4.validate().unwrap();
    assert_eq!(cnpj3, cnpj4);
}

#[test]
pub fn test_tipo_inscricao() {
    assert_eq!(1, crate::tipos::TipoCNPJ::CNPJ("".into()).tipo_inscricao());
    assert_eq!(2, crate::tipos::TipoCPF::CPF("".into()).tipo_inscricao());

    let mut tipo: u32  = crate::tipos::TipoCNPJ::CNPJ("".into()).into();
    assert_eq!(1, tipo);
    tipo = crate::tipos::TipoCPF::CPF("".into()).into();
    assert_eq!(2, tipo);
}
