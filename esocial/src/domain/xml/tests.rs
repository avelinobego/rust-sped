use quick_xml::se::to_string;

use crate::domain::xml::{Evtinfoempregador, Ideempregador, Ideevento, S1000};

#[test]
fn test_event_creation() {
    let ev = Evtinfoempregador {
        ide_evento: Ideevento {
            tp_amb: 1,
            proc_emi: 1,
            ver_proc: "1.0".to_string(),
        },
        ide_empregador: Ideempregador {
            tp_insc: 1,
            nr_insc: "12345678000195".to_string(),
        },
        id: "ID123456789".to_string(),
    };

    let s1000 = S1000 {
        xmnls: "http://www.esocial.gov.br/schema/evtInfoEmpregador/v1_0_0".to_string(),
        evt_info_empregador: ev,
    };

    match to_string(&s1000) {
        Ok(xml) => println!("XML Gerado:\n{}", xml),
        Err(e) => eprintln!("Erro ao serializar: {}", e),
    }
}
