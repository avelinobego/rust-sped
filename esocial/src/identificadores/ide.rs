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

use elementtree::Element;

use crate::toxml::toxml::ToElement;

struct Ide<'a> {
    tp_insc: i32,
    nr_insc: &'a str,
}
// --------------------------------------------------
pub struct IdeEmpregador<'a> {
    ide: Ide<'a>,
}

impl<'a> IdeEmpregador<'a> {
    pub fn new(tp_insc: i32, nr_insc: &'a str) -> Self {
        IdeEmpregador {
            ide: Ide {
                tp_insc: tp_insc,
                nr_insc: nr_insc,
            },
        }
    }
}

impl ToElement for IdeEmpregador<'_> {
    fn to_element(&self) -> Element {
        let mut root = Element::new("ideEmpregador");
        let tp_insc = root.append_new_child("tpInsc");
        tp_insc.set_text(self.ide.tp_insc.to_string());
        let nr_insc = root.append_new_child("nrInsc");
        nr_insc.set_text(self.ide.nr_insc);
        root
    }
}
// --------------------------------------------------
pub struct IdeTransmissor<'a> {
    ide: Ide<'a>,
}

impl<'a> IdeTransmissor<'a> {
    pub fn new(tp_insc: i32, nr_insc: &'a str) -> Self {
        IdeTransmissor {
            ide: Ide {
                tp_insc: tp_insc,
                nr_insc: nr_insc,
            },
        }
    }
}

impl ToElement for IdeTransmissor<'_> {
    fn to_element(&self) -> Element {
        let mut root = Element::new("ideTransmissor");
        let tp_insc = root.append_new_child("tpInsc");
        tp_insc.set_text(self.ide.tp_insc.to_string());
        let nr_insc = root.append_new_child("nrInsc");
        nr_insc.set_text(self.ide.nr_insc);
        root
    }
}
// --------------------------------------------------
