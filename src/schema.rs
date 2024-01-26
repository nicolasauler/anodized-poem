use std::fmt::Display;

use crate::data::{Levels, Meses, Teachers};

use chrono::NaiveDate;
use poem_openapi::{
    payload::{Html, Json, PlainText},
    ApiResponse, Object,
};

use sqlx::FromRow;

#[derive(Object, FromRow)]
pub struct Punch {
    pub id: i64,
    pub level: Levels,
    pub teacher: Teachers,
    pub date: NaiveDate,
}

#[derive(Object)]
pub struct PunchCreate {
    pub date: Option<NaiveDate>,
}

#[derive(Object, Clone)]
pub struct PunchCreateHypermedia {
    pub level: Levels,
    pub teacher: Teachers,
    pub date: Option<NaiveDate>,
}

#[derive(Object)]
pub struct PunchGet {
    pub level: Levels,
    pub teacher: Teachers,
    pub date: NaiveDate,
}

#[derive(Object)]
pub struct PunchGetInput {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Object, Clone)]
pub struct PunchGetInputHypermedia {
    pub mes: Meses,
    pub ano: String,
}

#[derive(Object, Clone)]
pub struct PunchGetTeacherHypermedia {
    pub teacher: Teachers,
    pub mes: Meses,
    pub ano: String,
}

#[derive(Object)]
pub struct PunchReport {
    pub n_classes: i64,
    pub punches: Vec<PunchGet>,
}

#[derive(Object)]
pub struct PunchQuantity {
    pub teacher: Teachers,
    pub n_classes: i64,
}

impl Display for PunchQuantity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.teacher, self.n_classes)
    }
}

#[derive(Object)]
pub struct SignInData {
    pub username: String,
    pub password: String,
}

#[derive(Object)]
pub struct MyError {
    pub error: String,
}

#[derive(ApiResponse)]
pub enum CreateResponse {
    #[oai(status = 200)]
    Punch(Json<Punch>),

    #[oai(status = 400)]
    BadRequest(Json<MyError>),

    #[oai(status = 401)]
    Unauthorized(PlainText<String>),
}

#[derive(ApiResponse)]
pub enum TeacherGetResponse {
    #[oai(status = 200)]
    PunchReport(Json<PunchReport>),
}

#[derive(ApiResponse)]
pub enum GetResponse {
    #[oai(status = 200, content_type = "json")]
    PunchQuantity(Json<Vec<PunchQuantity>>),
}

#[derive(ApiResponse)]
pub enum HealthResponse {
    #[oai(status = 200)]
    Health(Html<String>),
}

#[derive(ApiResponse)]
pub enum SignInResponse {
    #[oai(status = 200)]
    Redirect(PlainText<String>),
}
