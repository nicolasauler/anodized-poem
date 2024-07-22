use crate::data::{DiasDaSemana, Levels};
use crate::data_api::service::{get_general_attendance, get_teacher_attendance, sign_in};
use crate::schema::{
    PunchGetInput, PunchGetInputHypermedia, PunchGetTeacherHypermedia, SignInData, SignInResponse,
};
use crate::util::{get_first_day_from_month, get_last_day_from_month};
use crate::{ApiTags, DbPool, SecretCredentials};

use chrono::{Datelike, NaiveDate};
use poem::{session::Session, web::Data};
use poem_openapi::{
    param::Path,
    payload::PlainText,
    payload::{Html, Json, Response},
    OpenApi,
};

pub struct HypermediaApi;

#[OpenApi(tag = "ApiTags::Hypermedia")]
impl HypermediaApi {
    /// Get general attendance as html table
    #[oai(path = "/hypermedia/general_attendance", method = "post")]
    async fn general_attendance_table(
        &self,
        pool: Data<&DbPool>,
        punch_get_input_hyper: Json<PunchGetInputHypermedia>,
    ) -> Html<String> {
        let mes = &punch_get_input_hyper.mes;
        let ano_string: &str = &punch_get_input_hyper.ano;
        let ano: i32 = ano_string.parse().unwrap();
        let start_date = Some(get_first_day_from_month(
            mes.to_chrono_month().number_from_month(),
            ano,
        ));
        let end_date = Some(get_last_day_from_month(
            mes.to_chrono_month().number_from_month(),
            ano,
        ));
        let punch_get_input = PunchGetInput {
            start_date,
            end_date,
        };
        let punches = get_general_attendance(pool, Json(punch_get_input))
            .await
            .unwrap();

        Html(
            punches
                .iter()
                .map(|punch| {
                    format!(
                        "<tr><td>{}</td><td>{}</td></tr>",
                        punch.teacher, punch.n_classes
                    )
                })
                .collect::<Vec<String>>()
                .join("\n"),
        )
    }

    /// Get teacher attendance as html table
    #[oai(path = "/hypermedia/teacher_attendance", method = "post")]
    async fn teacher_attendance_table(
        &self,
        pool: Data<&DbPool>,
        punch_get_teacher_hypermedia: Json<PunchGetTeacherHypermedia>,
    ) -> Html<String> {
        let mes = &punch_get_teacher_hypermedia.mes;
        let ano: &str = &punch_get_teacher_hypermedia.ano;
        let ano: i32 = ano.parse().unwrap();
        let start_date = get_first_day_from_month(mes.to_chrono_month().number_from_month(), ano);
        let end_date = get_last_day_from_month(mes.to_chrono_month().number_from_month(), ano);
        let punch_get_input = PunchGetInput {
            start_date: Some(start_date),
            end_date: Some(end_date),
        };
        let punch_reports = get_teacher_attendance(
            pool,
            Path(punch_get_teacher_hypermedia.teacher.clone()),
            Json(punch_get_input),
        )
        .await
        .unwrap();

        let punches = punch_reports.punches;
        let mut punches_by_date = std::collections::BTreeMap::<NaiveDate, Vec<Levels>>::new();
        for punch in punches {
            if let Some(punchs) = punches_by_date.get_mut(&punch.date) {
                punchs.push(punch.level);
            } else {
                punches_by_date.insert(punch.date, vec![punch.level]);
            }
        }

        Html(
            format!(
                "<div class=\"overflow-auto\"> <h4 id=\"total\" >Total de aulas no período: {}</h4><table class=\"striped\" role=\"grid\"><thead><tr><th>Data</th><th>Dia da semana</th><th>Aula</th><th>Aula</th><th>Aula</th></tr></thead><tbody id=\"table-results-specific\">{}</tbody></table></div>",
                punch_reports.n_classes,
                punches_by_date
                    .iter()
                    .map(|(date, levels)| {
                        format!(
                            "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                            date,
                            DiasDaSemana::from_chrono_weekday(date.weekday()),
                            if let Some(level) = levels.first() {
                                level.to_string()
                            } else {
                                String::new()
                            },
                            if let Some(level) = levels.get(1) {
                                level.to_string()
                            } else {
                                String::new()
                            },
                            if let Some(level) = levels.get(2) {
                                level.to_string()
                            } else {
                                String::new()
                            },
                        )
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
        )
    }

    /// Get sign in page
    #[oai(path = "/signin", method = "post")]
    async fn signin(
        &self,
        secrets: Data<&SecretCredentials>,
        session: &Session,
        sign_in_data: Json<SignInData>,
    ) -> Response<SignInResponse> {
        if sign_in(&secrets.username, &secrets.password, session, sign_in_data).await {
            Response::new(SignInResponse::Redirect(PlainText("admin".to_string())))
                .header("HX-Redirect", "/admin")
        } else {
            Response::new(SignInResponse::Redirect(PlainText("login".to_string())))
                .header("HX-Redirect", "/login")
        }
    }
}
