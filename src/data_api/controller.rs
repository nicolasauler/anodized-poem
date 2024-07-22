use crate::data::{Levels, Teachers};
use crate::data_api::service::{create_punch, get_general_attendance, get_teacher_attendance};
use crate::schema::{
    CreateResponse, GetResponse, HealthResponse, MyError, PunchCreate, PunchCreateHypermedia,
    PunchGetInput, TeacherGetResponse,
};
use crate::{ApiTags, DbPool, SecretCredentials};

use chrono::Utc;
use poem::{http::StatusCode, web::Data, Error, Result};
use poem_openapi::{
    auth::Basic,
    param::Path,
    payload::PlainText,
    payload::{Html, Json, Response},
    OpenApi, SecurityScheme,
};

/// Basic authorization
#[derive(SecurityScheme)]
#[oai(ty = "basic")]
struct MyBasicAuthorization(Basic);

pub struct DataApi;

#[OpenApi(tag = "ApiTags::Data")]
impl DataApi {
    /// Create a new punch
    #[oai(path = "/data/attendance/:teacher/:level", method = "post")]
    async fn create_punch(
        &self,
        secrets: Data<&SecretCredentials>,
        auth: MyBasicAuthorization,
        pool: Data<&DbPool>,
        level: Path<Levels>,
        teacher: Path<Teachers>,
        punch_create: Json<PunchCreate>,
    ) -> Result<CreateResponse> {
        if auth.0.username != secrets.username || auth.0.password != secrets.password {
            tracing::info!("Not authenticated");
            return Err(Error::from_status(StatusCode::UNAUTHORIZED));
        }

        let today = Utc::now().naive_local().date();
        let date = match punch_create.date {
            Some(date) => {
                if date > today {
                    tracing::info!("Date {} is in the future. Today is {}", date, today);
                    // return Err(Error::from_status(StatusCode::BAD_REQUEST));
                    return Ok(CreateResponse::BadRequest(Json(MyError {
                        error: "Date is in the future".to_string(),
                    })));
                }
                date
            }
            None => today,
        };

        let punch = create_punch(pool, level, teacher, date).await.unwrap();
        Ok(CreateResponse::Punch(Json(punch)))
    }

    /// Get teacher attendance (more specific)
    #[oai(path = "/data/attendance/:teacher", method = "post")]
    async fn get_teacher_attendance(
        &self,
        pool: Data<&DbPool>,
        teacher: Path<Teachers>,
        punch_get_input: Json<PunchGetInput>,
    ) -> Result<TeacherGetResponse> {
        let punch_report = get_teacher_attendance(pool, teacher, punch_get_input)
            .await
            .unwrap();
        Ok(TeacherGetResponse::PunchReport(Json(punch_report)))
    }

    /// Get general attendance
    #[oai(path = "/data/attendance", method = "post")]
    async fn get_general_attendance(
        &self,
        pool: Data<&DbPool>,
        punch_get_input: Json<PunchGetInput>,
    ) -> Result<GetResponse> {
        let punches = get_general_attendance(pool, punch_get_input).await.unwrap();
        Ok(GetResponse::PunchQuantity(Json(punches)))
    }

    /// Health check
    #[oai(path = "/health", method = "get")]
    #[allow(clippy::unused_async)]
    async fn health(&self) -> Result<HealthResponse> {
        tracing::info!("TESTE TESTE TESTE\n\n\n");
        Ok(HealthResponse::Health(Html(
            "<html><body><h1>It works!</h1></body></html>".to_string(),
        )))
    }

    #[oai(path = "/data/attendance/create", method = "post")]
    async fn hypermedia_post(
        &self,
        secrets: Data<&SecretCredentials>,
        auth: MyBasicAuthorization,
        pool: Data<&DbPool>,
        punch_create_hypermedia: Json<PunchCreateHypermedia>,
    ) -> Response<CreateResponse> {
        if auth.0.username != secrets.username || auth.0.password != secrets.password {
            tracing::info!("Not authenticated");
            return Response::new(CreateResponse::Unauthorized(PlainText(
                "Not authenticated".to_string(),
            )));
        }

        let today = Utc::now().naive_local().date();
        let date = match punch_create_hypermedia.date {
            Some(date) => {
                if date > today {
                    tracing::info!("Date {} is in the future. Today is {}", date, today);
                    return Response::new(CreateResponse::BadRequest(Json(MyError {
                        error: "Date is in the future".to_string(),
                    })));
                }
                date
            }
            None => today,
        };

        let level = punch_create_hypermedia.level.clone();
        let teacher = punch_create_hypermedia.teacher.clone();
        let new_punch = create_punch(pool, Path(level), Path(teacher), date)
            .await
            .unwrap();
        Response::new(CreateResponse::Punch(Json(new_punch)))
            .header("HX-Trigger", "post-attendance")
    }
}
