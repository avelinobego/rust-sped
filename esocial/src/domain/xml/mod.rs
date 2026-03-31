#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideevento {
    // #[serde(rename = "indRetif")]
    // pub ind_retif: i64,
    // #[serde(rename = "nrRecibo")]
    // pub nr_recibo: Option<String>,
    #[serde(rename = "tpAmb")]
    pub tp_amb: i64,
    #[serde(rename = "procEmi")]
    pub proc_emi: i64,
    #[serde(rename = "verProc")]
    pub ver_proc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideempregador {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtinfoempregador {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "@Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "eSocial")]
pub struct S1000 {
    #[serde(rename = "@xmnls")]
    pub xmnls: String,
    #[serde(rename = "evtInfoEmpregador")]
    pub evt_info_empregador: Evtinfoempregador,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1005 {
    #[serde(rename = "evtTabEstab")]
    pub evt_tab_estab: Evtinfoempregador,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1010 {
    #[serde(rename = "evtTabRubrica")]
    pub evt_tab_rubrica: Evtinfoempregador,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1020 {
    #[serde(rename = "evtTabLotacao")]
    pub evt_tab_lotacao: Evtinfoempregador,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1070 {
    #[serde(rename = "evtTabProcesso")]
    pub evt_tab_processo: Evtinfoempregador,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Remunoutrempr {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "vlrRemunOE")]
    pub vlr_remun_oe: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infomv {
    #[serde(rename = "remunOutrEmpr")]
    pub remun_outr_empr: Vec<Remunoutrempr>,
    #[serde(rename = "indMV")]
    pub ind_mv: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sucessaovinc {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "matricAnt")]
    pub matric_ant: Option<String>,
    #[serde(rename = "dtAdm")]
    pub dt_adm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocomplem {
    #[serde(rename = "sucessaoVinc")]
    pub sucessao_vinc: Option<Sucessaovinc>,
    #[serde(rename = "nmTrab")]
    pub nm_trab: String,
    #[serde(rename = "dtNascto")]
    pub dt_nascto: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Procjudtrab {
    #[serde(rename = "nrProcJud")]
    pub nr_proc_jud: String,
    #[serde(rename = "codSusp")]
    pub cod_susp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infointerm {
    #[serde(rename = "dia")]
    pub dia: i64,
    #[serde(rename = "hrsTrab")]
    pub hrs_trab: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idetrabalhador {
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Despprocjud {
    #[serde(rename = "vlrDespCustas")]
    pub vlr_desp_custas: i64,
    #[serde(rename = "vlrDespAdvogados")]
    pub vlr_desp_advogados: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideadv {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "vlrAdv")]
    pub vlr_adv: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforra {
    #[serde(rename = "despProcJud")]
    pub desp_proc_jud: Option<Despprocjud>,
    #[serde(rename = "ideAdv")]
    pub ide_adv: Vec<Ideadv>,
    #[serde(rename = "tpProcRRA")]
    pub tp_proc_rra: i64,
    #[serde(rename = "nrProcRRA")]
    pub nr_proc_rra: Option<String>,
    #[serde(rename = "descRRA")]
    pub desc_rra: String,
    #[serde(rename = "qtdMesesRRA")]
    pub qtd_meses_rra: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Descfolha {
    #[serde(rename = "tpDesc")]
    pub tp_desc: i64,
    #[serde(rename = "instFinanc")]
    pub inst_financ: String,
    #[serde(rename = "nrDoc")]
    pub nr_doc: String,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Itensremun {
    #[serde(rename = "descFolha")]
    pub desc_folha: Option<Descfolha>,
    #[serde(rename = "codRubr")]
    pub cod_rubr: String,
    #[serde(rename = "ideTabRubr")]
    pub ide_tab_rubr: String,
    #[serde(rename = "qtdRubr")]
    pub qtd_rubr: Option<i64>,
    #[serde(rename = "fatorRubr")]
    pub fator_rubr: Option<i64>,
    #[serde(rename = "vrRubr")]
    pub vr_rubr: i64,
    #[serde(rename = "indApurIR")]
    pub ind_apur_ir: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoagnocivo {
    #[serde(rename = "grauExp")]
    pub grau_exp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Remunperapur {
    #[serde(rename = "itensRemun")]
    pub itens_remun: Vec<Itensremun>,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideestablot {
    #[serde(rename = "infoCategIncid")]
    pub info_categ_incid: Vec<Infocategincid>,
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "codLotacao")]
    pub cod_lotacao: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoperapur {
    #[serde(rename = "ideEstabLot")]
    pub ide_estab_lot: Vec<Ideestablot>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideperiodo {
    #[serde(rename = "baseCalculo")]
    pub base_calculo: Option<Basecalculo>,
    #[serde(rename = "infoFGTS")]
    pub info_fgts: Option<Infofgts>,
    #[serde(rename = "baseMudCateg")]
    pub base_mud_categ: Option<Basemudcateg>,
    #[serde(rename = "infoInterm")]
    pub info_interm: Vec<Infointerm>,
    #[serde(rename = "perRef")]
    pub per_ref: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideadc {
    #[serde(rename = "dtAcConv")]
    pub dt_ac_conv: Option<String>,
    #[serde(rename = "tpAcConv")]
    pub tp_ac_conv: String,
    #[serde(rename = "dsc")]
    pub dsc: String,
    #[serde(rename = "remunSuc")]
    pub remun_suc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoperant {
    #[serde(rename = "ideADC")]
    pub ide_adc: Vec<Ideadc>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocomplcont {
    #[serde(rename = "codCBO")]
    pub cod_cbo: String,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: Option<i64>,
    #[serde(rename = "qtdDiasTrab")]
    pub qtd_dias_trab: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dmdev {
    #[serde(rename = "infoIR")]
    pub info_ir: Vec<Infoir>,
    #[serde(rename = "totApurMen")]
    pub tot_apur_men: Vec<Totapurmen>,
    #[serde(rename = "totApurDia")]
    pub tot_apur_dia: Vec<Totapurdia>,
    #[serde(rename = "infoRRA")]
    pub info_rra: Option<Inforra>,
    #[serde(rename = "infoPgtoExt")]
    pub info_pgto_ext: Option<Infopgtoext>,
    #[serde(rename = "perRef")]
    pub per_ref: String,
    #[serde(rename = "ideDmDev")]
    pub ide_dm_dev: String,
    #[serde(rename = "tpPgto")]
    pub tp_pgto: i64,
    #[serde(rename = "dtPgto")]
    pub dt_pgto: String,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtremun {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabalhador")]
    pub ide_trabalhador: Idetrabalhador,
    #[serde(rename = "dmDev")]
    pub dm_dev: Vec<Dmdev>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1200 {
    #[serde(rename = "evtRemun")]
    pub evt_remun: Evtremun,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideestab {
    #[serde(rename = "basePerRef")]
    pub base_per_ref: Vec<Baseperref>,
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1202 {
    #[serde(rename = "evtRmnRPPS")]
    pub evt_rmn_rpps: Evtremun,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idebenef {
    #[serde(rename = "cpfBenef")]
    pub cpf_benef: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtbenprrp {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideBenef")]
    pub ide_benef: Idebenef,
    #[serde(rename = "dmDev")]
    pub dm_dev: Vec<Dmdev>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1207 {
    #[serde(rename = "evtBenPrRP")]
    pub evt_ben_pr_rp: Evtbenprrp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Endext {
    #[serde(rename = "endDscLograd")]
    pub end_dsc_lograd: Option<String>,
    #[serde(rename = "endNrLograd")]
    pub end_nr_lograd: Option<String>,
    #[serde(rename = "endComplem")]
    pub end_complem: Option<String>,
    #[serde(rename = "endBairro")]
    pub end_bairro: Option<String>,
    #[serde(rename = "endCidade")]
    pub end_cidade: Option<String>,
    #[serde(rename = "endEstado")]
    pub end_estado: Option<String>,
    #[serde(rename = "endCodPostal")]
    pub end_cod_postal: Option<String>,
    #[serde(rename = "telef")]
    pub telef: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infopgtoext {
    #[serde(rename = "endExt")]
    pub end_ext: Option<Endext>,
    #[serde(rename = "paisResidExt")]
    pub pais_resid_ext: String,
    #[serde(rename = "indNIF")]
    pub ind_nif: i64,
    #[serde(rename = "nifBenef")]
    pub nif_benef: Option<String>,
    #[serde(rename = "frmTribut")]
    pub frm_tribut: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infopgto {
    #[serde(rename = "infoPgtoExt")]
    pub info_pgto_ext: Option<Infopgtoext>,
    #[serde(rename = "dtPgto")]
    pub dt_pgto: String,
    #[serde(rename = "tpPgto")]
    pub tp_pgto: i64,
    #[serde(rename = "perRef")]
    pub per_ref: String,
    #[serde(rename = "ideDmDev")]
    pub ide_dm_dev: String,
    #[serde(rename = "vrLiq")]
    pub vr_liq: i64,
    #[serde(rename = "paisResidExt")]
    pub pais_resid_ext: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Perant {
    #[serde(rename = "perRefAjuste")]
    pub per_ref_ajuste: String,
    #[serde(rename = "nrRec1210Orig")]
    pub nr_rec1210_orig: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infodep {
    #[serde(rename = "cpfDep")]
    pub cpf_dep: String,
    #[serde(rename = "dtNascto")]
    pub dt_nascto: Option<String>,
    #[serde(rename = "nome")]
    pub nome: Option<String>,
    #[serde(rename = "depIRRF")]
    pub dep_irrf: Option<String>,
    #[serde(rename = "tpDep")]
    pub tp_dep: Option<String>,
    #[serde(rename = "descrDep")]
    pub descr_dep: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deddepen {
    #[serde(rename = "tpRend")]
    pub tp_rend: i64,
    #[serde(rename = "cpfDep")]
    pub cpf_dep: String,
    #[serde(rename = "vlrDedDep")]
    pub vlr_ded_dep: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Penalim {
    #[serde(rename = "tpRend")]
    pub tp_rend: i64,
    #[serde(rename = "cpfDep")]
    pub cpf_dep: String,
    #[serde(rename = "vlrDedPenAlim")]
    pub vlr_ded_pen_alim: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Previdcompl {
    #[serde(rename = "tpPrev")]
    pub tp_prev: i64,
    #[serde(rename = "cnpjEntidPC")]
    pub cnpj_entid_pc: String,
    #[serde(rename = "vlrDedPC")]
    pub vlr_ded_pc: Option<i64>,
    #[serde(rename = "vlrDedPC13")]
    pub vlr_ded_pc13: Option<i64>,
    #[serde(rename = "vlrPatrocFunp")]
    pub vlr_patroc_funp: Option<i64>,
    #[serde(rename = "vlrPatrocFunp13")]
    pub vlr_patroc_funp13: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Benefpen {
    #[serde(rename = "cpfDep")]
    pub cpf_dep: String,
    #[serde(rename = "vlrDepenSusp")]
    pub vlr_depen_susp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dedsusp {
    #[serde(rename = "benefPen")]
    pub benef_pen: Vec<Benefpen>,
    #[serde(rename = "indTpDeducao")]
    pub ind_tp_deducao: i64,
    #[serde(rename = "vlrDedSusp")]
    pub vlr_ded_susp: Option<i64>,
    #[serde(rename = "cnpjEntidPC")]
    pub cnpj_entid_pc: Option<String>,
    #[serde(rename = "vlrPatrocFunp")]
    pub vlr_patroc_funp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infovalores {
    #[serde(rename = "dedSusp")]
    pub ded_susp: Vec<Dedsusp>,
    #[serde(rename = "indApuracao")]
    pub ind_apuracao: i64,
    #[serde(rename = "vlrNRetido")]
    pub vlr_n_retido: Option<i64>,
    #[serde(rename = "vlrDepJud")]
    pub vlr_dep_jud: Option<i64>,
    #[serde(rename = "vlrCmpAnoCal")]
    pub vlr_cmp_ano_cal: Option<i64>,
    #[serde(rename = "vlrCmpAnoAnt")]
    pub vlr_cmp_ano_ant: Option<i64>,
    #[serde(rename = "vlrRendSusp")]
    pub vlr_rend_susp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoprocret {
    #[serde(rename = "infoValores")]
    pub info_valores: Vec<Infovalores>,
    #[serde(rename = "tpProcRet")]
    pub tp_proc_ret: i64,
    #[serde(rename = "nrProcRet")]
    pub nr_proc_ret: String,
    #[serde(rename = "codSusp")]
    pub cod_susp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoircr {
    #[serde(rename = "dedDepen")]
    pub ded_depen: Vec<Deddepen>,
    #[serde(rename = "penAlim")]
    pub pen_alim: Vec<Penalim>,
    #[serde(rename = "previdCompl")]
    pub previd_compl: Vec<Previdcompl>,
    #[serde(rename = "infoProcRet")]
    pub info_proc_ret: Vec<Infoprocret>,
    #[serde(rename = "tpCR")]
    pub tp_cr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infodepsau {
    #[serde(rename = "cpfDep")]
    pub cpf_dep: String,
    #[serde(rename = "vlrSaudeDep")]
    pub vlr_saude_dep: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plansaude {
    #[serde(rename = "infoDepSau")]
    pub info_dep_sau: Vec<Infodepsau>,
    #[serde(rename = "cnpjOper")]
    pub cnpj_oper: String,
    #[serde(rename = "regANS")]
    pub reg_ans: Option<String>,
    #[serde(rename = "vlrSaudeTit")]
    pub vlr_saude_tit: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detreembtit {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "vlrReemb")]
    pub vlr_reemb: Option<i64>,
    #[serde(rename = "vlrReembAnt")]
    pub vlr_reemb_ant: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforeembmed {
    #[serde(rename = "detReembTit")]
    pub det_reemb_tit: Vec<Detreembtit>,
    #[serde(rename = "infoReembDep")]
    pub info_reemb_dep: Vec<Idebenef>,
    #[serde(rename = "indOrgReemb")]
    pub ind_org_reemb: i64,
    #[serde(rename = "cnpjOper")]
    pub cnpj_oper: Option<String>,
    #[serde(rename = "regANS")]
    pub reg_ans: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoircomplem {
    #[serde(rename = "perAnt")]
    pub per_ant: Option<Perant>,
    #[serde(rename = "ideDep")]
    pub ide_dep: Vec<Infodep>,
    #[serde(rename = "infoIRCR")]
    pub info_ircr: Vec<Infoircr>,
    #[serde(rename = "planSaude")]
    pub plan_saude: Vec<Plansaude>,
    #[serde(rename = "infoReembMed")]
    pub info_reemb_med: Vec<Inforeembmed>,
    #[serde(rename = "dtLaudo")]
    pub dt_laudo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtpgtos {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideBenef")]
    pub ide_benef: Idebenef,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1210 {
    #[serde(rename = "evtPgtos")]
    pub evt_pgtos: Evtpgtos,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nfs {
    #[serde(rename = "serie")]
    pub serie: Option<String>,
    #[serde(rename = "nrDocto")]
    pub nr_docto: String,
    #[serde(rename = "dtEmisNF")]
    pub dt_emis_nf: String,
    #[serde(rename = "vlrBruto")]
    pub vlr_bruto: i64,
    #[serde(rename = "vrCPDescPR")]
    pub vr_cp_desc_pr: i64,
    #[serde(rename = "vrRatDescPR")]
    pub vr_rat_desc_pr: i64,
    #[serde(rename = "vrSenarDesc")]
    pub vr_senar_desc: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideadquir {
    #[serde(rename = "nfs")]
    pub nfs: Vec<Nfs>,
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "vrComerc")]
    pub vr_comerc: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoprocjud {
    #[serde(rename = "tpProc")]
    pub tp_proc: i64,
    #[serde(rename = "nrProc")]
    pub nr_proc: String,
    #[serde(rename = "codSusp")]
    pub cod_susp: i64,
    #[serde(rename = "vrCPSusp")]
    pub vr_cp_susp: Option<i64>,
    #[serde(rename = "vrRatSusp")]
    pub vr_rat_susp: Option<i64>,
    #[serde(rename = "vrSenarSusp")]
    pub vr_senar_susp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tpcomerc {
    #[serde(rename = "ideAdquir")]
    pub ide_adquir: Vec<Ideadquir>,
    #[serde(rename = "infoProcJud")]
    pub info_proc_jud: Vec<Infoprocjud>,
    #[serde(rename = "indComerc")]
    pub ind_comerc: i64,
    #[serde(rename = "vrTotCom")]
    pub vr_tot_com: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideestabel {
    #[serde(rename = "tpComerc")]
    pub tp_comerc: Vec<Tpcomerc>,
    #[serde(rename = "nrInscEstabRural")]
    pub nr_insc_estab_rural: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocomprod {
    #[serde(rename = "ideEstabel")]
    pub ide_estabel: Ideestabel,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcomprod {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoComProd")]
    pub info_com_prod: Infocomprod,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1260 {
    #[serde(rename = "evtComProd")]
    pub evt_com_prod: Evtcomprod,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Remunavnp {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "codLotacao")]
    pub cod_lotacao: String,
    #[serde(rename = "vrBcCp00")]
    pub vr_bc_cp00: i64,
    #[serde(rename = "vrBcCp15")]
    pub vr_bc_cp15: i64,
    #[serde(rename = "vrBcCp20")]
    pub vr_bc_cp20: i64,
    #[serde(rename = "vrBcCp25")]
    pub vr_bc_cp25: i64,
    #[serde(rename = "vrBcCp13")]
    pub vr_bc_cp13: i64,
    #[serde(rename = "vrBcFgts")]
    pub vr_bc_fgts: i64,
    #[serde(rename = "vrDescCP")]
    pub vr_desc_cp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcontratavnp {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "remunAvNP")]
    pub remun_av_np: Vec<Remunavnp>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1270 {
    #[serde(rename = "evtContratAvNP")]
    pub evt_contrat_av_np: Evtcontratavnp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infosubstpatr {
    #[serde(rename = "indSubstPatr")]
    pub ind_subst_patr: i64,
    #[serde(rename = "percRedContrib")]
    pub perc_red_contrib: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infosubstpatropport {
    #[serde(rename = "cnpjOpPortuario")]
    pub cnpj_op_portuario: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoativconcom {
    #[serde(rename = "fatorMes")]
    pub fator_mes: i64,
    #[serde(rename = "fator13")]
    pub fator13: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoperctransf11096 {
    #[serde(rename = "percTransf")]
    pub perc_transf: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtinfocomplper {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoSubstPatr")]
    pub info_subst_patr: Option<Infosubstpatr>,
    #[serde(rename = "infoSubstPatrOpPort")]
    pub info_subst_patr_op_port: Vec<Infosubstpatropport>,
    #[serde(rename = "infoAtivConcom")]
    pub info_ativ_concom: Option<Infoativconcom>,
    #[serde(rename = "infoPercTransf11096")]
    pub info_perc_transf11096: Option<Infoperctransf11096>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1280 {
    #[serde(rename = "evtInfoComplPer")]
    pub evt_info_compl_per: Evtinfocomplper,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1298 {
    #[serde(rename = "evtReabreEvPer")]
    pub evt_reabre_ev_per: Evtinfoempregador,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infofech {
    #[serde(rename = "evtRemun")]
    pub evt_remun: String,
    #[serde(rename = "evtPgtos")]
    pub evt_pgtos: String,
    #[serde(rename = "evtComProd")]
    pub evt_com_prod: String,
    #[serde(rename = "evtContratAvNP")]
    pub evt_contrat_av_np: String,
    #[serde(rename = "evtInfoComplPer")]
    pub evt_info_compl_per: String,
    #[serde(rename = "indExcApur1250")]
    pub ind_exc_apur1250: Option<String>,
    #[serde(rename = "transDCTFWeb")]
    pub trans_dctf_web: Option<String>,
    #[serde(rename = "naoValid")]
    pub nao_valid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtfechaevper {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoFech")]
    pub info_fech: Infofech,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S1299 {
    #[serde(rename = "evtFechaEvPer")]
    pub evt_fecha_ev_per: Evtfechaevper,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforegctps {
    #[serde(rename = "CBOCargo")]
    pub cbo_cargo: String,
    #[serde(rename = "vrSalFx")]
    pub vr_sal_fx: i64,
    #[serde(rename = "undSalFixo")]
    pub und_sal_fixo: i64,
    #[serde(rename = "tpContr")]
    pub tp_contr: i64,
    #[serde(rename = "dtTerm")]
    pub dt_term: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforegprelim {
    #[serde(rename = "infoRegCTPS")]
    pub info_reg_ctps: Option<Inforegctps>,
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
    #[serde(rename = "dtNascto")]
    pub dt_nascto: String,
    #[serde(rename = "dtAdm")]
    pub dt_adm: String,
    #[serde(rename = "matricula")]
    pub matricula: String,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtadmprelim {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoRegPrelim")]
    pub info_reg_prelim: Inforegprelim,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2190 {
    #[serde(rename = "evtAdmPrelim")]
    pub evt_adm_prelim: Evtadmprelim,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nascimento {
    #[serde(rename = "dtNascto")]
    pub dt_nascto: String,
    #[serde(rename = "paisNascto")]
    pub pais_nascto: String,
    #[serde(rename = "paisNac")]
    pub pais_nac: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trabimig {
    #[serde(rename = "tmpResid")]
    pub tmp_resid: Option<i64>,
    #[serde(rename = "condIng")]
    pub cond_ing: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infodeficiencia {
    #[serde(rename = "defFisica")]
    pub def_fisica: String,
    #[serde(rename = "defVisual")]
    pub def_visual: String,
    #[serde(rename = "defAuditiva")]
    pub def_auditiva: String,
    #[serde(rename = "defMental")]
    pub def_mental: String,
    #[serde(rename = "defIntelectual")]
    pub def_intelectual: String,
    #[serde(rename = "reabReadap")]
    pub reab_readap: String,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dependente {
    #[serde(rename = "tpDep")]
    pub tp_dep: Option<String>,
    #[serde(rename = "nmDep")]
    pub nm_dep: String,
    #[serde(rename = "dtNascto")]
    pub dt_nascto: String,
    #[serde(rename = "cpfDep")]
    pub cpf_dep: Option<String>,
    #[serde(rename = "sexoDep")]
    pub sexo_dep: String,
    #[serde(rename = "depIRRF")]
    pub dep_irrf: String,
    #[serde(rename = "incFisMen")]
    pub inc_fis_men: String,
    #[serde(rename = "descrDep")]
    pub descr_dep: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contato {
    #[serde(rename = "fonePrinc")]
    pub fone_princ: Option<String>,
    #[serde(rename = "emailPrinc")]
    pub email_princ: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trabalhador {
    #[serde(rename = "nascimento")]
    pub nascimento: Nascimento,
    #[serde(rename = "trabImig")]
    pub trab_imig: Option<Trabimig>,
    #[serde(rename = "infoDeficiencia")]
    pub info_deficiencia: Option<Infodeficiencia>,
    #[serde(rename = "dependente")]
    pub dependente: Vec<Dependente>,
    #[serde(rename = "contato")]
    pub contato: Option<Contato>,
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
    #[serde(rename = "nmTrab")]
    pub nm_trab: String,
    #[serde(rename = "sexo")]
    pub sexo: String,
    #[serde(rename = "racaCor")]
    pub raca_cor: i64,
    #[serde(rename = "estCiv")]
    pub est_civ: Option<i64>,
    #[serde(rename = "grauInstr")]
    pub grau_instr: String,
    #[serde(rename = "nmSoc")]
    pub nm_soc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Remuneracao {
    #[serde(rename = "dtRemun")]
    pub dt_remun: String,
    #[serde(rename = "vrSalFx")]
    pub vr_sal_fx: i64,
    #[serde(rename = "undSalFixo")]
    pub und_sal_fixo: i64,
    #[serde(rename = "dscSalVar")]
    pub dsc_sal_var: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Duracao {
    #[serde(rename = "tpContr")]
    pub tp_contr: i64,
    #[serde(rename = "dtTerm")]
    pub dt_term: Option<String>,
    #[serde(rename = "clauAssec")]
    pub clau_assec: Option<String>,
    #[serde(rename = "objDet")]
    pub obj_det: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Localtrabgeral {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "descComp")]
    pub desc_comp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Localtrabalho {
    #[serde(rename = "localTrabGeral")]
    pub local_trab_geral: Option<Localtrabgeral>,
    #[serde(rename = "localTempDom")]
    pub local_temp_dom: Option<Localtempdom>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Horcontratual {
    #[serde(rename = "qtdHrsSem")]
    pub qtd_hrs_sem: Option<i64>,
    #[serde(rename = "tpJornada")]
    pub tp_jornada: i64,
    #[serde(rename = "tmpParc")]
    pub tmp_parc: i64,
    #[serde(rename = "horNoturno")]
    pub hor_noturno: Option<String>,
    #[serde(rename = "dscJorn")]
    pub dsc_jorn: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alvarajudicial {
    #[serde(rename = "nrProcJud")]
    pub nr_proc_jud: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Observacoes {
    #[serde(rename = "observacao")]
    pub observacao: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Treicap {
    #[serde(rename = "codTreiCap")]
    pub cod_trei_cap: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocontrato {
    #[serde(rename = "remuneracao")]
    pub remuneracao: Option<Remuneracao>,
    #[serde(rename = "duracao")]
    pub duracao: Option<Duracao>,
    #[serde(rename = "localTrabalho")]
    pub local_trabalho: Localtrabalho,
    #[serde(rename = "horContratual")]
    pub hor_contratual: Option<Horcontratual>,
    #[serde(rename = "alvaraJudicial")]
    pub alvara_judicial: Option<Alvarajudicial>,
    #[serde(rename = "observacoes")]
    pub observacoes: Vec<Observacoes>,
    #[serde(rename = "treiCap")]
    pub trei_cap: Vec<Treicap>,
    #[serde(rename = "nmCargo")]
    pub nm_cargo: Option<String>,
    #[serde(rename = "CBOCargo")]
    pub cbo_cargo: Option<String>,
    #[serde(rename = "nmFuncao")]
    pub nm_funcao: Option<String>,
    #[serde(rename = "CBOFuncao")]
    pub cbo_funcao: Option<String>,
    #[serde(rename = "acumCargo")]
    pub acum_cargo: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transfdom {
    #[serde(rename = "cpfSubstituido")]
    pub cpf_substituido: String,
    #[serde(rename = "matricAnt")]
    pub matric_ant: Option<String>,
    #[serde(rename = "dtTransf")]
    pub dt_transf: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mudancacpf {
    #[serde(rename = "cpfAnt")]
    pub cpf_ant: String,
    #[serde(rename = "nrBeneficioAnt")]
    pub nr_beneficio_ant: String,
    #[serde(rename = "dtAltCPF")]
    pub dt_alt_cpf: String,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Afastamento {
    #[serde(rename = "dtIniAfast")]
    pub dt_ini_afast: String,
    #[serde(rename = "codMotAfast")]
    pub cod_mot_afast: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Desligamento {
    #[serde(rename = "mtvDeslig")]
    pub mtv_deslig: String,
    #[serde(rename = "dtDeslig")]
    pub dt_deslig: String,
    #[serde(rename = "dtProjFimAPI")]
    pub dt_proj_fim_api: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cessao {
    #[serde(rename = "dtIniCessao")]
    pub dt_ini_cessao: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vinculo {
    #[serde(rename = "infoRegimeTrab")]
    pub info_regime_trab: Option<Inforegimetrab>,
    #[serde(rename = "infoContrato")]
    pub info_contrato: Infocontrato,
    #[serde(rename = "tpRegPrev")]
    pub tp_reg_prev: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtadmissao {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "trabalhador")]
    pub trabalhador: Trabalhador,
    #[serde(rename = "vinculo")]
    pub vinculo: Vinculo,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2200 {
    #[serde(rename = "evtAdmissao")]
    pub evt_admissao: Evtadmissao,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dadostrabalhador {
    #[serde(rename = "trabImig")]
    pub trab_imig: Option<Trabimig>,
    #[serde(rename = "infoDeficiencia")]
    pub info_deficiencia: Option<Infodeficiencia>,
    #[serde(rename = "dependente")]
    pub dependente: Vec<Dependente>,
    #[serde(rename = "contato")]
    pub contato: Option<Contato>,
    #[serde(rename = "nmTrab")]
    pub nm_trab: String,
    #[serde(rename = "sexo")]
    pub sexo: String,
    #[serde(rename = "racaCor")]
    pub raca_cor: i64,
    #[serde(rename = "estCiv")]
    pub est_civ: Option<i64>,
    #[serde(rename = "grauInstr")]
    pub grau_instr: String,
    #[serde(rename = "nmSoc")]
    pub nm_soc: Option<String>,
    #[serde(rename = "paisNac")]
    pub pais_nac: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Alteracao {
    #[serde(rename = "dadosBenef")]
    pub dados_benef: Dadosbenef,
    #[serde(rename = "dtAlteracao")]
    pub dt_alteracao: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtaltcadastral {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabalhador")]
    pub ide_trabalhador: Idetrabalhador,
    #[serde(rename = "alteracao")]
    pub alteracao: Alteracao,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2205 {
    #[serde(rename = "evtAltCadastral")]
    pub evt_alt_cadastral: Evtaltcadastral,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idevinculo {
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
    #[serde(rename = "matricula")]
    pub matricula: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trabtemporario {
    #[serde(rename = "justProrr")]
    pub just_prorr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aprend {
    #[serde(rename = "indAprend")]
    pub ind_aprend: i64,
    #[serde(rename = "cnpjEntQual")]
    pub cnpj_ent_qual: Option<String>,
    #[serde(rename = "tpInsc")]
    pub tp_insc: Option<i64>,
    #[serde(rename = "nrInsc")]
    pub nr_insc: Option<String>,
    #[serde(rename = "cnpjPrat")]
    pub cnpj_prat: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoceletista {
    #[serde(rename = "trabTemporario")]
    pub trab_temporario: Option<Trabtemporario>,
    #[serde(rename = "aprend")]
    pub aprend: Option<Aprend>,
    #[serde(rename = "tpRegJor")]
    pub tp_reg_jor: i64,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: i64,
    #[serde(rename = "dtBase")]
    pub dt_base: Option<i64>,
    #[serde(rename = "cnpjSindCategProf")]
    pub cnpj_sind_categ_prof: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoestatutario {
    #[serde(rename = "tpPlanRP")]
    pub tp_plan_rp: i64,
    #[serde(rename = "indTetoRGPS")]
    pub ind_teto_rgps: String,
    #[serde(rename = "indAbonoPerm")]
    pub ind_abono_perm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforegimetrab {
    #[serde(rename = "infoCeletista")]
    pub info_celetista: Option<Infoceletista>,
    #[serde(rename = "infoEstatutario")]
    pub info_estatutario: Option<Infoestatutario>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Localtempdom {
    #[serde(rename = "tpLograd")]
    pub tp_lograd: Option<String>,
    #[serde(rename = "dscLograd")]
    pub dsc_lograd: String,
    #[serde(rename = "nrLograd")]
    pub nr_lograd: String,
    #[serde(rename = "complemento")]
    pub complemento: Option<String>,
    #[serde(rename = "bairro")]
    pub bairro: Option<String>,
    #[serde(rename = "cep")]
    pub cep: String,
    #[serde(rename = "codMunic")]
    pub cod_munic: i64,
    #[serde(rename = "uf")]
    pub uf: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Altcontratual {
    #[serde(rename = "vinculo")]
    pub vinculo: Vinculo,
    #[serde(rename = "dtAlteracao")]
    pub dt_alteracao: String,
    #[serde(rename = "dtEf")]
    pub dt_ef: Option<String>,
    #[serde(rename = "dscAlt")]
    pub dsc_alt: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtaltcontratual {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "altContratual")]
    pub alt_contratual: Altcontratual,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2206 {
    #[serde(rename = "evtAltContratual")]
    pub evt_alt_contratual: Evtaltcontratual,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Localacidente {
    #[serde(rename = "ideLocalAcid")]
    pub ide_local_acid: Option<Ideempregador>,
    #[serde(rename = "tpLocal")]
    pub tp_local: i64,
    #[serde(rename = "dscLocal")]
    pub dsc_local: Option<String>,
    #[serde(rename = "tpLograd")]
    pub tp_lograd: Option<String>,
    #[serde(rename = "dscLograd")]
    pub dsc_lograd: String,
    #[serde(rename = "nrLograd")]
    pub nr_lograd: String,
    #[serde(rename = "complemento")]
    pub complemento: Option<String>,
    #[serde(rename = "bairro")]
    pub bairro: Option<String>,
    #[serde(rename = "cep")]
    pub cep: Option<String>,
    #[serde(rename = "codMunic")]
    pub cod_munic: Option<i64>,
    #[serde(rename = "uf")]
    pub uf: Option<String>,
    #[serde(rename = "pais")]
    pub pais: Option<String>,
    #[serde(rename = "codPostal")]
    pub cod_postal: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parteatingida {
    #[serde(rename = "codParteAting")]
    pub cod_parte_ating: i64,
    #[serde(rename = "lateralidade")]
    pub lateralidade: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agentecausador {
    #[serde(rename = "codAgntCausador")]
    pub cod_agnt_causador: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Emitente {
    #[serde(rename = "nmEmit")]
    pub nm_emit: String,
    #[serde(rename = "ideOC")]
    pub ide_oc: i64,
    #[serde(rename = "nrOC")]
    pub nr_oc: String,
    #[serde(rename = "ufOC")]
    pub uf_oc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Atestado {
    #[serde(rename = "emitente")]
    pub emitente: Emitente,
    #[serde(rename = "dtAtendimento")]
    pub dt_atendimento: String,
    #[serde(rename = "hrAtendimento")]
    pub hr_atendimento: String,
    #[serde(rename = "indInternacao")]
    pub ind_internacao: String,
    #[serde(rename = "durTrat")]
    pub dur_trat: i64,
    #[serde(rename = "indAfast")]
    pub ind_afast: String,
    #[serde(rename = "dscLesao")]
    pub dsc_lesao: i64,
    #[serde(rename = "dscCompLesao")]
    pub dsc_comp_lesao: Option<String>,
    #[serde(rename = "diagProvavel")]
    pub diag_provavel: Option<String>,
    #[serde(rename = "codCID")]
    pub cod_cid: String,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Catorigem {
    #[serde(rename = "nrRecCatOrig")]
    pub nr_rec_cat_orig: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cat {
    #[serde(rename = "localAcidente")]
    pub local_acidente: Localacidente,
    #[serde(rename = "parteAtingida")]
    pub parte_atingida: Parteatingida,
    #[serde(rename = "agenteCausador")]
    pub agente_causador: Agentecausador,
    #[serde(rename = "atestado")]
    pub atestado: Atestado,
    #[serde(rename = "catOrigem")]
    pub cat_origem: Option<Catorigem>,
    #[serde(rename = "dtAcid")]
    pub dt_acid: String,
    #[serde(rename = "tpAcid")]
    pub tp_acid: i64,
    #[serde(rename = "hrAcid")]
    pub hr_acid: Option<String>,
    #[serde(rename = "hrsTrabAntesAcid")]
    pub hrs_trab_antes_acid: Option<String>,
    #[serde(rename = "tpCat")]
    pub tp_cat: i64,
    #[serde(rename = "indCatObito")]
    pub ind_cat_obito: String,
    #[serde(rename = "dtObito")]
    pub dt_obito: Option<String>,
    #[serde(rename = "indComunPolicia")]
    pub ind_comun_policia: String,
    #[serde(rename = "codSitGeradora")]
    pub cod_sit_geradora: i64,
    #[serde(rename = "iniciatCAT")]
    pub iniciat_cat: i64,
    #[serde(rename = "obsCAT")]
    pub obs_cat: Option<String>,
    #[serde(rename = "ultDiaTrab")]
    pub ult_dia_trab: Option<String>,
    #[serde(rename = "houveAfast")]
    pub houve_afast: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcat {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "cat")]
    pub cat: Cat,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2210 {
    #[serde(rename = "evtCAT")]
    pub evt_cat: Evtcat,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exame {
    #[serde(rename = "dtExm")]
    pub dt_exm: String,
    #[serde(rename = "procRealizado")]
    pub proc_realizado: i64,
    #[serde(rename = "obsProc")]
    pub obs_proc: Option<String>,
    #[serde(rename = "ordExame")]
    pub ord_exame: Option<i64>,
    #[serde(rename = "indResult")]
    pub ind_result: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Medico {
    #[serde(rename = "nmMed")]
    pub nm_med: String,
    #[serde(rename = "nrCRM")]
    pub nr_crm: Option<String>,
    #[serde(rename = "ufCRM")]
    pub uf_crm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Aso {
    #[serde(rename = "exame")]
    pub exame: Vec<Exame>,
    #[serde(rename = "medico")]
    pub medico: Medico,
    #[serde(rename = "dtAso")]
    pub dt_aso: String,
    #[serde(rename = "resAso")]
    pub res_aso: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Respmonit {
    #[serde(rename = "cpfResp")]
    pub cpf_resp: Option<String>,
    #[serde(rename = "nmResp")]
    pub nm_resp: String,
    #[serde(rename = "nrCRM")]
    pub nr_crm: String,
    #[serde(rename = "ufCRM")]
    pub uf_crm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Exmedocup {
    #[serde(rename = "aso")]
    pub aso: Aso,
    #[serde(rename = "respMonit")]
    pub resp_monit: Option<Respmonit>,
    #[serde(rename = "tpExameOcup")]
    pub tp_exame_ocup: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtmonit {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "exMedOcup")]
    pub ex_med_ocup: Exmedocup,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2220 {
    #[serde(rename = "evtMonit")]
    pub evt_monit: Evtmonit,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Toxicologico {
    #[serde(rename = "dtExame")]
    pub dt_exame: String,
    #[serde(rename = "cnpjLab")]
    pub cnpj_lab: String,
    #[serde(rename = "codSeqExame")]
    pub cod_seq_exame: String,
    #[serde(rename = "nmMed")]
    pub nm_med: String,
    #[serde(rename = "nrCRM")]
    pub nr_crm: Option<String>,
    #[serde(rename = "ufCRM")]
    pub uf_crm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evttoxic {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "toxicologico")]
    pub toxicologico: Toxicologico,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2221 {
    #[serde(rename = "evtToxic")]
    pub evt_toxic: Evttoxic,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Peraquis {
    #[serde(rename = "dtInicio")]
    pub dt_inicio: String,
    #[serde(rename = "dtFim")]
    pub dt_fim: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocessao {
    #[serde(rename = "cnpjCess")]
    pub cnpj_cess: String,
    #[serde(rename = "infOnus")]
    pub inf_onus: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infomandsind {
    #[serde(rename = "cnpjSind")]
    pub cnpj_sind: String,
    #[serde(rename = "infOnusRemun")]
    pub inf_onus_remun: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infomandelet {
    #[serde(rename = "indRemunCargo")]
    pub ind_remun_cargo: Option<String>,
    #[serde(rename = "tpRegPrev")]
    pub tp_reg_prev: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Iniafastamento {
    #[serde(rename = "perAquis")]
    pub per_aquis: Option<Peraquis>,
    #[serde(rename = "infoCessao")]
    pub info_cessao: Option<Infocessao>,
    #[serde(rename = "infoMandSind")]
    pub info_mand_sind: Option<Infomandsind>,
    #[serde(rename = "infoMandElet")]
    pub info_mand_elet: Option<Infomandelet>,
    #[serde(rename = "dtIniAfast")]
    pub dt_ini_afast: String,
    #[serde(rename = "codMotAfast")]
    pub cod_mot_afast: String,
    #[serde(rename = "infoMesmoMtv")]
    pub info_mesmo_mtv: Option<String>,
    #[serde(rename = "tpAcidTransito")]
    pub tp_acid_transito: Option<i64>,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforetif {
    #[serde(rename = "origRetif")]
    pub orig_retif: i64,
    #[serde(rename = "tpProc")]
    pub tp_proc: Option<i64>,
    #[serde(rename = "nrProc")]
    pub nr_proc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Fimafastamento {
    #[serde(rename = "dtTermAfast")]
    pub dt_term_afast: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoafastamento {
    #[serde(rename = "iniAfastamento")]
    pub ini_afastamento: Option<Iniafastamento>,
    #[serde(rename = "infoRetif")]
    pub info_retif: Option<Inforetif>,
    #[serde(rename = "fimAfastamento")]
    pub fim_afastamento: Option<Fimafastamento>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtafasttemp {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "infoAfastamento")]
    pub info_afastamento: Infoafastamento,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2230 {
    #[serde(rename = "evtAfastTemp")]
    pub evt_afast_temp: Evtafasttemp,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcessao {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2231 {
    #[serde(rename = "evtCessao")]
    pub evt_cessao: Evtcessao,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoamb {
    #[serde(rename = "localAmb")]
    pub local_amb: i64,
    #[serde(rename = "dscSetor")]
    pub dsc_setor: String,
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoativ {
    #[serde(rename = "dscAtivDes")]
    pub dsc_ativ_des: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Epi {
    #[serde(rename = "docAval")]
    pub doc_aval: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Epicompl {
    #[serde(rename = "medProtecao")]
    pub med_protecao: String,
    #[serde(rename = "condFuncto")]
    pub cond_functo: String,
    #[serde(rename = "usoInint")]
    pub uso_inint: String,
    #[serde(rename = "przValid")]
    pub prz_valid: String,
    #[serde(rename = "periodicTroca")]
    pub periodic_troca: String,
    #[serde(rename = "higienizacao")]
    pub higienizacao: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Epcepi {
    #[serde(rename = "epi")]
    pub epi: Vec<Epi>,
    #[serde(rename = "epiCompl")]
    pub epi_compl: Option<Epicompl>,
    #[serde(rename = "utilizEPC")]
    pub utiliz_epc: i64,
    #[serde(rename = "eficEpc")]
    pub efic_epc: Option<String>,
    #[serde(rename = "utilizEPI")]
    pub utiliz_epi: i64,
    #[serde(rename = "eficEpi")]
    pub efic_epi: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Agnoc {
    #[serde(rename = "epcEpi")]
    pub epc_epi: Option<Epcepi>,
    #[serde(rename = "codAgNoc")]
    pub cod_ag_noc: String,
    #[serde(rename = "dscAgNoc")]
    pub dsc_ag_noc: Option<String>,
    #[serde(rename = "tpAval")]
    pub tp_aval: Option<i64>,
    #[serde(rename = "intConc")]
    pub int_conc: Option<i64>,
    #[serde(rename = "limTol")]
    pub lim_tol: Option<i64>,
    #[serde(rename = "unMed")]
    pub un_med: Option<i64>,
    #[serde(rename = "tecMedicao")]
    pub tec_medicao: Option<String>,
    #[serde(rename = "nrProcJud")]
    pub nr_proc_jud: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Respreg {
    #[serde(rename = "cpfResp")]
    pub cpf_resp: String,
    #[serde(rename = "ideOC")]
    pub ide_oc: Option<i64>,
    #[serde(rename = "dscOC")]
    pub dsc_oc: Option<String>,
    #[serde(rename = "nrOC")]
    pub nr_oc: Option<String>,
    #[serde(rename = "ufOC")]
    pub uf_oc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Obs {
    #[serde(rename = "obsCompl")]
    pub obs_compl: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoexprisco {
    #[serde(rename = "infoAmb")]
    pub info_amb: Vec<Infoamb>,
    #[serde(rename = "infoAtiv")]
    pub info_ativ: Infoativ,
    #[serde(rename = "agNoc")]
    pub ag_noc: Vec<Agnoc>,
    #[serde(rename = "respReg")]
    pub resp_reg: Vec<Respreg>,
    #[serde(rename = "obs")]
    pub obs: Option<Obs>,
    #[serde(rename = "dtIniCondicao")]
    pub dt_ini_condicao: String,
    #[serde(rename = "dtFimCondicao")]
    pub dt_fim_condicao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtexprisco {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "infoExpRisco")]
    pub info_exp_risco: Infoexprisco,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2240 {
    #[serde(rename = "evtExpRisco")]
    pub evt_exp_risco: Evtexprisco,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforeintegr {
    #[serde(rename = "tpReint")]
    pub tp_reint: i64,
    #[serde(rename = "nrProcJud")]
    pub nr_proc_jud: Option<String>,
    #[serde(rename = "nrLeiAnistia")]
    pub nr_lei_anistia: Option<String>,
    #[serde(rename = "dtEfetRetorno")]
    pub dt_efet_retorno: String,
    #[serde(rename = "dtEfeito")]
    pub dt_efeito: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtreintegr {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "infoReintegr")]
    pub info_reintegr: Inforeintegr,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2298 {
    #[serde(rename = "evtReintegr")]
    pub evt_reintegr: Evtreintegr,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transftit {
    #[serde(rename = "cpfSubstituto")]
    pub cpf_substituto: String,
    #[serde(rename = "dtNascto")]
    pub dt_nascto: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detverbas {
    #[serde(rename = "descFolha")]
    pub desc_folha: Option<Descfolha>,
    #[serde(rename = "codRubr")]
    pub cod_rubr: String,
    #[serde(rename = "ideTabRubr")]
    pub ide_tab_rubr: String,
    #[serde(rename = "qtdRubr")]
    pub qtd_rubr: Option<i64>,
    #[serde(rename = "fatorRubr")]
    pub fator_rubr: Option<i64>,
    #[serde(rename = "vrRubr")]
    pub vr_rubr: i64,
    #[serde(rename = "indApurIR")]
    pub ind_apur_ir: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infosimples {
    #[serde(rename = "indSimples")]
    pub ind_simples: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Verbasresc {
    #[serde(rename = "dmDev")]
    pub dm_dev: Vec<Dmdev>,
    #[serde(rename = "procJudTrab")]
    pub proc_jud_trab: Vec<Procjudtrab>,
    #[serde(rename = "infoMV")]
    pub info_mv: Option<Infomv>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Remunaposdeslig {
    #[serde(rename = "indRemun")]
    pub ind_remun: Option<i64>,
    #[serde(rename = "dtFimRemun")]
    pub dt_fim_remun: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Consigfgts {
    #[serde(rename = "insConsig")]
    pub ins_consig: String,
    #[serde(rename = "nrContr")]
    pub nr_contr: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infodeslig {
    #[serde(rename = "dtDeslig")]
    pub dt_deslig: String,
    #[serde(rename = "mtvDeslig")]
    pub mtv_deslig: String,
    #[serde(rename = "dtProjFimAPI")]
    pub dt_proj_fim_api: Option<String>,
    #[serde(rename = "pensAlim")]
    pub pens_alim: Option<i64>,
    #[serde(rename = "percAliment")]
    pub perc_aliment: Option<i64>,
    #[serde(rename = "vrAlim")]
    pub vr_alim: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtdeslig {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "infoDeslig")]
    pub info_deslig: Infodeslig,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2299 {
    #[serde(rename = "evtDeslig")]
    pub evt_deslig: Evtdeslig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cargofuncao {
    #[serde(rename = "nmCargo")]
    pub nm_cargo: Option<String>,
    #[serde(rename = "CBOCargo")]
    pub cbo_cargo: Option<String>,
    #[serde(rename = "nmFuncao")]
    pub nm_funcao: Option<String>,
    #[serde(rename = "CBOFuncao")]
    pub cbo_funcao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FGTS {
    #[serde(rename = "dtOpcFGTS")]
    pub dt_opc_fgts: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infodirigentesindical {
    #[serde(rename = "tpRegPrev")]
    pub tp_reg_prev: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotrabcedido {
    #[serde(rename = "categOrig")]
    pub categ_orig: i64,
    #[serde(rename = "cnpjCednt")]
    pub cnpj_cednt: String,
    #[serde(rename = "matricCed")]
    pub matric_ced: String,
    #[serde(rename = "dtAdmCed")]
    pub dt_adm_ced: String,
    #[serde(rename = "tpRegTrab")]
    pub tp_reg_trab: i64,
    #[serde(rename = "tpRegPrev")]
    pub tp_reg_prev: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instensino {
    #[serde(rename = "cnpjInstEnsino")]
    pub cnpj_inst_ensino: Option<String>,
    #[serde(rename = "nmRazao")]
    pub nm_razao: Option<String>,
    #[serde(rename = "dscLograd")]
    pub dsc_lograd: Option<String>,
    #[serde(rename = "nrLograd")]
    pub nr_lograd: Option<String>,
    #[serde(rename = "bairro")]
    pub bairro: Option<String>,
    #[serde(rename = "cep")]
    pub cep: Option<String>,
    #[serde(rename = "codMunic")]
    pub cod_munic: Option<i64>,
    #[serde(rename = "uf")]
    pub uf: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ageintegracao {
    #[serde(rename = "cnpjAgntInteg")]
    pub cnpj_agnt_integ: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Supervisorestagio {
    #[serde(rename = "cpfSupervisor")]
    pub cpf_supervisor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoestagiario {
    #[serde(rename = "instEnsino")]
    pub inst_ensino: Instensino,
    #[serde(rename = "ageIntegracao")]
    pub age_integracao: Option<Ageintegracao>,
    #[serde(rename = "supervisorEstagio")]
    pub supervisor_estagio: Option<Supervisorestagio>,
    #[serde(rename = "natEstagio")]
    pub nat_estagio: String,
    #[serde(rename = "nivEstagio")]
    pub niv_estagio: Option<i64>,
    #[serde(rename = "areaAtuacao")]
    pub area_atuacao: Option<String>,
    #[serde(rename = "nrApol")]
    pub nr_apol: Option<String>,
    #[serde(rename = "dtPrevTerm")]
    pub dt_prev_term: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocomplementares {
    #[serde(rename = "cargoFuncao")]
    pub cargo_funcao: Option<Cargofuncao>,
    #[serde(rename = "remuneracao")]
    pub remuneracao: Option<Remuneracao>,
    #[serde(rename = "infoDirigenteSindical")]
    pub info_dirigente_sindical: Option<Infodirigentesindical>,
    #[serde(rename = "infoTrabCedido")]
    pub info_trab_cedido: Option<Infodirigentesindical>,
    #[serde(rename = "infoMandElet")]
    pub info_mand_elet: Option<Infomandelet>,
    #[serde(rename = "infoEstagiario")]
    pub info_estagiario: Option<Infoestagiario>,
    #[serde(rename = "localTrabGeral")]
    pub local_trab_geral: Option<Localtrabgeral>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Termino {
    #[serde(rename = "dtTerm")]
    pub dt_term: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotsvinicio {
    #[serde(rename = "infoComplementares")]
    pub info_complementares: Option<Infocomplementares>,
    #[serde(rename = "mudancaCPF")]
    pub mudanca_cpf: Option<Mudancacpf>,
    #[serde(rename = "afastamento")]
    pub afastamento: Option<Afastamento>,
    #[serde(rename = "termino")]
    pub termino: Option<Termino>,
    #[serde(rename = "cadIni")]
    pub cad_ini: String,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "dtInicio")]
    pub dt_inicio: String,
    #[serde(rename = "nrProcTrab")]
    pub nr_proc_trab: Option<String>,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evttsvinicio {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "trabalhador")]
    pub trabalhador: Trabalhador,
    #[serde(rename = "infoTSVInicio")]
    pub info_tsv_inicio: Infotsvinicio,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2300 {
    #[serde(rename = "evtTSVInicio")]
    pub evt_tsv_inicio: Evttsvinicio,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idetrabsemvinculo {
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotsvalteracao {
    #[serde(rename = "infoComplementares")]
    pub info_complementares: Option<Infocomplementares>,
    #[serde(rename = "dtAlteracao")]
    pub dt_alteracao: String,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evttsvaltcontr {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabSemVinculo")]
    pub ide_trab_sem_vinculo: Idetrabsemvinculo,
    #[serde(rename = "infoTSVAlteracao")]
    pub info_tsv_alteracao: Infotsvalteracao,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2306 {
    #[serde(rename = "evtTSVAltContr")]
    pub evt_tsv_alt_contr: Evttsvaltcontr,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotsvtermino {
    #[serde(rename = "mudancaCPF")]
    pub mudanca_cpf: Option<Mudancacpf>,
    #[serde(rename = "verbasResc")]
    pub verbas_resc: Option<Verbasresc>,
    #[serde(rename = "remunAposTerm")]
    pub remun_apos_term: Option<Remunaposdeslig>,
    #[serde(rename = "dtTerm")]
    pub dt_term: String,
    #[serde(rename = "mtvDesligTSV")]
    pub mtv_deslig_tsv: Option<String>,
    #[serde(rename = "pensAlim")]
    pub pens_alim: Option<i64>,
    #[serde(rename = "percAliment")]
    pub perc_aliment: Option<i64>,
    #[serde(rename = "vrAlim")]
    pub vr_alim: Option<i64>,
    #[serde(rename = "nrProcTrab")]
    pub nr_proc_trab: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evttsvtermino {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabSemVinculo")]
    pub ide_trab_sem_vinculo: Idetrabsemvinculo,
    #[serde(rename = "infoTSVTermino")]
    pub info_tsv_termino: Infotsvtermino,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2399 {
    #[serde(rename = "evtTSVTermino")]
    pub evt_tsv_termino: Evttsvtermino,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Beneficiario {
    #[serde(rename = "cpfBenef")]
    pub cpf_benef: String,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "cnpjOrigem")]
    pub cnpj_origem: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcdbenefin {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "beneficiario")]
    pub beneficiario: Beneficiario,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2400 {
    #[serde(rename = "evtCdBenefIn")]
    pub evt_cd_benef_in: Evtcdbenefin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dadosbenef {
    #[serde(rename = "dependente")]
    pub dependente: Vec<Dependente>,
    #[serde(rename = "nmBenefic")]
    pub nm_benefic: String,
    #[serde(rename = "sexo")]
    pub sexo: String,
    #[serde(rename = "racaCor")]
    pub raca_cor: i64,
    #[serde(rename = "estCiv")]
    pub est_civ: Option<i64>,
    #[serde(rename = "incFisMen")]
    pub inc_fis_men: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcdbenefalt {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideBenef")]
    pub ide_benef: Idebenef,
    #[serde(rename = "alteracao")]
    pub alteracao: Alteracao,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2405 {
    #[serde(rename = "evtCdBenefAlt")]
    pub evt_cd_benef_alt: Evtcdbenefalt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Instpenmorte {
    #[serde(rename = "tpDepInst")]
    pub tp_dep_inst: String,
    #[serde(rename = "descrDepInst")]
    pub descr_dep_inst: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infopenmorte {
    #[serde(rename = "instPenMorte")]
    pub inst_pen_morte: Option<Instpenmorte>,
    #[serde(rename = "tpPenMorte")]
    pub tp_pen_morte: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infohomolog {
    #[serde(rename = "sitHomolog")]
    pub sit_homolog: i64,
    #[serde(rename = "dtHomolog")]
    pub dt_homolog: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dadosbeneficio {
    #[serde(rename = "infoPenMorte")]
    pub info_pen_morte: Option<Infopenmorte>,
    #[serde(rename = "suspensao")]
    pub suspensao: Option<Suspensao>,
    #[serde(rename = "tpBeneficio")]
    pub tp_beneficio: String,
    #[serde(rename = "tpPlanRP")]
    pub tp_plan_rp: i64,
    #[serde(rename = "dsc")]
    pub dsc: Option<String>,
    #[serde(rename = "indSuspensao")]
    pub ind_suspensao: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sucessaobenef {
    #[serde(rename = "cnpjOrgaoAnt")]
    pub cnpj_orgao_ant: String,
    #[serde(rename = "nrBeneficioAnt")]
    pub nr_beneficio_ant: String,
    #[serde(rename = "dtTransf")]
    pub dt_transf: String,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobentermino {
    #[serde(rename = "dtTermBeneficio")]
    pub dt_term_beneficio: String,
    #[serde(rename = "mtvTermino")]
    pub mtv_termino: String,
    #[serde(rename = "cnpjOrgaoSuc")]
    pub cnpj_orgao_suc: Option<String>,
    #[serde(rename = "novoCPF")]
    pub novo_cpf: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobeninicio {
    #[serde(rename = "dadosBeneficio")]
    pub dados_beneficio: Dadosbeneficio,
    #[serde(rename = "sucessaoBenef")]
    pub sucessao_benef: Option<Sucessaobenef>,
    #[serde(rename = "mudancaCPF")]
    pub mudanca_cpf: Option<Mudancacpf>,
    #[serde(rename = "infoBenTermino")]
    pub info_ben_termino: Option<Infobentermino>,
    #[serde(rename = "cadIni")]
    pub cad_ini: String,
    #[serde(rename = "indSitBenef")]
    pub ind_sit_benef: Option<i64>,
    #[serde(rename = "nrBeneficio")]
    pub nr_beneficio: String,
    #[serde(rename = "dtIniBeneficio")]
    pub dt_ini_beneficio: String,
    #[serde(rename = "dtPublic")]
    pub dt_public: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcdbenin {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "beneficiario")]
    pub beneficiario: Beneficiario,
    #[serde(rename = "infoBenInicio")]
    pub info_ben_inicio: Infobeninicio,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2410 {
    #[serde(rename = "evtCdBenIn")]
    pub evt_cd_ben_in: Evtcdbenin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idebeneficio {
    #[serde(rename = "cpfBenef")]
    pub cpf_benef: String,
    #[serde(rename = "nrBeneficio")]
    pub nr_beneficio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Suspensao {
    #[serde(rename = "mtvSuspensao")]
    pub mtv_suspensao: String,
    #[serde(rename = "dscSuspensao")]
    pub dsc_suspensao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobenalteracao {
    #[serde(rename = "dadosBeneficio")]
    pub dados_beneficio: Dadosbeneficio,
    #[serde(rename = "dtAltBeneficio")]
    pub dt_alt_beneficio: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcdbenalt {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideBeneficio")]
    pub ide_beneficio: Idebeneficio,
    #[serde(rename = "infoBenAlteracao")]
    pub info_ben_alteracao: Infobenalteracao,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2416 {
    #[serde(rename = "evtCdBenAlt")]
    pub evt_cd_ben_alt: Evtcdbenalt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inforeativ {
    #[serde(rename = "dtEfetReativ")]
    pub dt_efet_reativ: String,
    #[serde(rename = "dtEfeito")]
    pub dt_efeito: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtreativben {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideBeneficio")]
    pub ide_beneficio: Idebeneficio,
    #[serde(rename = "infoReativ")]
    pub info_reativ: Inforeativ,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2418 {
    #[serde(rename = "evtReativBen")]
    pub evt_reativ_ben: Evtreativben,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcdbenterm {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideBeneficio")]
    pub ide_beneficio: Idebeneficio,
    #[serde(rename = "infoBenTermino")]
    pub info_ben_termino: Infobentermino,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2420 {
    #[serde(rename = "evtCdBenTerm")]
    pub evt_cd_ben_term: Evtcdbenterm,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideresp {
    #[serde(rename = "tpInsc")]
    pub tp_insc: i64,
    #[serde(rename = "nrInsc")]
    pub nr_insc: String,
    #[serde(rename = "dtAdmRespDir")]
    pub dt_adm_resp_dir: Option<String>,
    #[serde(rename = "matRespDir")]
    pub mat_resp_dir: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoprocesso {
    #[serde(rename = "nrProcTrab")]
    pub nr_proc_trab: String,
    #[serde(rename = "dtSent")]
    pub dt_sent: String,
    #[serde(rename = "ufVara")]
    pub uf_vara: String,
    #[serde(rename = "codMunic")]
    pub cod_munic: i64,
    #[serde(rename = "idVara")]
    pub id_vara: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infovinc {
    #[serde(rename = "duracao")]
    pub duracao: Option<Duracao>,
    #[serde(rename = "observacoes")]
    pub observacoes: Vec<Observacoes>,
    #[serde(rename = "sucessaoVinc")]
    pub sucessao_vinc: Option<Sucessaovinc>,
    #[serde(rename = "infoDeslig")]
    pub info_deslig: Option<Infodeslig>,
    #[serde(rename = "tpRegTrab")]
    pub tp_reg_trab: i64,
    #[serde(rename = "tpRegPrev")]
    pub tp_reg_prev: i64,
    #[serde(rename = "dtAdm")]
    pub dt_adm: String,
    #[serde(rename = "tmpParc")]
    pub tmp_parc: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoterm {
    #[serde(rename = "dtTerm")]
    pub dt_term: String,
    #[serde(rename = "mtvDesligTSV")]
    pub mtv_deslig_tsv: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocompl {
    #[serde(rename = "sucessaoVinc")]
    pub sucessao_vinc: Option<Sucessaovinc>,
    #[serde(rename = "infoInterm")]
    pub info_interm: Vec<Infointerm>,
    #[serde(rename = "infoComplCont")]
    pub info_compl_cont: Vec<Infocomplcont>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Mudcategativ {
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: Option<i64>,
    #[serde(rename = "dtMudCategAtiv")]
    pub dt_mud_categ_ativ: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Uniccontr {
    #[serde(rename = "matUnic")]
    pub mat_unic: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: Option<i64>,
    #[serde(rename = "dtInicio")]
    pub dt_inicio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Abono {
    #[serde(rename = "anoBase")]
    pub ano_base: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basecalculo {
    #[serde(rename = "infoAgNocivo")]
    pub info_ag_nocivo: Option<Infoagnocivo>,
    #[serde(rename = "vrBcCpMensal")]
    pub vr_bc_cp_mensal: i64,
    #[serde(rename = "vrBcCp13")]
    pub vr_bc_cp13: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infofgts {
    #[serde(rename = "ideEstab")]
    pub ide_estab: Vec<Ideestab>,
    #[serde(rename = "nrRecArqBase")]
    pub nr_rec_arq_base: String,
    #[serde(rename = "indExistInfo")]
    pub ind_exist_info: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basemudcateg {
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "vrBcCPrev")]
    pub vr_bc_c_prev: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infovlr {
    #[serde(rename = "abono")]
    pub abono: Vec<Abono>,
    #[serde(rename = "idePeriodo")]
    pub ide_periodo: Vec<Ideperiodo>,
    #[serde(rename = "compIni")]
    pub comp_ini: String,
    #[serde(rename = "compFim")]
    pub comp_fim: String,
    #[serde(rename = "indReperc")]
    pub ind_reperc: i64,
    #[serde(rename = "indenSD")]
    pub inden_sd: Option<String>,
    #[serde(rename = "indenAbono")]
    pub inden_abono: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocontr {
    #[serde(rename = "infoCompl")]
    pub info_compl: Option<Infocompl>,
    #[serde(rename = "mudCategAtiv")]
    pub mud_categ_ativ: Vec<Mudcategativ>,
    #[serde(rename = "unicContr")]
    pub unic_contr: Vec<Uniccontr>,
    #[serde(rename = "ideEstab")]
    pub ide_estab: Ideestab,
    #[serde(rename = "tpContr")]
    pub tp_contr: i64,
    #[serde(rename = "indContr")]
    pub ind_contr: String,
    #[serde(rename = "dtAdmOrig")]
    pub dt_adm_orig: Option<String>,
    #[serde(rename = "indReint")]
    pub ind_reint: Option<String>,
    #[serde(rename = "indCateg")]
    pub ind_categ: String,
    #[serde(rename = "indNatAtiv")]
    pub ind_nat_ativ: String,
    #[serde(rename = "indMotDeslig")]
    pub ind_mot_deslig: String,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: Option<i64>,
    #[serde(rename = "dtInicio")]
    pub dt_inicio: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idetrab {
    #[serde(rename = "calcTrib")]
    pub calc_trib: Vec<Calctrib>,
    #[serde(rename = "infoCRIRRF")]
    pub info_crirrf: Vec<Infocrirrf>,
    #[serde(rename = "infoIRComplem")]
    pub info_ir_complem: Option<Infoircomplem>,
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtproctrab {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoProcesso")]
    pub info_processo: Infoprocesso,
    #[serde(rename = "ideTrab")]
    pub ide_trab: Idetrab,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2500 {
    #[serde(rename = "evtProcTrab")]
    pub evt_proc_trab: Evtproctrab,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideproc {
    #[serde(rename = "origem")]
    pub origem: i64,
    #[serde(rename = "nrProcTrab")]
    pub nr_proc_trab: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocrcontrib {
    #[serde(rename = "tpCR")]
    pub tp_cr: i64,
    #[serde(rename = "vrCR")]
    pub vr_cr: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calctrib {
    #[serde(rename = "infoCRContrib")]
    pub info_cr_contrib: Vec<Infocrcontrib>,
    #[serde(rename = "perRef")]
    pub per_ref: String,
    #[serde(rename = "vrBcCpMensal")]
    pub vr_bc_cp_mensal: i64,
    #[serde(rename = "vrBcCp13")]
    pub vr_bc_cp13: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rendisen0561 {
    #[serde(rename = "vlrDiarias")]
    pub vlr_diarias: Option<i64>,
    #[serde(rename = "vlrAjudaCusto")]
    pub vlr_ajuda_custo: Option<i64>,
    #[serde(rename = "vlrIndResContrato")]
    pub vlr_ind_res_contrato: Option<i64>,
    #[serde(rename = "vlrAbonoPec")]
    pub vlr_abono_pec: Option<i64>,
    #[serde(rename = "vlrAuxMoradia")]
    pub vlr_aux_moradia: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoir {
    #[serde(rename = "infoProcJudRub")]
    pub info_proc_jud_rub: Vec<Infoprocjudrub>,
    #[serde(rename = "tpInfoIR")]
    pub tp_info_ir: i64,
    #[serde(rename = "valor")]
    pub valor: i64,
    #[serde(rename = "descRendimento")]
    pub desc_rendimento: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocrirrf {
    #[serde(rename = "infoIR")]
    pub info_ir: Option<Infoir>,
    #[serde(rename = "infoRRA")]
    pub info_rra: Option<Inforra>,
    #[serde(rename = "dedDepen")]
    pub ded_depen: Vec<Deddepen>,
    #[serde(rename = "penAlim")]
    pub pen_alim: Vec<Penalim>,
    #[serde(rename = "infoProcRet")]
    pub info_proc_ret: Vec<Infoprocret>,
    #[serde(rename = "tpCR")]
    pub tp_cr: i64,
    #[serde(rename = "vrCR")]
    pub vr_cr: i64,
    #[serde(rename = "vrCR13")]
    pub vr_cr13: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcontproc {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideProc")]
    pub ide_proc: Ideproc,
    #[serde(rename = "ideTrab")]
    pub ide_trab: Vec<Idetrab>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2501 {
    #[serde(rename = "evtContProc")]
    pub evt_cont_proc: Evtcontproc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtconsolidcontproc {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideProc")]
    pub ide_proc: Ideproc,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S2555 {
    #[serde(rename = "evtConsolidContProc")]
    pub evt_consolid_cont_proc: Evtconsolidcontproc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idefolhapagto {
    #[serde(rename = "indApuracao")]
    pub ind_apuracao: Option<i64>,
    #[serde(rename = "perApur")]
    pub per_apur: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoexclusao {
    #[serde(rename = "ideProcTrab")]
    pub ide_proc_trab: Ideproctrab,
    #[serde(rename = "tpEvento")]
    pub tp_evento: String,
    #[serde(rename = "nrRecEvt")]
    pub nr_rec_evt: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtexclusao {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoExclusao")]
    pub info_exclusao: Infoexclusao,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3000 {
    #[serde(rename = "evtExclusao")]
    pub evt_exclusao: Evtexclusao,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideproctrab {
    #[serde(rename = "nrProcTrab")]
    pub nr_proc_trab: String,
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: Option<String>,
    #[serde(rename = "perApurPgto")]
    pub per_apur_pgto: Option<String>,
    #[serde(rename = "ideSeqProc")]
    pub ide_seq_proc: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S3500 {
    #[serde(rename = "evtExcProcTrab")]
    pub evt_exc_proc_trab: Evtexclusao,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocpcalc {
    #[serde(rename = "tpCR")]
    pub tp_cr: i64,
    #[serde(rename = "vrCpSeg")]
    pub vr_cp_seg: i64,
    #[serde(rename = "vrDescSeg")]
    pub vr_desc_seg: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobasecs {
    #[serde(rename = "ind13")]
    pub ind13: i64,
    #[serde(rename = "tpValor")]
    pub tp_valor: i64,
    #[serde(rename = "valor")]
    pub valor: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calcterc {
    #[serde(rename = "tpCR")]
    pub tp_cr: i64,
    #[serde(rename = "vrCsSegTerc")]
    pub vr_cs_seg_terc: i64,
    #[serde(rename = "vrDescTerc")]
    pub vr_desc_terc: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detinfoperref {
    #[serde(rename = "ind13")]
    pub ind13: i64,
    #[serde(rename = "tpVrPerRef")]
    pub tp_vr_per_ref: i64,
    #[serde(rename = "vrPerRef")]
    pub vr_per_ref: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoperref {
    #[serde(rename = "ideADC")]
    pub ide_adc: Vec<Ideadc>,
    #[serde(rename = "detInfoPerRef")]
    pub det_info_per_ref: Vec<Detinfoperref>,
    #[serde(rename = "perRef")]
    pub per_ref: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocategincid {
    #[serde(rename = "infoBaseCS")]
    pub info_base_cs: Vec<Infobasecs>,
    #[serde(rename = "calcTerc")]
    pub calc_terc: Vec<Calcterc>,
    #[serde(rename = "infoPerRef")]
    pub info_per_ref: Vec<Infoperref>,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "indSimples")]
    pub ind_simples: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocp {
    #[serde(rename = "ideEstabLot")]
    pub ide_estab_lot: Vec<Ideestablot>,
    #[serde(rename = "classTrib")]
    pub class_trib: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobasepispasep {
    #[serde(rename = "ind13")]
    pub ind13: i64,
    #[serde(rename = "tpValorPisPasep")]
    pub tp_valor_pis_pasep: i64,
    #[serde(rename = "valorPisPasep")]
    pub valor_pis_pasep: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocategpispasep {
    #[serde(rename = "infoBasePisPasep")]
    pub info_base_pis_pasep: Vec<Infobasepispasep>,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infopispasep {
    #[serde(rename = "ideEstab")]
    pub ide_estab: Vec<Ideestab>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtbasestrab {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabalhador")]
    pub ide_trabalhador: Idetrabalhador,
    #[serde(rename = "infoCpCalc")]
    pub info_cp_calc: Vec<Infocpcalc>,
    #[serde(rename = "infoCp")]
    pub info_cp: Option<Infocp>,
    #[serde(rename = "infoPisPasep")]
    pub info_pis_pasep: Option<Infopispasep>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5001 {
    #[serde(rename = "evtBasesTrab")]
    pub evt_bases_trab: Evtbasestrab,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoprocjudrub {
    #[serde(rename = "nrProc")]
    pub nr_proc: String,
    #[serde(rename = "ufVara")]
    pub uf_vara: String,
    #[serde(rename = "codMunic")]
    pub cod_munic: i64,
    #[serde(rename = "idVara")]
    pub id_vara: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Totapurmen {
    #[serde(rename = "CRMen")]
    pub cr_men: String,
    #[serde(rename = "vlrRendTrib")]
    pub vlr_rend_trib: i64,
    #[serde(rename = "vlrRendTrib13")]
    pub vlr_rend_trib13: i64,
    #[serde(rename = "vlrPrevOficial")]
    pub vlr_prev_oficial: i64,
    #[serde(rename = "vlrPrevOficial13")]
    pub vlr_prev_oficial13: i64,
    #[serde(rename = "vlrCRMen")]
    pub vlr_cr_men: i64,
    #[serde(rename = "vlrCR13Men")]
    pub vlr_cr13_men: i64,
    #[serde(rename = "vlrParcIsenta65")]
    pub vlr_parc_isenta65: i64,
    #[serde(rename = "vlrParcIsenta65Dec")]
    pub vlr_parc_isenta65_dec: i64,
    #[serde(rename = "vlrDiarias")]
    pub vlr_diarias: i64,
    #[serde(rename = "vlrAjudaCusto")]
    pub vlr_ajuda_custo: i64,
    #[serde(rename = "vlrIndResContrato")]
    pub vlr_ind_res_contrato: i64,
    #[serde(rename = "vlrAbonoPec")]
    pub vlr_abono_pec: i64,
    #[serde(rename = "vlrRendMoleGrave")]
    pub vlr_rend_mole_grave: i64,
    #[serde(rename = "vlrRendMoleGrave13")]
    pub vlr_rend_mole_grave13: i64,
    #[serde(rename = "vlrAuxMoradia")]
    pub vlr_aux_moradia: i64,
    #[serde(rename = "vlrBolsaMedico")]
    pub vlr_bolsa_medico: i64,
    #[serde(rename = "vlrBolsaMedico13")]
    pub vlr_bolsa_medico13: i64,
    #[serde(rename = "vlrJurosMora")]
    pub vlr_juros_mora: i64,
    #[serde(rename = "vlrIsenOutros")]
    pub vlr_isen_outros: i64,
    #[serde(rename = "descRendimento")]
    pub desc_rendimento: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Totapurdia {
    #[serde(rename = "perApurDia")]
    pub per_apur_dia: i64,
    #[serde(rename = "CRDia")]
    pub cr_dia: String,
    #[serde(rename = "frmTribut")]
    pub frm_tribut: String,
    #[serde(rename = "paisResidExt")]
    pub pais_resid_ext: String,
    #[serde(rename = "vlrPagoDia")]
    pub vlr_pago_dia: i64,
    #[serde(rename = "vlrCRDia")]
    pub vlr_cr_dia: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Totinfoir {
    #[serde(rename = "consolidApurMen")]
    pub consolid_apur_men: Vec<Totapurmen>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtirrfbenef {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabalhador")]
    pub ide_trabalhador: Idetrabalhador,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5002 {
    #[serde(rename = "evtIrrfBenef")]
    pub evt_irrf_benef: Evtirrfbenef,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ideprocessofgts {
    #[serde(rename = "nrProc")]
    pub nr_proc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Detrubrsusp {
    #[serde(rename = "ideProcessoFGTS")]
    pub ide_processo_fgts: Vec<Ideprocessofgts>,
    #[serde(rename = "codRubr")]
    pub cod_rubr: String,
    #[serde(rename = "ideTabRubr")]
    pub ide_tab_rubr: String,
    #[serde(rename = "vrRubr")]
    pub vr_rubr: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Baseperapur {
    #[serde(rename = "tpValor")]
    pub tp_valor: i64,
    #[serde(rename = "indIncid")]
    pub ind_incid: i64,
    #[serde(rename = "baseFGTS")]
    pub base_fgts: i64,
    #[serde(rename = "vrFGTS")]
    pub vr_fgts: Option<i64>,
    #[serde(rename = "notAFT")]
    pub not_aft: Option<String>,
    #[serde(rename = "natRubr")]
    pub nat_rubr: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Baseperante {
    #[serde(rename = "tpValorE")]
    pub tp_valor_e: i64,
    #[serde(rename = "indIncidE")]
    pub ind_incid_e: i64,
    #[serde(rename = "baseFGTSE")]
    pub base_fgtse: i64,
    #[serde(rename = "vrFGTSE")]
    pub vr_fgtse: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobaseperante {
    #[serde(rename = "basePerAntE")]
    pub base_per_ant_e: Vec<Baseperante>,
    #[serde(rename = "perRef")]
    pub per_ref: String,
    #[serde(rename = "tpAcConv")]
    pub tp_ac_conv: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobasefgts {
    #[serde(rename = "basePerApur")]
    pub base_per_apur: Vec<Baseperapur>,
    #[serde(rename = "infoBasePerAntE")]
    pub info_base_per_ant_e: Vec<Infobaseperante>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Econsignado {
    #[serde(rename = "instFinanc")]
    pub inst_financ: String,
    #[serde(rename = "nrContrato")]
    pub nr_contrato: String,
    #[serde(rename = "vreConsignado")]
    pub vre_consignado: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotrabfgts {
    #[serde(rename = "infoFGTSProcTrab")]
    pub info_fgts_proc_trab: Infofgtsproctrab,
    #[serde(rename = "matricula")]
    pub matricula: Option<String>,
    #[serde(rename = "codCateg")]
    pub cod_categ: Option<i64>,
    #[serde(rename = "categOrig")]
    pub categ_orig: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Idelotacao {
    #[serde(rename = "infoBaseFGTS")]
    pub info_base_fgts: Option<Infobasefgts>,
    #[serde(rename = "codLotacao")]
    pub cod_lotacao: String,
    #[serde(rename = "tpLotacao")]
    pub tp_lotacao: String,
    #[serde(rename = "tpInsc")]
    pub tp_insc: Option<i64>,
    #[serde(rename = "nrInsc")]
    pub nr_insc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtbasesfgts {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideTrabalhador")]
    pub ide_trabalhador: Idetrabalhador,
    #[serde(rename = "infoFGTS")]
    pub info_fgts: Infofgts,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5003 {
    #[serde(rename = "evtBasesFGTS")]
    pub evt_bases_fgts: Evtbasesfgts,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocpseg {
    #[serde(rename = "vrDescCP")]
    pub vr_desc_cp: i64,
    #[serde(rename = "vrCpSeg")]
    pub vr_cp_seg: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infopj {
    #[serde(rename = "infoAtConc")]
    pub info_at_conc: Option<Infoativconcom>,
    #[serde(rename = "indCoop")]
    pub ind_coop: Option<i64>,
    #[serde(rename = "indConstr")]
    pub ind_constr: i64,
    #[serde(rename = "indSubstPatr")]
    pub ind_subst_patr: Option<i64>,
    #[serde(rename = "percRedContrib")]
    pub perc_red_contrib: Option<i64>,
    #[serde(rename = "percTransf")]
    pub perc_transf: Option<i64>,
    #[serde(rename = "indTribFolhaPisPasep")]
    pub ind_trib_folha_pis_pasep: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocontrib {
    #[serde(rename = "infoPJ")]
    pub info_pj: Option<Infopj>,
    #[serde(rename = "classTrib")]
    pub class_trib: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoestabref {
    #[serde(rename = "aliqRat")]
    pub aliq_rat: i64,
    #[serde(rename = "fap")]
    pub fap: Option<i64>,
    #[serde(rename = "aliqRatAjust")]
    pub aliq_rat_ajust: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocomplobra {
    #[serde(rename = "indSubstPatrObra")]
    pub ind_subst_patr_obra: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoestab {
    #[serde(rename = "infoEstabRef")]
    pub info_estab_ref: Option<Infoestabref>,
    #[serde(rename = "infoComplObra")]
    pub info_compl_obra: Option<Infocomplobra>,
    #[serde(rename = "cnaePrep")]
    pub cnae_prep: i64,
    #[serde(rename = "cnpjResp")]
    pub cnpj_resp: Option<String>,
    #[serde(rename = "aliqRat")]
    pub aliq_rat: i64,
    #[serde(rename = "fap")]
    pub fap: Option<i64>,
    #[serde(rename = "aliqRatAjust")]
    pub aliq_rat_ajust: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotercsusp {
    #[serde(rename = "codTerc")]
    pub cod_terc: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoemprparcial {
    #[serde(rename = "tpInscContrat")]
    pub tp_insc_contrat: i64,
    #[serde(rename = "nrInscContrat")]
    pub nr_insc_contrat: String,
    #[serde(rename = "tpInscProp")]
    pub tp_insc_prop: i64,
    #[serde(rename = "nrInscProp")]
    pub nr_insc_prop: String,
    #[serde(rename = "cnoObra")]
    pub cno_obra: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Dadosopport {
    #[serde(rename = "cnpjOpPortuario")]
    pub cnpj_op_portuario: String,
    #[serde(rename = "aliqRat")]
    pub aliq_rat: i64,
    #[serde(rename = "fap")]
    pub fap: i64,
    #[serde(rename = "aliqRatAjust")]
    pub aliq_rat_ajust: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basescp {
    #[serde(rename = "vrBcCp00")]
    pub vr_bc_cp00: i64,
    #[serde(rename = "vrBcCp15")]
    pub vr_bc_cp15: i64,
    #[serde(rename = "vrBcCp20")]
    pub vr_bc_cp20: i64,
    #[serde(rename = "vrBcCp25")]
    pub vr_bc_cp25: i64,
    #[serde(rename = "vrSuspBcCp00")]
    pub vr_susp_bc_cp00: i64,
    #[serde(rename = "vrSuspBcCp15")]
    pub vr_susp_bc_cp15: i64,
    #[serde(rename = "vrSuspBcCp20")]
    pub vr_susp_bc_cp20: i64,
    #[serde(rename = "vrSuspBcCp25")]
    pub vr_susp_bc_cp25: i64,
    #[serde(rename = "vrBcCp00VA")]
    pub vr_bc_cp00_va: Option<i64>,
    #[serde(rename = "vrBcCp15VA")]
    pub vr_bc_cp15_va: Option<i64>,
    #[serde(rename = "vrBcCp20VA")]
    pub vr_bc_cp20_va: Option<i64>,
    #[serde(rename = "vrBcCp25VA")]
    pub vr_bc_cp25_va: Option<i64>,
    #[serde(rename = "vrSuspBcCp00VA")]
    pub vr_susp_bc_cp00_va: Option<i64>,
    #[serde(rename = "vrSuspBcCp15VA")]
    pub vr_susp_bc_cp15_va: Option<i64>,
    #[serde(rename = "vrSuspBcCp20VA")]
    pub vr_susp_bc_cp20_va: Option<i64>,
    #[serde(rename = "vrSuspBcCp25VA")]
    pub vr_susp_bc_cp25_va: Option<i64>,
    #[serde(rename = "vrDescSest")]
    pub vr_desc_sest: i64,
    #[serde(rename = "vrCalcSest")]
    pub vr_calc_sest: i64,
    #[serde(rename = "vrDescSenat")]
    pub vr_desc_senat: i64,
    #[serde(rename = "vrCalcSenat")]
    pub vr_calc_senat: i64,
    #[serde(rename = "vrSalFam")]
    pub vr_sal_fam: i64,
    #[serde(rename = "vrSalMat")]
    pub vr_sal_mat: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basescp13 {
    #[serde(rename = "vrBcCp00")]
    pub vr_bc_cp00: i64,
    #[serde(rename = "vrBcCp15")]
    pub vr_bc_cp15: i64,
    #[serde(rename = "vrBcCp20")]
    pub vr_bc_cp20: i64,
    #[serde(rename = "vrBcCp25")]
    pub vr_bc_cp25: i64,
    #[serde(rename = "vrSuspBcCp00")]
    pub vr_susp_bc_cp00: i64,
    #[serde(rename = "vrSuspBcCp15")]
    pub vr_susp_bc_cp15: i64,
    #[serde(rename = "vrSuspBcCp20")]
    pub vr_susp_bc_cp20: i64,
    #[serde(rename = "vrSuspBcCp25")]
    pub vr_susp_bc_cp25: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basesremun {
    #[serde(rename = "basesCp")]
    pub bases_cp: Basescp,
    #[serde(rename = "basesCp13")]
    pub bases_cp13: Option<Basescp13>,
    #[serde(rename = "indIncid")]
    pub ind_incid: i64,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basesavnport {
    #[serde(rename = "vrBcCp00")]
    pub vr_bc_cp00: i64,
    #[serde(rename = "vrBcCp15")]
    pub vr_bc_cp15: i64,
    #[serde(rename = "vrBcCp20")]
    pub vr_bc_cp20: i64,
    #[serde(rename = "vrBcCp25")]
    pub vr_bc_cp25: i64,
    #[serde(rename = "vrBcCp13")]
    pub vr_bc_cp13: i64,
    #[serde(rename = "vrDescCP")]
    pub vr_desc_cp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basesaquis {
    #[serde(rename = "indAquis")]
    pub ind_aquis: i64,
    #[serde(rename = "vlrAquis")]
    pub vlr_aquis: i64,
    #[serde(rename = "vrCPDescPR")]
    pub vr_cp_desc_pr: i64,
    #[serde(rename = "vrCPNRet")]
    pub vr_cpn_ret: i64,
    #[serde(rename = "vrRatNRet")]
    pub vr_rat_n_ret: i64,
    #[serde(rename = "vrSenarNRet")]
    pub vr_senar_n_ret: i64,
    #[serde(rename = "vrCPCalcPR")]
    pub vr_cp_calc_pr: i64,
    #[serde(rename = "vrRatDescPR")]
    pub vr_rat_desc_pr: i64,
    #[serde(rename = "vrRatCalcPR")]
    pub vr_rat_calc_pr: i64,
    #[serde(rename = "vrSenarDesc")]
    pub vr_senar_desc: i64,
    #[serde(rename = "vrSenarCalc")]
    pub vr_senar_calc: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basescomerc {
    #[serde(rename = "indComerc")]
    pub ind_comerc: i64,
    #[serde(rename = "vrBcComPR")]
    pub vr_bc_com_pr: i64,
    #[serde(rename = "vrCPSusp")]
    pub vr_cp_susp: Option<i64>,
    #[serde(rename = "vrRatSusp")]
    pub vr_rat_susp: Option<i64>,
    #[serde(rename = "vrSenarSusp")]
    pub vr_senar_susp: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocrestab {
    #[serde(rename = "tpCR")]
    pub tp_cr: i64,
    #[serde(rename = "vrCR")]
    pub vr_cr: i64,
    #[serde(rename = "vrSuspCR")]
    pub vr_susp_cr: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Basespispasep {
    #[serde(rename = "vrBcPisPasep")]
    pub vr_bc_pis_pasep: i64,
    #[serde(rename = "vrBcPisPasepSusp")]
    pub vr_bc_pis_pasep_susp: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocs {
    #[serde(rename = "infoCPSeg")]
    pub info_cp_seg: Option<Infocpseg>,
    #[serde(rename = "infoContrib")]
    pub info_contrib: Infocontrib,
    #[serde(rename = "ideEstab")]
    pub ide_estab: Vec<Ideestab>,
    #[serde(rename = "infoCRContrib")]
    pub info_cr_contrib: Vec<Infocrcontrib>,
    #[serde(rename = "nrRecArqBase")]
    pub nr_rec_arq_base: String,
    #[serde(rename = "indExistInfo")]
    pub ind_exist_info: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtcs {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoCS")]
    pub info_cs: Infocs,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5011 {
    #[serde(rename = "evtCS")]
    pub evt_cs: Evtcs,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocrmen {
    #[serde(rename = "CRMen")]
    pub cr_men: String,
    #[serde(rename = "vrCRMen")]
    pub vr_cr_men: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infocrdia {
    #[serde(rename = "perApurDia")]
    pub per_apur_dia: i64,
    #[serde(rename = "CRDia")]
    pub cr_dia: String,
    #[serde(rename = "vrCRDia")]
    pub vr_cr_dia: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoirrf {
    #[serde(rename = "infoCRMen")]
    pub info_cr_men: Vec<Infocrmen>,
    #[serde(rename = "infoCRDia")]
    pub info_cr_dia: Vec<Infocrdia>,
    #[serde(rename = "nrRecArqBase")]
    pub nr_rec_arq_base: String,
    #[serde(rename = "indExistInfo")]
    pub ind_exist_info: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtirrf {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoIRRF")]
    pub info_irrf: Infoirrf,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5012 {
    #[serde(rename = "evtIrrf")]
    pub evt_irrf: Evtirrf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtfgts {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoFGTS")]
    pub info_fgts: Infofgts,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5013 {
    #[serde(rename = "evtFGTS")]
    pub evt_fgts: Evtfgts,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infotributos {
    #[serde(rename = "infoCRContrib")]
    pub info_cr_contrib: Vec<Infocrcontrib>,
    #[serde(rename = "perRef")]
    pub per_ref: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5501 {
    #[serde(rename = "evtTribProcTrab")]
    pub evt_trib_proc_trab: Evtconsolidcontproc,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Baseperref {
    #[serde(rename = "perRef")]
    pub per_ref: String,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "tpValorProcTrab")]
    pub tp_valor_proc_trab: i64,
    #[serde(rename = "remFGTSProcTrab")]
    pub rem_fgts_proc_trab: i64,
    #[serde(rename = "dpsFGTSProcTrab")]
    pub dps_fgts_proc_trab: Option<i64>,
    #[serde(rename = "remFGTSSefip")]
    pub rem_fgts_sefip: Option<i64>,
    #[serde(rename = "dpsFGTSSefip")]
    pub dps_fgts_sefip: Option<i64>,
    #[serde(rename = "remFGTSDecAnt")]
    pub rem_fgts_dec_ant: Option<i64>,
    #[serde(rename = "dpsFGTSDecAnt")]
    pub dps_fgts_dec_ant: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infofgtsproctrab {
    #[serde(rename = "ideEstab")]
    pub ide_estab: Option<Ideestab>,
    #[serde(rename = "totalFGTS")]
    pub total_fgts: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtfgtsproctrab {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideProc")]
    pub ide_proc: Ideproc,
    #[serde(rename = "ideTrabalhador")]
    pub ide_trabalhador: Idetrabalhador,
    #[serde(rename = "infoTrabFGTS")]
    pub info_trab_fgts: Vec<Infotrabfgts>,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S5503 {
    #[serde(rename = "evtFGTSProcTrab")]
    pub evt_fgts_proc_trab: Evtfgtsproctrab,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Cargo {
    #[serde(rename = "dtCargo")]
    pub dt_cargo: String,
    #[serde(rename = "CBOCargo")]
    pub cbo_cargo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Incorporacao {
    #[serde(rename = "tpInsc")]
    pub tp_insc: Option<i64>,
    #[serde(rename = "nrInsc")]
    pub nr_insc: Option<String>,
    #[serde(rename = "matIncorp")]
    pub mat_incorp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infoanotjud {
    #[serde(rename = "cargo")]
    pub cargo: Vec<Cargo>,
    #[serde(rename = "remuneracao")]
    pub remuneracao: Vec<Remuneracao>,
    #[serde(rename = "incorporacao")]
    pub incorporacao: Vec<Incorporacao>,
    #[serde(rename = "afastamento")]
    pub afastamento: Option<Afastamento>,
    #[serde(rename = "desligamento")]
    pub desligamento: Option<Desligamento>,
    #[serde(rename = "cpfTrab")]
    pub cpf_trab: String,
    #[serde(rename = "nmTrab")]
    pub nm_trab: String,
    #[serde(rename = "dtNascto")]
    pub dt_nascto: String,
    #[serde(rename = "dtAdm")]
    pub dt_adm: String,
    #[serde(rename = "matricula")]
    pub matricula: String,
    #[serde(rename = "codCateg")]
    pub cod_categ: i64,
    #[serde(rename = "natAtividade")]
    pub nat_atividade: i64,
    #[serde(rename = "tpContr")]
    pub tp_contr: i64,
    #[serde(rename = "dtTerm")]
    pub dt_term: Option<String>,
    #[serde(rename = "tpInscTrab")]
    pub tp_insc_trab: Option<i64>,
    #[serde(rename = "localTrabalho")]
    pub local_trabalho: Option<String>,
    #[serde(rename = "tpRegTrab")]
    pub tp_reg_trab: i64,
    #[serde(rename = "tpRegPrev")]
    pub tp_reg_prev: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtanotjud {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "infoProcesso")]
    pub info_processo: Infoprocesso,
    #[serde(rename = "infoAnotJud")]
    pub info_anot_jud: Infoanotjud,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S8200 {
    #[serde(rename = "evtAnotJud")]
    pub evt_anot_jud: Evtanotjud,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Infobaixa {
    #[serde(rename = "mtvDeslig")]
    pub mtv_deslig: String,
    #[serde(rename = "dtDeslig")]
    pub dt_deslig: String,
    #[serde(rename = "dtProjFimAPI")]
    pub dt_proj_fim_api: Option<String>,
    #[serde(rename = "nrProcTrab")]
    pub nr_proc_trab: String,
    #[serde(rename = "observacao")]
    pub observacao: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Evtbaixa {
    #[serde(rename = "ideEvento")]
    pub ide_evento: Ideevento,
    #[serde(rename = "ideEmpregador")]
    pub ide_empregador: Ideempregador,
    #[serde(rename = "ideVinculo")]
    pub ide_vinculo: Idevinculo,
    #[serde(rename = "infoBaixa")]
    pub info_baixa: Infobaixa,
    #[serde(rename = "Id")]
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct S8299 {
    #[serde(rename = "evtBaixa")]
    pub evt_baixa: Evtbaixa,
}
