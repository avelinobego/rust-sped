
#[test]
pub fn test_evento_s1000() {
    use crate::eventos::evt_s1000::EvtInfoEmpregador;
    use crate::tipos::OrigemEvento;
    use crate::tipos::{
        Alteracao, DadosInsencao, InfoOrgInternacional, TIdePeriodo, TInfoCadasro, TipoCNPJ
    };

    let hoje = chrono::Local::now().naive_local();
    let data = crate::tipos::Data(hoje.into());

    let ide_evento = crate::tipos::IdentEvento {
        tp_amb: crate::tipos::Ambiente::Testes,
        proc_emi: OrigemEvento::AplicativoGovernamentalWebGeral,
        ver_proc: 1,
    };

    let id = crate::tipos::Id("13378331000101".into(), hoje, 1);

    //TODO: Modificar os tipos de data de string para Data
    let evento = EvtInfoEmpregador {
        id,
        ide_evento,
        info_empregador: crate::tipos::TipoInfoEmpregador {
            value: crate::tipos::EnumInfoEmpregador::Alteracao(Alteracao {
                ide_periodo: TIdePeriodo {
                    ini_validade: "2005-01-01".into(),
                    fim_validade: "2005-12-31".into(),
                },
                info_cadastro: TInfoCadasro {
                    class_trib: 1,
                    ind_coop: 0,
                    ind_constr: 0,
                    ind_des_folha: 0,
                    ind_opc_cp: 0,
                    ind_port: 0,
                    ind_opt_reg_eletron: 0,
                    cnpj_efr: TipoCNPJ::CNPJ("12345678000195".into()),
                    dt_trans_1096: hoje.into(),
                    ind_tribo_folha_pis_pasep: 0,
                    dados_isencao: DadosInsencao {
                        ide_min_lei: 0,
                        nr_certif: 0,
                        dt_emis_certif: data.clone(),
                        dt_venc_certif: data.clone(),
                        nr_prot_renov: "12345".into(),
                        dt_prot_renov: data.clone(),
                        dt_dou: data.clone(),
                        pag_dou: 0,
                    },
                    info_org_internacional: InfoOrgInternacional {
                        ind_acordo_isen_multa: "12345".into(),
                    },
                },
                nova_validade: TIdePeriodo {
                    ini_validade: "2005-01-01".into(),
                    fim_validade: "2005-12-31".into(),
                },
            }),
        },
    };
    let xml = quick_xml::se::to_string(&evento).unwrap();
    // assert!(xml.contains("<evento>"));
    println!("{xml}");
}
