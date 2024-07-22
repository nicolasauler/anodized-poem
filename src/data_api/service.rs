use crate::{
    data::{Levels, Teachers},
    schema::{Punch, PunchGet, PunchGetInput, PunchQuantity, PunchReport, SignInData},
    util::{get_first_day_from_month, get_last_day_from_month},
    DbPool,
};

use base64::{engine::general_purpose, Engine as _};
use chrono::{Datelike, NaiveDate, Utc};
use num_traits::cast::ToPrimitive;
use poem::{error::InternalServerError, session::Session, web::Data, Result};
use poem_openapi::{param::Path, payload::Json};
use std::str::FromStr;

pub async fn create_punch(
    pool: Data<&DbPool>,
    level: Path<Levels>,
    teacher: Path<Teachers>,
    date: NaiveDate,
) -> Result<Punch> {
    let punch: (i64, String, String, NaiveDate) = sqlx::query_as(
            "INSERT INTO attendance (level, teacher, date) VALUES ($1, $2, $3) RETURNING id, level, teacher, date",
        )
            .bind(level.to_string())
            .bind(teacher.to_string())
            .bind(date)
            .fetch_one(pool.0)
            .await
            .map_err(InternalServerError)?;

    Ok(Punch {
        id: punch.0,
        level: Levels::from_str(&punch.1).unwrap(),
        teacher: Teachers::from_str(&punch.2).unwrap(),
        date: punch.3,
    })
}

pub async fn get_teacher_attendance(
    pool: Data<&DbPool>,
    teacher: Path<Teachers>,
    punch_get_input: Json<PunchGetInput>,
) -> Result<PunchReport> {
    let punches: Vec<(i64, String, String, NaiveDate)> = sqlx::query_as(
            "SELECT id, level, teacher, date FROM attendance WHERE teacher = $1 AND date BETWEEN $2 AND $3 ORDER BY date",
        )
        .bind(teacher.to_string())
        .bind(punch_get_input.start_date.unwrap_or_else(|| get_first_day_from_month(Utc::now().month(), Utc::now().year())))
        .bind(punch_get_input.end_date.unwrap_or_else(|| get_last_day_from_month(Utc::now().month(), Utc::now().year())))
        .fetch_all(pool.0)
        .await
        .map_err(InternalServerError)?;

    let punches: Vec<PunchGet> = punches
        .into_iter()
        .map(|punch| PunchGet {
            level: Levels::from_str(&punch.1).unwrap(),
            teacher: Teachers::from_str(&punch.2).unwrap(),
            date: punch.3,
        })
        .collect();

    Ok(PunchReport {
        n_classes: punches.len().to_u32().unwrap_or(u32::MAX),
        punches,
    })
}

pub async fn get_general_attendance(
    pool: Data<&DbPool>,
    punch_get_input: Json<PunchGetInput>,
) -> Result<Vec<PunchQuantity>> {
    let punches: Vec<(String, i64)> = sqlx::query_as(
        "SELECT teacher, COUNT(*) FROM attendance WHERE date BETWEEN $1 AND $2 GROUP BY teacher ORDER BY teacher",
    )
    .bind(
        punch_get_input
            .start_date
            .unwrap_or_else(|| get_first_day_from_month(Utc::now().month(), Utc::now().year())),
    )
    .bind(
        punch_get_input
            .end_date
            .unwrap_or_else(|| get_last_day_from_month(Utc::now().month(), Utc::now().year())),
    )
    .fetch_all(pool.0)
    .await
    .map_err(InternalServerError)?;

    let punch_quantities: Vec<PunchQuantity> = punches
        .into_iter()
        .map(|punch| PunchQuantity {
            teacher: Teachers::from_str(&punch.0).unwrap(),
            n_classes: punch.1.to_u32().unwrap_or(u32::MAX),
        })
        .collect();

    Ok(punch_quantities)
}

pub async fn sign_in(
    username: &str,
    password: &str,
    session: &Session,
    sign_in_data: Json<SignInData>,
) -> bool {
    if sign_in_data.username == username && sign_in_data.password == password {
        session.set("username", sign_in_data.username.clone());
        let uname = &sign_in_data.username;
        let pass = &sign_in_data.password;
        session.set(
            "auth",
            general_purpose::STANDARD.encode(format!("{uname}:{pass}").as_bytes()),
        );
        true
    } else {
        false
    }
}
