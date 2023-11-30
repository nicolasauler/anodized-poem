use poem_openapi::Enum;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use strum::EnumIter;

#[derive(Enum, Serialize, Clone, EnumIter, Deserialize)]
pub enum Teachers {
    Amanda,
    Ana,
    Belga,
    Bruna,
    Carol,
    Dani,
    Fe,
    Gil,
    Isa,
    Mafe,
    Mel,
    Rascunho,
    Toyo,
    Valle,
    Vitinho,
}

impl FromStr for Teachers {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Amanda" => Ok(Teachers::Amanda),
            "Ana" => Ok(Teachers::Ana),
            "Belga" => Ok(Teachers::Belga),
            "Bruna" => Ok(Teachers::Bruna),
            "Carol" => Ok(Teachers::Carol),
            "Dani" => Ok(Teachers::Dani),
            "Fe" => Ok(Teachers::Fe),
            "Gil" => Ok(Teachers::Gil),
            "Isa" => Ok(Teachers::Isa),
            "Mafe" => Ok(Teachers::Mafe),
            "Mel" => Ok(Teachers::Mel),
            "Rascunho" => Ok(Teachers::Rascunho),
            "Toyo" => Ok(Teachers::Toyo),
            "Valle" => Ok(Teachers::Valle),
            "Vitinho" => Ok(Teachers::Vitinho),
            _ => Err(format!("{} is not a valid teacher", s)),
        }
    }
}

impl Display for Teachers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Teachers::Amanda => write!(f, "Amanda"),
            Teachers::Ana => write!(f, "Ana"),
            Teachers::Belga => write!(f, "Belga"),
            Teachers::Bruna => write!(f, "Bruna"),
            Teachers::Carol => write!(f, "Carol"),
            Teachers::Dani => write!(f, "Dani"),
            Teachers::Fe => write!(f, "Fe"),
            Teachers::Gil => write!(f, "Gil"),
            Teachers::Isa => write!(f, "Isa"),
            Teachers::Mafe => write!(f, "Mafe"),
            Teachers::Mel => write!(f, "Mel"),
            Teachers::Rascunho => write!(f, "Rascunho"),
            Teachers::Toyo => write!(f, "Toyo"),
            Teachers::Valle => write!(f, "Valle"),
            Teachers::Vitinho => write!(f, "Vitinho"),
        }
    }
}

#[derive(Enum, Clone, EnumIter, Serialize, Deserialize)]
pub enum Levels {
    Experimental,
    Ciclo,
    Transp,
    Branca,
    Azul,
    AzulAv1,
    AzulAv2,
    Preta,
    PretaAv,
}

impl FromStr for Levels {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Experimental" => Ok(Levels::Experimental),
            "Ciclo" => Ok(Levels::Ciclo),
            "Transp" => Ok(Levels::Transp),
            "Branca" => Ok(Levels::Branca),
            "Azul" => Ok(Levels::Azul),
            "AzulAv1" => Ok(Levels::AzulAv1),
            "AzulAv2" => Ok(Levels::AzulAv2),
            "Preta" => Ok(Levels::Preta),
            "PretaAv" => Ok(Levels::PretaAv),
            _ => Err(format!("{} is not a valid level", s)),
        }
    }
}

impl Display for Levels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Levels::Experimental => write!(f, "Experimental"),
            Levels::Ciclo => write!(f, "Ciclo"),
            Levels::Transp => write!(f, "Transp"),
            Levels::Branca => write!(f, "Branca"),
            Levels::Azul => write!(f, "Azul"),
            Levels::AzulAv1 => write!(f, "AzulAv1"),
            Levels::AzulAv2 => write!(f, "AzulAv2"),
            Levels::Preta => write!(f, "Preta"),
            Levels::PretaAv => write!(f, "PretaAv"),
        }
    }
}

#[derive(Enum, Clone, EnumIter, Serialize, Deserialize, PartialEq)]
pub enum Meses {
    Janeiro,
    Fevereiro,
    Março,
    Abril,
    Maio,
    Junho,
    Julho,
    Agosto,
    Setembro,
    Outubro,
    Novembro,
    Dezembro,
}

impl FromStr for Meses {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Janeiro" => Ok(Meses::Janeiro),
            "Fevereiro" => Ok(Meses::Fevereiro),
            "Março" => Ok(Meses::Março),
            "Abril" => Ok(Meses::Abril),
            "Maio" => Ok(Meses::Maio),
            "Junho" => Ok(Meses::Junho),
            "Julho" => Ok(Meses::Julho),
            "Agosto" => Ok(Meses::Agosto),
            "Setembro" => Ok(Meses::Setembro),
            "Outubro" => Ok(Meses::Outubro),
            "Novembro" => Ok(Meses::Novembro),
            "Dezembro" => Ok(Meses::Dezembro),
            _ => Err(format!("{} is not a valid month", s)),
        }
    }
}

impl Display for Meses {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Meses::Janeiro => write!(f, "Janeiro"),
            Meses::Fevereiro => write!(f, "Fevereiro"),
            Meses::Março => write!(f, "Março"),
            Meses::Abril => write!(f, "Abril"),
            Meses::Maio => write!(f, "Maio"),
            Meses::Junho => write!(f, "Junho"),
            Meses::Julho => write!(f, "Julho"),
            Meses::Agosto => write!(f, "Agosto"),
            Meses::Setembro => write!(f, "Setembro"),
            Meses::Outubro => write!(f, "Outubro"),
            Meses::Novembro => write!(f, "Novembro"),
            Meses::Dezembro => write!(f, "Dezembro"),
        }
    }
}

impl Meses {
    pub fn from_chrono_month(month: chrono::Month) -> Self {
        match month {
            chrono::Month::January => Meses::Janeiro,
            chrono::Month::February => Meses::Fevereiro,
            chrono::Month::March => Meses::Março,
            chrono::Month::April => Meses::Abril,
            chrono::Month::May => Meses::Maio,
            chrono::Month::June => Meses::Junho,
            chrono::Month::July => Meses::Julho,
            chrono::Month::August => Meses::Agosto,
            chrono::Month::September => Meses::Setembro,
            chrono::Month::October => Meses::Outubro,
            chrono::Month::November => Meses::Novembro,
            chrono::Month::December => Meses::Dezembro,
        }
    }

    pub fn to_chrono_month(&self) -> chrono::Month {
        match self {
            Meses::Janeiro => chrono::Month::January,
            Meses::Fevereiro => chrono::Month::February,
            Meses::Março => chrono::Month::March,
            Meses::Abril => chrono::Month::April,
            Meses::Maio => chrono::Month::May,
            Meses::Junho => chrono::Month::June,
            Meses::Julho => chrono::Month::July,
            Meses::Agosto => chrono::Month::August,
            Meses::Setembro => chrono::Month::September,
            Meses::Outubro => chrono::Month::October,
            Meses::Novembro => chrono::Month::November,
            Meses::Dezembro => chrono::Month::December,
        }
    }
}

#[derive(Enum, Clone, EnumIter, Serialize, Deserialize)]
pub enum DiasDaSemana {
    Domingo,
    Segunda,
    Terça,
    Quarta,
    Quinta,
    Sexta,
    Sábado,
}

impl Display for DiasDaSemana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DiasDaSemana::Domingo => write!(f, "Domingo"),
            DiasDaSemana::Segunda => write!(f, "Segunda"),
            DiasDaSemana::Terça => write!(f, "Terça"),
            DiasDaSemana::Quarta => write!(f, "Quarta"),
            DiasDaSemana::Quinta => write!(f, "Quinta"),
            DiasDaSemana::Sexta => write!(f, "Sexta"),
            DiasDaSemana::Sábado => write!(f, "Sábado"),
        }
    }
}

impl DiasDaSemana {
    pub fn from_chrono_weekday(weekday: chrono::Weekday) -> Self {
        match weekday {
            chrono::Weekday::Sun => DiasDaSemana::Domingo,
            chrono::Weekday::Mon => DiasDaSemana::Segunda,
            chrono::Weekday::Tue => DiasDaSemana::Terça,
            chrono::Weekday::Wed => DiasDaSemana::Quarta,
            chrono::Weekday::Thu => DiasDaSemana::Quinta,
            chrono::Weekday::Fri => DiasDaSemana::Sexta,
            chrono::Weekday::Sat => DiasDaSemana::Sábado,
        }
    }
}
