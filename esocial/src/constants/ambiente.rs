// Copyright (C) 2024 Avelino Bego
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

pub enum Ambiente {
    Producao = 1,
    PreProducaoDadosReais, 
    PreProduçaoDadosFicticios, 
    Homologação = 6, 
    Validação, 
    Testes, 
    Desenvolvimento,    
}