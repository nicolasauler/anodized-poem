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
    Convidado,
    Dani,
    Fe,
    Gil,
    Isa,
    Lucas,
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
            "Amanda" => Ok(Self::Amanda),
            "Ana" => Ok(Self::Ana),
            "Belga" => Ok(Self::Belga),
            "Bruna" => Ok(Self::Bruna),
            "Carol" => Ok(Self::Carol),
            "Convidado" => Ok(Self::Convidado),
            "Dani" => Ok(Self::Dani),
            "Fe" => Ok(Self::Fe),
            "Gil" => Ok(Self::Gil),
            "Isa" => Ok(Self::Isa),
            "Lucas" => Ok(Self::Lucas),
            "Mafe" => Ok(Self::Mafe),
            "Mel" => Ok(Self::Mel),
            "Rascunho" => Ok(Self::Rascunho),
            "Toyo" => Ok(Self::Toyo),
            "Valle" => Ok(Self::Valle),
            "Vitinho" => Ok(Self::Vitinho),
            _ => Err(format!("{s} is not a valid teacher")),
        }
    }
}

impl Display for Teachers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amanda => write!(f, "Amanda"),
            Self::Ana => write!(f, "Ana"),
            Self::Belga => write!(f, "Belga"),
            Self::Bruna => write!(f, "Bruna"),
            Self::Carol => write!(f, "Carol"),
            Self::Convidado => write!(f, "Convidado"),
            Self::Dani => write!(f, "Dani"),
            Self::Fe => write!(f, "Fe"),
            Self::Gil => write!(f, "Gil"),
            Self::Isa => write!(f, "Isa"),
            Self::Lucas => write!(f, "Lucas"),
            Self::Mafe => write!(f, "Mafe"),
            Self::Mel => write!(f, "Mel"),
            Self::Rascunho => write!(f, "Rascunho"),
            Self::Toyo => write!(f, "Toyo"),
            Self::Valle => write!(f, "Valle"),
            Self::Vitinho => write!(f, "Vitinho"),
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
    AzulIntermediaria,
    AzulAvancada,
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
            "AzulIntermediaria" => Ok(Levels::AzulIntermediaria),
            "AzulAvancada" => Ok(Levels::AzulAvancada),
            "Preta" => Ok(Levels::Preta),
            "PretaAv" => Ok(Levels::PretaAv),
            _ => Err(format!("{s} is not a valid level")),
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
            Levels::AzulIntermediaria => write!(f, "AzulIntermediaria"),
            Levels::AzulAvancada => write!(f, "AzulAvancada"),
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
            _ => Err(format!("{s} is not a valid month")),
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
