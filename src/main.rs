mod data;
mod data_api;
mod hypermedia_api;
mod schema;
mod util;

use crate::data::{Levels, LevelsIter, Meses, MesesIter, Teachers, TeachersIter};
use crate::data_api::controller::DataApi;
use crate::hypermedia_api::controller::HypermediaApi;

use askama::Template;
use chrono::{Datelike, Month, Utc};
use poem::EndpointExt;
use poem::IntoResponse;
use poem::{
    endpoint::StaticFilesEndpoint,
    get, handler,
    middleware::Cors,
    middleware::Tracing,
    session::{CookieConfig, CookieSession, Session},
    Endpoint, Route,
};
use poem_openapi::{OpenApiService, OperationId, Tags};
use shuttle_poem::ShuttlePoem;
use shuttle_runtime::CustomError;
use shuttle_secrets::SecretStore;
use sqlx::{Executor, PgPool};
use std::env;
use strum::IntoEnumIterator;

pub type DbPool = PgPool;

#[derive(Clone, Debug)]
pub struct SecretCredentials {
    username: String,
    password: String,
}

#[derive(Tags)]
pub enum ApiTags {
    Data,
    Hypermedia,
}

#[derive(Template)]
#[template(path = "admin.html")]
struct AdminTemplate {
    auth: String,
    name: String,
    teachers: TeachersIter,
    levels: LevelsIter,
    meses: MesesIter,
    mes_atual: Meses,
}

impl Default for AdminTemplate {
    fn default() -> Self {
        Self {
            auth: "Basic".to_string(),
            name: "Shuttle".to_string(),
            teachers: Teachers::iter(),
            levels: Levels::iter(),
            meses: Meses::iter(),
            mes_atual: Meses::from_chrono_month(
                Month::try_from(u8::try_from(Utc::now().month()).unwrap()).unwrap(),
            ),
        }
    }
}

#[derive(Template)]
#[template(path = "guest.html")]
struct GuestTemplate {
    teachers: TeachersIter,
    meses: MesesIter,
    mes_passado: Meses,
}

impl Default for GuestTemplate {
    fn default() -> Self {
        Self {
            teachers: Teachers::iter(),
            meses: Meses::iter(),
            mes_passado: Meses::from_chrono_month(
                Month::try_from(u8::try_from(Utc::now().month()).unwrap() - 1).unwrap(),
            ),
        }
    }
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate {}

struct HtmlTemplate<T>(T);

#[handler]
async fn admin(session: &Session) -> impl IntoResponse {
    match session.get::<String>("username") {
        Some(name) => HtmlTemplate(AdminTemplate {
            auth: session.get::<String>("auth").unwrap(),
            name,
            ..Default::default()
        })
        .into_response(),
        None => HtmlTemplate(LoginTemplate {}).into_response(),
    }
}

#[handler]
async fn guest() -> impl IntoResponse {
    HtmlTemplate(GuestTemplate {
        ..Default::default()
    })
}

#[handler]
async fn login(session: &Session) -> poem::Response {
    match session.get::<String>("username") {
        Some(name) => HtmlTemplate(AdminTemplate {
            name,
            ..Default::default()
        })
        .into_response(),
        None => HtmlTemplate(LoginTemplate {}).into_response(),
    }
}

impl<T> IntoResponse for HtmlTemplate<T>
where
    T: Template + Send + Sync + 'static,
{
    fn into_response(self) -> poem::Response {
        let body = self.0.render().unwrap();
        poem::Response::builder()
            .content_type("text/html; charset=utf-8")
            .body(body)
    }
}

#[shuttle_runtime::main]
async fn poem(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> ShuttlePoem<impl poem::Endpoint> {
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "poem_openapi=debug,poem=debug,fernanda=debug");
    }

    pool.execute(include_str!("../schema.sql"))
        .await
        .map_err(CustomError::new)?;
    let secret = SecretCredentials {
        username: secret_store.get("USERNAME").unwrap(),
        password: secret_store.get("PASSWORD").unwrap(),
    };

    let api_service = OpenApiService::new((DataApi, HypermediaApi), "Attendance", "1.0.0")
        .server("https://pd-presenca.shuttleapp.rs/api");
    //.server("http://127.0.0.1:8000/api");
    let ui = api_service.swagger_ui();
    let route = Route::new()
        .nest("/api", api_service)
        .nest("/docs", ui)
        .nest(
            "/static",
            StaticFilesEndpoint::new("./css/").show_files_listing(),
        )
        .at("/", get(guest))
        .at("/admin", get(admin))
        .at("/login", get(login))
        .with(Cors::new())
        .with(CookieSession::new(CookieConfig::new()))
        .data(pool)
        .data(secret)
        .around(|ep, req| async move {
            let uri = req.uri().clone();
            let resp = ep.get_response(req).await;

            if let Some(operation_id) = resp.data::<OperationId>() {
                println!("[{}]{} {}", operation_id, uri, resp.status());
            } else {
                println!("{} {}", uri, resp.status());
            }

            Ok(resp)
        })
        .with(Tracing);

    Ok(route.into())
}
