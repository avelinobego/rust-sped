
#[test]
// #[allow(unused_imports)]
pub fn tests_evt_adm_prelim() {
    use crate::eventos::evt_s2190::EvtAdmPrelim;
    use quick_xml::se::to_string;
    use chrono::TimeZone;

    let hoje = chrono::Local::with_ymd_and_hms(&chrono::Local, 2023, 3, 15, 12, 0, 0)
        .unwrap()
        .naive_utc();

    let id = crate::tipos::Id("13378331000101".into(), hoje, 1);

    let info_reg_ctps = crate::eventos::evt_s2190::InfoRegCTPS {
        cbo_cargo: Some("123456".into()),
        vr_sal_fx: Some(1500.into()),
        und_sal_fixo: Some(5),      // Por mês
        tp_contr: Some(1),          // Prazo indeterminado
        dt_term: Some(hoje.into()), // Não aplicável para contrato indeterminado
    };

    let info_reg = crate::eventos::evt_s2190::InfoRegPrelim {
        cpf_trab: "12345678901".into(),
        dt_nasc: hoje.into(),
        dt_adm: hoje.into(),
        matricula: "123456".into(),
        cod_categ: 1,
        nat_atividade: Some(crate::tipos::NaturezaAtividade::TrabalhoUrbano),
        info_reg_ctps: Some(info_reg_ctps),
    };
    let evt = EvtAdmPrelim {
        id,
        ide_empregador: "13378331000101".into(),
        info_reg_prelim: Some(info_reg),
    };

    let serialized = to_string(&evt).unwrap();

    println!("Serialized EvtAdmPrelim: {serialized}");
}
