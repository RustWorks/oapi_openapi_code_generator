#[allow(clippy::ptr_arg)]
#[allow(clippy::clone_on_copy)]
#[allow(clippy::unit_arg, clippy::redundant_clone)]

{{~#if info.version}}
pub const VERSION: &str = "{{info.version}}";
{{~/if}}

{{#if info.title}}
/*******************************client code*******************************/

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

{{/if}}

{{#if components.schemas}}
/*******************************models code*******************************/

#[allow(unused_assignments, unused_imports, unused_variables)]
pub mod components {
{{~#with components}}
    #[allow(unused_assignments, unused_imports, unused_variables)]
    pub mod schemas {
        use super::super::components;
        use serde::{Deserialize, Serialize};

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
{{/if}}

{{#if info.title}}
/*******************************server code*******************************/
#[allow(unused_assignments, unused_imports, unused_variables)]
pub mod server {

use actix_web::{web::*, Responder, HttpResponse, HttpResponseBuilder, http::StatusCode, error::InternalError};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// This structure is aiming to deliver the already serialized content, so we can avoid serializing
/// it twice and wasting time, after we've passed the separate parts to the endpoint method.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct PayloadRawParts {
    pub raw_uri: String,
    pub raw_body: Option<String>,
    // NOTE: Questionable if this should be in that struct at all. It could potentially result into
    // doubled allocation, because `Body::try_from` is consuming.
    // pub raw_data: Option<HashMap<String, Vec<Bytes>>>
}

{{~#*inline "operation_fn_trait"}}
    async fn {{snakecase operationId}}(
        &self,
        payload: PayloadRawParts,
        request: {{~#if security}}Self::AuthorizedData{{~else~}}HttpRequest{{~/if}},
        parameters: super::{{snakecase operationId}}::Parameters,
        {{#unless noBody~}} body: super::{{snakecase operationId}}::Body, {{~/unless}}
    ) -> Result<super::{{snakecase operationId}}::Success, super::{{snakecase operationId}}::Error<Self::Error>>;
{{~/inline}}

{{~#*inline "auth_fn_trait"}}
    async fn {{snakecase key}}(
        &self,
        request: actix_web::HttpRequest,
        payload: &PayloadRawParts,
    ) -> Result<Self::AuthorizedData, Self::Error>;
{{~/inline}}

#[async_trait(?Send)]
pub trait {{camelcase info.title}} {
    {{~#if components.securitySchemes }}
    type AuthorizedData;
    {{~/if}}
    type Error: std::error::Error;

    fn handle_error<E: std::error::Error>(e: &E) -> actix_web::body::AnyBody;

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
    request: actix_web::HttpRequest,
    server: Data<Server>,{{!-- {{~#if parameters}} --}}
    {{~#if (has parameters "in" "query")~}}
    query: Query<super::{{snakecase operationId}}::Query>,
    {{~/if}}
    {{~#if (has parameters "in" "path")~}}
    path: Path<super::{{snakecase operationId}}::Path>,
    {{~/if}}

    {{~#if (and requestBody (not noBody))}}
        {{~#with requestBody.content.[application/x-www-form-urlencoded]}}
            body: Bytes,
        {{~/with}}
        {{~#with requestBody.content.[application/json]}}
            body: Bytes,
        {{~/with}}
        {{~#with requestBody.content.[multipart/form-data]}}
            mut payload: Multipart,
        {{~/with}}
    {{~/if}}
) -> impl Responder {
    use super::{{snakecase operationId}}::*;

    let parameters = match Parameters::new(
        {{~#if (has parameters "in" "query")~}}query.into_inner(),{{~/if}}
        {{~#if (has parameters "in" "path")~}}path.into_inner(),{{~/if}}
    ) {
        Ok(x) => x,
        Err(err) => return HttpResponse::BadRequest()
            {{~#if (and requestBody (not noBody))}}
            .content_type(
                {{~#with requestBody.content.[application/x-www-form-urlencoded]}}
                    "application/x-www-form-urlencoded"
                {{~/with}}
                {{~#with requestBody.content.[application/json]}}
                    "application/json"
                {{~/with}}
                {{~#with requestBody.content.[multipart/form-data]}}
                    "multipart/form-data"
                {{~/with}}
            )
            {{~/if}}
            .body(<Server as {{camelcase title}}>::handle_error(e)),
    };

    {{~#unless noBody}}
        {{~#if requestBody}}

            {{~#with requestBody.content.[application/json]}}
                let body_str = String::from_utf8_lossy(&body);
                let body = match serde_json::from_str(body_str.as_ref()) {
                    Ok(body) => body,
                    Err(e) => return HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(<Server as {{camelcase @../../title}}>::handle_error(e)),
                };
            {{~/with}}

            {{~#with requestBody.content.[application/x-www-form-urlencoded]}}
                let body_str = String::from_utf8_lossy(&body);
                let body = match serde_urlencoded::from_str(body_str.as_ref()) {
                    Ok(body) => body,
                    Err(e) => return HttpResponse::BadRequest()
                        .content_type("application/x-www-form-urlencoded")
                        .body(<Server as {{camelcase @../../title}}>::handle_error(e)),
                };
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

    let payload_raw = PayloadRawParts {
        raw_uri: request.uri().to_string(),
        raw_body:
            {{~#if (and requestBody (not noBody))}}
                {{~#with requestBody.content.[application/json]}}
                Some(body_str.to_string())
                {{~/with}}

                {{~#with requestBody.content.[application/x-www-form-urlencoded]}}
                Some(body_str.to_string())
                {{~/with}}
            {{~else~}}
                None
            {{~/if}}
        };

    {{~#if security }}
        {{~#each security as |obj|}}
            {{~#each obj as |o  key|}}
                let request = match server.{{snakecase key}}(request, &payload_raw).await {
                    Ok(auth_data) => auth_data,
                    Err(e) => return HttpResponse::Unauthorized()
                        .content_type("application/json")
                        .body(<Server as {{camelcase @../../../title}}>::handle_error(e)),
                };
            {{~/each}}
        {{~/each}}
    {{~/if}}

    match server.{{snakecase operationId}}(payload_raw, request, parameters {{~#unless noBody}}, body{{/unless}}).await {
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
                            Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).json(&response),
                        {{~/with}}

                        {{~#with content.[application/yaml]}}
                            Ok(Success::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("application/yaml").body(response),
                        {{~/with}}

                    {{~else~}}
                        Ok(Success::{{camelcase "Status" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(&response),
                    {{~/if}}
                {{~else~}}
                    {{~#if content}}
                        {{~#with content.[text/plain]}}
                            Err(Error::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).content_type("text/plain").body(response),
                        {{~/with}}

                        {{~#with content.[application/json]}}
                            Err(Error::{{camelcase "Status" @../key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@../key}}).unwrap()).json(&response),
                        {{~/with}}

                    {{~else~}}
                        Err(Error::{{camelcase "Status" @key}}(response)) => HttpResponseBuilder::new(StatusCode::from_u16({{@key}}).unwrap()).json(&response),
                    {{~/if}}
                {{~/if}}
            {{~/if}}
        {{~/each}}
        Err(Error::Unknown(err)) =>
            HttpResponse::Unauthorized()
                .content_type("application/json")
                .body(<Server as {{camelcase title}}>::handle_error(e)),
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

pub fn config<Server: {{camelcase info.title}} + 'static>(
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
    {{~/each}}
        .app_data(actix_web::web::JsonConfig::default()
            .error_handler(|err, _| {
                let mut response = HttpResponseBuilder::new(StatusCode::BAD_REQUEST);
                response.body(err_to_string(&err));
                InternalError::from_response(err, response.into()).into()
            })
        );
}

}

pub trait OpenapiSerialization {
  fn serialize(&self) -> Option<String>;
}

impl OpenapiSerialization for i32 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for i64 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for f32 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for f64 {
  fn serialize(&self) -> Option<String> {
    Some(format!("{:?}", self))
  }
}

impl OpenapiSerialization for String {
  fn serialize(&self) -> Option<String> {
    Some(self.clone())
  }
}

impl<T: OpenapiSerialization> OpenapiSerialization for Option<T> {
  fn serialize(&self) -> Option<String> {
    self.as_ref().map(|n| match n.serialize() {
      Some(n) => n,
      None => "".to_string(),
    })
  }
}
{{/if}}
