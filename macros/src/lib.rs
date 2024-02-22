// Copyright (C) 2024 Avelino Bego
//
// This file is part of SPED.
//
// SPED is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 2 of the License, orst
// (at your option) any later version.
//
// SPED is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with SPED.  If not, see <http://www.gnu.org/licenses/>.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct};

#[proc_macro_derive(builder_elements)]
pub fn builder_elements(item: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(item);

    let temp = ident.to_string();
    let mut name = temp[0..3].to_lowercase();
    let sufix = &temp[3..];

    name.push_str(sufix);

    let func_validate_name = Ident::new(&format!("validate_{}", temp.to_lowercase()), ident.span());

    let implementacao = quote! {
        impl #ident {
            pub fn build_elements(&self) -> Result<Element, &'static str>{

                let mut root = Element::new(#name);

                if let Err(e) = #func_validate_name(self) {
                    return Err(e);
                }

                // Construção


                Ok(root)

            }
        }
    };

    implementacao.into()
}
