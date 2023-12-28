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

#[allow(dead_code)]
pub enum Eventos {
    EventosTabelas = 1,
    EventosNaoPeriodicos,
    EventosPeriodicos,
}

#[test]
pub fn test_eventos() {

    let et = Eventos::EventosTabelas;
    assert_eq!(1, et as i32);

    let enp = Eventos::EventosNaoPeriodicos;
    assert_eq!(2, enp as i32);

    let ep = Eventos::EventosPeriodicos;
    assert_eq!(3, ep as i32);
}
