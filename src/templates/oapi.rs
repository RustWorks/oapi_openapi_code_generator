#[allow(clippy::ptr_arg)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::unit_arg, clippy::redundant_clone)]

pub const VERSION: &str = "{{info.version}}";

/*******************************client code*******************************/

pub mod client {
    use url::Url;
    use std::time::Duration;
/* Reqwest's errors are bad-mannered and recurse on their source when displayed.
 * This behavior doesn't interact well with thiserror which also recurse on error's cause
 * when displayed. To prevent this issue, this wrapper hides the error's source from thiserror.
 */
pub struct ReqwestError(pub reqwest::Error);

impl std::error::Error for ReqwestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

impl From<reqwest::Error> for ReqwestError {
    fn from(err: reqwest::Error) -> Self {
        Self(err)
    }
}

impl std::fmt::Debug for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for ReqwestError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Clone)]
pub struct {{camelcase info.title "Client"}} {
    pub url: Url,
    pub client: reqwest::Client,
}

{{~#*inline "async_operation_fn"}}

    pub async fn {{snakecase operationId}}(
        &self,
        {{~#if ../parameters~}} parameters: &{{snakecase operationId}}::Parameters,{{/if}}
        {{~#if requestBody~}}
            {{~#with requestBody.content.[application/json]}}body: &{{snakecase ../operationId}}::Body,{{~/with}}
            {{~#with requestBody.content.[multipart/form-data]}}form: reqwest::multipart::Form,{{~/with}}
        {{/if~}}
    ) -> Result<super::{{snakecase operationId}}::Success, self::{{snakecase operationId}}::Error> {
        use {{snakecase ../operationId}}::*;
        let url = self.url.join(
            {{#if (has parameters "in" "path")~}}
            format!("{{@../key}}"
            {{~#each parameters}}
                {{~#if (eq in "path")}}, {{name}} = parameters.{{snakecase name}}{{/if}}
            {{~/each~}})
            {{~else~}}
            "{{@../key}}"
            {{~/if~}}
            .trim_start_matches('/')
        ).expect("url parse error");
        let response = self.client
            .{{operation_verb}}(url)
            {{#if (has parameters "in" "query")~}}
            .query(&parameters.query())
            {{~/if}}
            {{~#if requestBody}}
                {{~#with requestBody.content.[application/json]}}.json(&body){{~/with}}
                {{~#with requestBody.content.[multipart/form-data]}}.multipart(form){{~/with}}
            {{~/if}}
            .send().await.map_err(ReqwestError)?;
        match response.status().as_str() {
            {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (eq @key "204")}}
                "{{@key}}" => {
                    Ok(Success::{{camelcase "Status" @key}}(()))
                }
                {{~else~}}
                "{{@key}}" => {
                    {{~#if content}}
                        {{~#with content.[image/png]}}let response_body = response.json().await.map_err(ReqwestError)?;{{~/with}}
                        {{~#with content.[image/jpeg]}}let response_body = response.json().await.map_err(ReqwestError)?;{{~/with}}
                        {{~#with content.[text/plain]}}let response_body = response.text().await.map_err(ReqwestError)?;{{~/with}}
                        {{~#with content.[application/json]}}let response_body = response.json().await.map_err(ReqwestError)?;{{~/with}}
                    {{~else~}}
                        let response_body = ();
                    {{~/if}}
                    {{~#if (is_http_code_success @key)}}
                    Ok(Success::{{camelcase "Status" @key}}(response_body))
                    {{else}}
                    Err(Error::{{camelcase "Status" @key}}(response_body))
                    {{~/if}}
                }
                {{~/if}}
            {{~/if}}
            {{~/each}}
                _ => Err(Error::Unknown(response)),
        }
    }
{{~/inline}}

impl {{camelcase info.title "Client"}} {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).expect("cannot parse url");
        Self {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.client = reqwest::Client::builder().timeout(timeout).build().expect("bad client build");
        self
    }

    {{~#each paths}}
        {{~#with get}}{{~> async_operation_fn operation_verb="get"}}{{~/with}}
        {{~#with head}}{{~> async_operation_fn operation_verb="head"}}{{~/with}}
        {{~#with post}}{{~> async_operation_fn operation_verb="post"}}{{~/with}}
        {{~#with put}}{{~> async_operation_fn operation_verb="put"}}{{~/with}}
        {{~#with delete}}{{~> async_operation_fn operation_verb="delete"}}{{~/with}}
        {{~#with options}}{{~> async_operation_fn operation_verb="options"}}{{~/with}}
        {{~#with trace}}{{~> async_operation_fn operation_verb="trace"}}{{~/with}}
        {{~#with patch}}{{~> async_operation_fn operation_verb="patch"}}{{~/with}}
    {{~/each}}
}

{{~#*inline "shortcut_to_data_model"}}

pub mod {{snakecase operationId}} {
    pub use super::super::{{snakecase operationId}}::*;

    #[derive(Debug, thiserror::Error, displaydoc::Display)]
    pub enum Error {
        /// Request failed
        Client(#[from] super::ReqwestError),
        /// IO error occured while retrieving response body
        Io(#[from] std::io::Error),
        /// Request body serialization to JSON failed
        BodySerialization(#[from] serde_json::Error),
        /// Request parameters serialization failed
        ParametersSerialization(#[from] serde_urlencoded::ser::Error),
        /// Timeout occured during request
        Timeout(#[from] async_std::future::TimeoutError),
        {{~#each responses}}
        {{~#if (not (eq @key "default"))}}
        /// Status {{@key}} error: {0:?}
        {{camelcase "Status" @key}}({{camelcase "Status" @key}}),
        {{~/if}}
        {{~/each}}
        /// Unknown: {0:?}
        Unknown(reqwest::Response),
    }
}
{{~/inline}}

{{~#each paths}}
{{~#with get}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with head}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with post}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with put}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with delete}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with options}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with trace}}{{~> shortcut_to_data_model}}{{~/with}}
{{~#with patch}}{{~> shortcut_to_data_model}}{{~/with}}
{{~/each}}

}

/*******************************models code*******************************/

pub mod components {
{{~#with components}}
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};
        use serde_json;

        {{~#each schemas}}
            {{>schema name=@key this}}
        {{~/each}}
    }
{{~/with}}
}
{{#each paths}}
    {{~>operation_types get noBody=true}}
    {{~>operation_types head noBody=true}}
    {{~>operation_types post}}
    {{~>operation_types put}}
    {{~>operation_types delete}}
    {{~>operation_types options}}
    {{~>operation_types trace}}
    {{~>operation_types patch}}
{{~/each}}


/*******************************server code*******************************/
pub mod server {

use actix_web::{web::*, Responder, HttpResponse, dev::HttpResponseBuilder, http::StatusCode};
use async_trait::async_trait;

{{~#*inline "operation_fn_trait"}}

    async fn {{snakecase operationId}}(
        &self,
        _parameters: super::{{snakecase operationId}}::Parameters,
        {{#unless noBody~}} _body: super::{{snakecase operationId}}::Body, {{~/unless}}
    ) -> Result<super::{{snakecase operationId}}::Success, super::{{snakecase operationId}}::Error<Self::Error>> {
        unimplemented!()
    }
{{~/inline}}

{{~#*inline "auth_fn_trait"}}
    async fn {{snakecase key}}(
        &self,
        _request: actix_web::dev::ServiceRequest,
    ) -> Result<(), actix_web::error::Error> {
        unimplemented!();
    }
{{~/inline}}

#[async_trait(?Send)]
pub trait {{camelcase info.title}} {
    type Error: std::error::Error;
{{~#each paths}}
    {{~#with get}}{{~> operation_fn_trait noBody=true}}{{~/with}}
    {{~#with head}}{{~> operation_fn_trait noBody=true}}{{~/with}}
    {{~#with post}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with put}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with delete}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with options}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with trace}}{{~> operation_fn_trait}}{{~/with}}
    {{~#with patch}}{{~> operation_fn_trait}}{{~/with}}
{{~/each}}

{{#each components.securitySchemes as | obj  key |}}
    {{~#with key}}{{~> auth_fn_trait}}{{~/with}}
{{~/each}}
}
{{#*inline "operation_fn"}}
{{#if summary}}/// {{summary}}{{/if}}
{{~#if description}}/// {{description}}{{/if}}
async fn {{snakecase operationId}}<Server: {{camelcase title}}>(
    server: Data<Server>,{{!-- {{~#if parameters}} --}}
    {{~#if (has parameters "in" "query")~}}
    query: Query<super::{{snakecase operationId}}::Query>,
    {{~/if}}
    {{~#if (has parameters "in" "path")~}}
    path: Path<{{snakecase operationId}}::Path>,
    {{~/if}}

    {{~#if (and requestBody (not noBody))}}
        {{~#with requestBody.content.[application/json]}}
            body: Json<super::{{snakecase ../operationId}}::Body>,
        {{~/with}}
        {{~#with requestBody.content.[multipart/form-data]}}
            mut payload: Multipart,
        {{~/with}}
    {{~/if}}
) -> impl Responder {
    use super::{{snakecase operationId}}::*;
    let parameters = Parameters::new(
        {{~#if (has parameters "in" "query")~}}query.into_inner(),{{~/if}}
        {{~#if (has parameters "in" "path")~}}path.into_inner(),{{~/if}}
    );
    {{~#unless noBody}}
        {{~#if requestBody}}

            {{~#with requestBody.content.[application/json]}}
                let body = body.into_inner();
            {{~/with}}

            {{~#with requestBody.content.[multipart/form-data]}}
                let mut data = HashMap::new();

                while let Ok(Some(mut field)) = payload.try_next().await {
                    let content_disposition = field.content_disposition().unwrap();
                    let field_name = content_disposition.get_name().unwrap().to_string();
                    let mut buffer = vec![];
                    while let Some(chunk) = field.next().await {
                        buffer.extend_from_slice(chunk.unwrap().as_ref());
                    }
                    data.insert(
                        field_name,
                        buffer,
                    );
                }
                let body = match {{snakecase ../operationId}}::Body::try_from(data) {
                    Ok(body) => body,
                    Err(err) => return HttpResponse::InternalServerError().body(err)
                };
            {{~/with}}

        {{~else~}}
            let body = {{snakecase operationId}}::Body {};
        {{~/if}}
    {{~/unless}}

    match server.{{snakecase operationId}}(parameters {{~#unless noBody}}, body{{/unless}}).await {
        {{~#each responses}}
            {{~#if (not (eq @key "default"))}}
                {{~#if (is_http_code_success @key)}}
                    {{~#if content}}

                        {{~#with content.[image/png]}}
                            Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("image/png").body(response),
                        {{~/with}}

                        {{~#with content.[image/jpeg]}}
                            Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("image/jpeg").body(response),
                        {{~/with}}

                        {{~#with content.[text/plain]}}
                            Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("text/plain").body(response),
                        {{~/with}}

                        {{~#with content.[application/json]}}
                            Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).json(response),
                        {{~/with}}

                    {{~else~}}
                        Ok(Success::{{camelcase "Status" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(response),
                    {{~/if}}
                {{~else~}}
                    {{~#if content}}
                        {{~#with content.[text/plain]}}
                            Err(Error::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("text/plain").body(response),
                        {{~/with}}

                        {{~#with content.[application/json]}}
                            Err(Error::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).json(response),
                        {{~/with}}

                    {{~else~}}
                        Err(Error::{{camelcase "Status" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(response),
                    {{~/if}}
                {{~/if}}
            {{~/if}}
        {{~/each}}
        Err(Error::Unknown(err)) => HttpResponse::InternalServerError().body(err_to_string(&err)),
    }
}
{{~/inline}}

fn err_to_string(err: &dyn std::error::Error) -> String {
    let mut errors_str = Vec::new();
    let mut current_err = err.source();
    while let Some(err) = current_err {
        errors_str.push(err.to_string());
        current_err = err.source();
    }
    format!("error: {}\n\ncaused by:\n\t{}", err, errors_str.as_slice().join("\n\t"))
}

{{#each paths}}
    {{~#with get}}{{~> operation_fn title=../../info.title noBody=true}}{{~/with}}
    {{~#with head}}{{~> operation_fn title=../../info.title noBody=true}}{{~/with}}
    {{~#with post}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with put}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with delete}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with options}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with trace}}{{~> operation_fn title=../../info.title}}{{~/with}}
    {{~#with patch}}{{~> operation_fn title=../../info.title}}{{~/with}}
{{~/each}}

pub fn config<Server: {{camelcase info.title}} + Send + Sync + Clone + 'static>(
    app: &mut ServiceConfig,
) {
    app
    {{~#each paths}}
        .service(
            resource("{{@key}}")
                {{~#with get}}
                .route(get().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with head}}
                .route(head().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with post}}
                .route(post().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with put}}
                .route(put().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with delete}}
                .route(delete().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with options}}
                .route(options().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with trace}}
                .route(trace().to({{snakecase operationId}}::<Server>))
                {{~/with}}
                {{~#with patch}}
                .route(patch().to({{snakecase operationId}}::<Server>))
                {{~/with}}
        )
    {{~/each}};
}

}

pub trait OpenapiSerialization {
  fn serialize(self: &Self) -> Option<String>;
}

impl OpenapiSerialization for i32 {
  fn serialize(self: &Self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for i64 {
  fn serialize(self: &Self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for f32 {
  fn serialize(self: &Self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for f64 {
  fn serialize(self: &Self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for String {
  fn serialize(self: &Self) -> Option<String> {
    Some(self.clone())
  }
}

impl<T: OpenapiSerialization> OpenapiSerialization for Option<T> {
  fn serialize(self: &Self) -> Option<String> {
    self.as_ref().map(|n| match n.serialize() {
      Some(n) => n,
      None => "".to_string(),
    })
  }
}
