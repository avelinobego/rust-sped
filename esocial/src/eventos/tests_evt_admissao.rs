#[test]
fn test_info_empregador() {
    use crate::eventos::evt_s2200::TransfDom;
    use crate::eventos::{ESocial, evt_s2200::InfoRegimeTrab};
    use crate::tipos::OrigemEvento;

    let hoje = chrono::Local::now().naive_local();
    let data = crate::tipos::Data(hoje.into());

    let ide_evento = crate::tipos::IdentEvento {
        tp_amb: crate::tipos::Ambiente::Testes,
        proc_emi: OrigemEvento::AplicativoGovernamentalWebGeral,
        ver_proc: 1,
    };

    let trabalhador = crate::eventos::evt_s2200::TipoTrabalhador {
        cpf_trab: "16944301890".into(),
        nm_trab: "Avelino de Almeida Bego".into(),
        sexo: crate::tipos::EnumSexo::MASCULINO,
        raca_cor: crate::tipos::EnumRacaCor::BRANCA,
        est_civ: Some(crate::tipos::EnumEstadoCivil::Casado),
        grau_instr: crate::tipos::GrauInstrucao::EducacaoSuperiorCompleta,
        nm_soc: Some("Avelino Bego".into()),
        nascimento: crate::eventos::evt_s2200::Nascimento {
            dt_nascto: data.clone(),
            pais_nac: 0,
            pais_nascto: 0,
        },
        endereco: Some(
            crate::eventos::evt_s2200::EndExterior {
                bairro: Some("Brookling".into()),
                cod_postal: Some("94495250".into()),
                complemento: Some("street".into()),
                dsc_lograd: "Park Avenue".into(),
                nm_cid: "New York".into(),
                nr_lograd: "10".into(),
                pais_resid: "USA".into(),
            }
            .into(),
        ),
        // trab_imig: Some(crate::eventos::evt_admissao::CondicaoImigracao::TratadoAmizade.into()),
        trab_imig: Some(crate::eventos::evt_s2200::TempoResidencia::Indeterminado.into()),
        info_def: Some(
            crate::eventos::evt_s2200::InfoDefComposer::default()
                .auditiva()
                .cota()
                .fisica()
                .observacao("Testes".into()),
        ),
        dependente: Some(vec![
            crate::eventos::evt_s2200::Dependente {
                tp_dep: Some("01".into()),
                cpf_dep: Some("16944301890".into()),
                dt_nascto: data.clone(),
                dep_irrf: crate::tipos::TipoSimNao::SIM,
                nm_dep: "Avelino de Almeida Bego".into(),
                dep_sf: crate::tipos::TipoSimNao::SIM,
                descr_dep: Some("Esta é uma descrição de testes".into()),
                inc_trab: Some(crate::tipos::TipoSimNao::SIM),
                sexo_dep: Some(crate::tipos::EnumSexo::MASCULINO),
            },
            crate::eventos::evt_s2200::Dependente {
                tp_dep: Some("02".into()),
                cpf_dep: Some("00184166080".into()),
                dt_nascto: data.clone(),
                dep_irrf: crate::tipos::TipoSimNao::SIM,
                nm_dep: "Fabiana Fagundes Bego".into(),
                dep_sf: crate::tipos::TipoSimNao::SIM,
                descr_dep: Some("Esta é uma outra descrição de testes".into()),
                inc_trab: Some(crate::tipos::TipoSimNao::SIM),
                sexo_dep: Some(crate::tipos::EnumSexo::FEMININO),
            },
        ]),
        contato: Some("51999999999".into()),
    };

    let evt_adm = crate::eventos::EvtAdimissao {
        id: crate::tipos::Id("13378331000101".into(), hoje, 1),
        ide_evento,
        trabalhador,
        inscricao: "13378331000101".into(),
        vinculo: Some(crate::eventos::evt_s2200::Vinculo {
            matricula: "123456".into(),
            tp_reg_trab: crate::tipos::RegimeTrabalhista::CLT,
            tp_reg_prev: crate::tipos::RegimePrevidenciario::RegimeGeral,
            cad_ini: crate::tipos::TipoSimNao::SIM,
            info_regime_trab: InfoRegimeTrab {
                info_celetista: Some(crate::eventos::evt_s2200::InfoCeletista {
                    dt_adm: hoje.date(),
                    tp_admissao: crate::tipos::TipoAdmissao::Admissao,
                    ind_admissao: crate::tipos::IndicativoAdmissao::Normal,
                    nr_proc_trab: Some("123456789".into()),
                    tp_reg_jor: crate::tipos::RegimeJornada::Teletrabalho,
                    nat_atividade: crate::tipos::NaturezaAtividade::TrabalhoUrbano,
                    dt_base: Some(crate::tipos::DataMes::Dezembro),
                    cnpj_sind_categ_prof: "11028076000141".into(),
                    mat_anot_jud: Some("123456789".into()),
                    fgts: Some(crate::eventos::evt_s2200::Fgts {
                        dt_opc_fgts: hoje.into(),
                    }),
                    trab_temporario: Some(crate::eventos::evt_s2200::TrabTemporario {
                        hip_leg:
                            crate::tipos::HipoteseContratacaoTemporario::SubstituicaoTransitoria,
                        just_contr: "Justificativa de teste".into(),
                        ide_estab_vinc: "13378331000101".into(),
                        ide_trab_substituido: None,
                    }),
                    aprend: Some(crate::eventos::evt_s2200::TAprend {
                        ind_aprend: 1024,
                        cnpj_ent_qual: "07440871000175".into(),
                        cnpj_prat: "07440871000175".into(),
                        identificador: "13378331000101".into()
                    }),
                }),
                info_estatutario: Some(crate::eventos::evt_s2200::InfoEstatutario {
                    tp_prov: 1024,
                    dt_exercicio: data.clone(),
                    tp_plan_rp: Some(crate::tipos::TipoPlanoRP::MantidoTesouro),
                    ind_teto_rgps: Some(true.into()),
                    ind_abono_perm: Some(false.into()),
                    dt_ini_abono: Some(data.clone()),
                }),
            },
            info_contrato: crate::eventos::evt_s2200::InfoContrato {
                nm_cargo: Some("Cargo de Teste".into()),
                cbo_cargo: Some("123456".into()),
                dt_ingr_cargo: Some(data.clone()),
                nm_funcao: Some("Função de Teste".into()),
                cbo_funcao: Some("123456".into()),
                acum_cargo: Some(false.into()),
                cod_categ: 123.into(),
                remuneracao: Some(crate::eventos::evt_s2200::Remuneracao {
                    vr_sal_fx: "1000.00".into(),
                    und_sal_fixo: 1,
                    dsc_sal_var: Some("Salário Variável de Teste".into()),
                }),
                duracao: Some(crate::eventos::evt_s2200::Duracao {
                    tp_contr: 1234,
                    dt_term: Some(data.clone()),
                    clau_assec: Some(true.into()),
                    obj_det: Some("Objeto Detalhado de Teste".into()),
                }),
                local_trabalho: Some(crate::eventos::evt_s2200::LocalTrabalho {
                    local_trab_geral: Some("Local de Trabalho Geral".into()),
                    local_temp_dom: Some(crate::eventos::evt_s2200::EndBrasil {
                        tp_lograd: Some("Rua".into()),
                        dsc_lograd: "Criciumal".into(),
                        nr_lograd: "10".into(),
                        complemento: Some("casa".into()),
                        bairro: Some("Cecília".into()),
                        cep: "94495250".into(),
                        cod_munic: 45,
                        uf: crate::tipos::EnumUf::RS,
                    }),
                }),
                hor_contratual: Some("99.99".into()),
                alvara_judicial: Some("123456789".into()),
                observacoes: Some(vec![
                    crate::eventos::evt_s2200::ObservacaoContrato {
                        observacao: "Observação 1".into(),
                    },
                    crate::eventos::evt_s2200::ObservacaoContrato {
                        observacao: "Observação 2".into(),
                    },
                    crate::eventos::evt_s2200::ObservacaoContrato {
                        observacao: "Observação 3".into(),
                    },
                ]),
                trei_cap: Some(vec!["001".into(), "002".into()]),
            },
            sucessao_vinc: Some(crate::eventos::evt_s2200::SucessaoVinc {
                matric_ant: Some("123456".into()),
                dt_transf: data.clone(),
                observacao: Some("Observação de Sucessão de Vínculo".into()),
                identificador: "13378331000101".into(),
            }),
            transf_dom: Some(TransfDom {
                cpf_substituido: "50156484056".into(),
                dt_transf: data.clone(),
                matric_ant: Some("Teste matricula".into()),
            }),
            mudanca_cpf: Some(crate::eventos::evt_s2200::MudancaCpf {
                cpf_ant: "08998225093".into(),
                matric_ant: "1234567890".into(),
                dt_alt_cpf: data.clone(),
                observacao: Some("Mudança de CPF de Teste".into()),
            }),
            afastamento: Some(crate::eventos::evt_s2200::Afastamento {
                dt_ini_afast: data.clone(),
                cod_mot_afast: 1,
            }),
            desligamento: Some(data.clone().into()),
            cessao: Some(data.clone().into()),
        }),
    };

    let root: ESocial = (evt_adm.into(), "xpto").into();

    let result = quick_xml::se::to_string(&root).unwrap();
    println!("{result}")
}

#[test]
fn test_deserializer_endreco() {
    let xml = r#"<brasil><tpLograd>Rua</tpLograd><dscLograd>Criciumal</dscLograd><nrLograd>10</nrLograd><complemento>casa</complemento><bairro>Ce</bairro><cep>94495250</cep><codMunic>45</codMunic><uf>RS</uf></brasil>"#;
    let des = quick_xml::de::from_str::<crate::eventos::evt_s2200::EndBrasil>(xml);
    dbg!(des.unwrap());
}

#[test]
fn test_evento_admissao() {
    let xml = r#"<eSocial xmlns="xpto"><evtAdmissao Id="ID1133783310001012025080708481200001"><ideEvento><tpAmb>8</tpAmb><procEmi>3</procEmi><verProc>1</verProc></ideEvento><inscricao><tpInsc>1</tpInsc><nrInsc>13378331000101</nrInsc></inscricao><trabalhador><cpfTrab>16944301890</cpfTrab><nmTrab>Avelino de Almeida Bego</nmTrab><sexo>M</sexo><racaCor>1</racaCor><estCiv>2</estCiv><grauInstr>08</grauInstr><nmSoc>Avelino Bego</nmSoc><nascimento><dtNascto>2025-08-07</dtNascto><paisNascto>0</paisNascto><paisNac>0</paisNac></nascimento><endereco><exterior><paisResid>USA</paisResid><dscLograd>Park Avenue</dscLograd><nrLograd>10</nrLograd><complemento>street</complemento><bairro>Brookling</bairro><nmCid>New York</nmCid><codPostal>94495250</codPostal></exterior></endereco><trabImig><tmpResid>1</tmpResid></trabImig><infoDef><defAuditiva>S</defAuditiva><infoCota>S</infoCota><defFisica>S</defFisica><observacao>Testes</observacao></infoDef><dependente><tpDep>01</tpDep><nmDep>Avelino de Almeida Bego</nmDep><dtNascto>2025-08-07</dtNascto><cpfDep>16944301890</cpfDep><sexoDep>M</sexoDep><depIRRF>S</depIRRF><depSF>S</depSF><incTrab>S</incTrab><descrDep>Esta é uma descrição de testes</descrDep></dependente><dependente><tpDep>02</tpDep><nmDep>Fabiana Fagundes Bego</nmDep><dtNascto>2025-08-07</dtNascto><cpfDep>00184166080</cpfDep><sexoDep>F</sexoDep><depIRRF>S</depIRRF><depSF>S</depSF><incTrab>S</incTrab><descrDep>Esta é uma outra descrição de testes</descrDep></dependente><contato>51999999999</contato></trabalhador><vinculo><matricula>123456</matricula><tpRegTrab>1</tpRegTrab><tpRegPrev>1</tpRegPrev><cadIni>S</cadIni><infoRegimeTrab><infoCeletista><dtAdm>2025-08-07</dtAdm><tpAdmissao>1</tpAdmissao><indAdmissao>1</indAdmissao><nrProcTrab>123456789</nrProcTrab><tpRegJor>4</tpRegJor><natAtividade>1</natAtividade><dtBase>12</dtBase><cnpjSindCategProf>11028076000141</cnpjSindCategProf><matAnotJud>123456789</matAnotJud><fgts><dtOpcFgts>2025-08-07</dtOpcFgts></fgts><trabTemporario><hipLeg>1</hipLeg><justContr>Justificativa de teste</justContr><ideEstabVinc><tpInsc>1</tpInsc><nrInsc>13378331000101</nrInsc></ideEstabVinc></trabTemporario><aprend><indAprend>1024</indAprend><identificador><tpInsc>1</tpInsc><nrInsc>13378331000101</nrInsc></identificador><cnpjEntQual>07440871000175</cnpjEntQual><cnpjPrat>07440871000175</cnpjPrat></aprend></infoCeletista><infoEstatutario><tpProv>1024</tpProv><dtExercicio>2025-08-07</dtExercicio><tpPlanRp>3</tpPlanRp><indTetoRgps>S</indTetoRgps><indAbonoPerm>N</indAbonoPerm><dtIniAbono>2025-08-07</dtIniAbono></infoEstatutario></infoRegimeTrab><infoContrato><nmCargo>Cargo de Teste</nmCargo><cboCargo>123456</cboCargo><dtIngrCargo>2025-08-07</dtIngrCargo><nmFuncao>Função de Teste</nmFuncao><cboFuncao>123456</cboFuncao><acumCargo>N</acumCargo><codCateg>123</codCateg><remuneracao><vrSalFx>1000.00</vrSalFx><undSalFixo>1</undSalFixo><dscSalVar>Salário Variável de Teste</dscSalVar></remuneracao><duracao><tpContr>1234</tpContr><dtTerm>2025-08-07</dtTerm><clauAssec>S</clauAssec><objDet>Objeto Detalhado de Teste</objDet></duracao><localTrabalho><localTrabGeral>Local de Trabalho Geral</localTrabGeral><localTempDom><tpLograd>Rua</tpLograd><dscLograd>Criciumal</dscLograd><nrLograd>10</nrLograd><complemento>casa</complemento><bairro>Cecília</bairro><cep>94495250</cep><codMunic>45</codMunic><uf>RS</uf></localTempDom></localTrabalho><horContratual>99.99</horContratual><alvaraJudicial>123456789</alvaraJudicial><observacoes><observacao>Observação 1</observacao></observacoes><observacoes><observacao>Observação 2</observacao></observacoes><observacoes><observacao>Observação 3</observacao></observacoes><treiCap>001</treiCap><treiCap>002</treiCap></infoContrato><sucessaoVinc><identificador><tpInsc>1</tpInsc><nrInsc>13378331000101</nrInsc></identificador><matricAnt>123456</matricAnt><dtTransf>2025-08-07</dtTransf><observacao>Observação de Sucessão de Vínculo</observacao></sucessaoVinc><transfDom><cpfSubstituido>50156484056</cpfSubstituido><matricAnt>Teste matricula</matricAnt><dtTransf>2025-08-07</dtTransf></transfDom><mudancaCpf><cpfAnt>08998225093</cpfAnt><matricAnt>1234567890</matricAnt><dtAltCpf>2025-08-07</dtAltCpf><observacao>Mudança de CPF de Teste</observacao></mudancaCpf><afastamento><dtIniAfast>2025-08-07</dtIniAfast><codMotAfast>1</codMotAfast></afastamento><desligamento><dtDeslig>2025-08-07</dtDeslig></desligamento><cessao><dtIniCessao>2025-08-07</dtIniCessao></cessao></vinculo></evtAdmissao></eSocial>"#;
    let result = quick_xml::de::from_str::<crate::eventos::ESocial>(xml);
    dbg!(result.unwrap());
}
