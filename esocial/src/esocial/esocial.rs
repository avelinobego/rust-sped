// Copyright (C) 2023 Avelino Bego
//
// This file is part of SPED.
//
// SPED is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, or
// (at your option) any later version.
//
// SPED is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with SPED.  If not, see <http://www.gnu.org/licenses/>.

use crate::{toxml::toxml::ToElement, utils::exec};
use elementtree::Element;

pub struct ESocial<'a, T: ToElement> {
    versao: &'a str,
    lote: T,
}

impl<'a, T: ToElement> ESocial<'a, T> {
    pub fn new(versao: &'a str, lote: T) -> Self {
        ESocial { versao, lote }
    }
}

impl<T: ToElement> ToElement for ESocial<'_, T> {
    fn do_element(&self) -> Element {
        let ns = format!(
            "http://www.esocial.gov.br/schema/lote/eventos/envio/{}/",
            self.versao
        );

        let mut root = Element::new((ns.as_str(), "eSocial"));
        exec::add_child(&mut root, self.lote.to_element());
        root
    }

    fn validate(&self) -> Result<(), &'static str> {
        Ok(())
    }
}

#[test]
fn test_esocial_xml() {
    // use structured_logger::Builder;
    // Initialize the logger.
    // Builder::new().init();

    impl ToElement for &'static str {
        fn do_element(&self) -> Element {
            let mut root = Element::new("tag");
            root.set_text(*self);
            root
        }

        fn validate(&self) -> Result<(), &'static str> {
            Ok(())
        }
    }

    let e = ESocial::new("v1_1_1", "teste");
    let dado = "<?xml version=\"1.0\" encoding=\"utf-8\"?><eSocial xmlns=\"http://www.esocial.gov.br/schema/lote/eventos/envio/v1_1_1/\"><tag>teste</tag></eSocial>";
    let mut correto = false;

    if let Some(ele) = e.to_element() {
        if let Ok(s) = ele.to_string() {
            correto = dado == s;
        }
    }

    assert!(correto, "Falha no teste");

}
