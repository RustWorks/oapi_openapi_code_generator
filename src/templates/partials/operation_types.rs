{{~#if operationId}}
#[allow(unused_assignments, unused_imports, unused_variables)]
pub mod {{snakecase operationId}} {
    use super::components;
    use serde::{Deserialize, Serialize};

    {{#each parameters}}
        {{~>schema name=name schema}}
    {{~/each}}

    {{~#if parameters}}
    lazy_static::lazy_static! {
    {{~#each parameters}}
    {{~#if (and (schema.pattern) (patterns ""))}}
        static ref {{shoutysnakecase ../operationId}}_{{shoutysnakecase name}}_PATTERN: regex::Regex
            = regex::Regex::new("{{schema.pattern}}").expect("Regex for `{{../operationId}}`'s parameter `{{name}}`");
    {{~/if}}
    {{~/each}}
    }
    {{~/if}}

    {{~#if parameters}}
    /// Parameters for the `{{snakecase operationId}}` operation
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Parameters {
    {{~#each parameters}}
        {{#if description}}/// {{description}}{{/if}}
        pub {{snakecase name}}: {{>data_type name=name required=required schema}},
    {{~/each}}
    }
    {{~else}}
    #[derive(Serialize)]
    pub struct Parameters;
    {{~/if}}

    impl Parameters {
        {{~#unless parameters}}
        #[allow(clippy::new_without_default)]
        {{~/unless}}
        pub fn new(
            {{~#if (has parameters "in" "query")~}}query: Query,{{~/if}}
            {{~#if (has parameters "in" "path")~}}path: Path,{{~/if}}
        ) -> Result<Self, serde::de::value::Error> {
            Ok(Self {
            {{~#each parameters}}
                {{snakecase name}}:
                    {{~#if (and (schema.pattern) (patterns ""))}}
                    {{~#if (eq in "query")}} {
                        {{~#if required}}
                        if !{{shoutysnakecase ../operationId}}_{{shoutysnakecase name}}_PATTERN.is_match(&query.{{snakecase name}}) {
                            return Err(serde::de::Error::custom("`{{../operationId}}`'s parameter `{{name}}` does not match its required pattern"));
                        }
                        {{~else}}
                        if let Some(res) = query.{{snakecase name}}.as_ref() {
                            if !{{shoutysnakecase ../operationId}}_{{shoutysnakecase name}}_PATTERN.is_match(&res) {
                                return Err(serde::de::Error::custom("`{{../operationId}}`'s optional parameter `{{name}}` is present but does not match its required pattern"));
                            }
                        }
                        {{~/if}}

                        query.{{snakecase name}}
                    }, {{~/if}}
                    {{~#if (eq in "path")}} {
                        if !{{shoutysnakecase ../operationId}}_{{shoutysnakecase name}}_PATTERN.is_match(&path.{{snakecase name}}) {
                            return Err(serde::de::Error::custom("`{{../operationId}}`'s parameter `{{snakecase name}}` does not match its required pattern"));
                        }

                        path.{{snakecase name}}
                    }, {{~/if}}
                    {{~else}}
                    {{~#if (eq in "query")}} query.{{snakecase name}}, {{~/if}}
                    {{~#if (eq in "path")}} path.{{snakecase name}}, {{~/if}}
                    {{~/if}}
            {{~/each}}
            })
        }

        {{#if (has parameters "in" "query")~}}
        pub fn query(&self) -> Query {
            Query {
            {{~#each parameters}}
                {{~#if (eq in "query")}}{{snakecase name}}: self.{{snakecase name}}.clone(),{{~/if}}
            {{~/each}}
            }
        }
        {{~/if}}

        {{#if (has parameters "in" "path")~}}
        pub fn path(&self) -> Path {
            Path {
            {{~#each parameters}}
                {{~#if (eq in "path")}}{{snakecase name}}: self.{{snakecase name}}.clone(),{{~/if}}
            {{~/each}}
            }
        }
        {{~/if}}
    }

    {{~#if (has parameters "in" "query")~}}
    {{>parameter_type type="query"}}
    {{~/if}}

    {{~#if (has parameters "in" "path")~}}
    {{>parameter_type type="path"}}
    {{~/if}}

    {{#unless noBody}}
        {{~#if requestBody}}
            {{~#with requestBody.content.[application/x-www-form-urlencoded]}}{{~>schema name="Body" description=../description schema}}{{~/with}}
            {{~#with requestBody.content.[application/json]}}{{~>schema name="Body" description=../description schema}}{{~/with}}
            {{~#with requestBody.content.[multipart/form-data]}}{{~>schema name="Body" description=../description schema}}{{~/with}}
        {{~else~}}
            #[derive(Serialize, Deserialize)]
            pub struct Body;
        {{~/if}}
    {{~/unless}}

    #[derive(Debug)]
    pub enum Response<T> {
    {{~#each responses}}
        {{~#if (not (eq @key "default"))}}
        {{camelcase "Response" @key}}({{camelcase "Response" @key}}),
        {{~/if}}
    {{~/each}}
        Unspecified(T),
    }

    {{#each responses}}
        {{~#if (not (eq @key "default"))}}
            {{~#with content.[image/png]}}{{~>schema name=(camelcase "Response" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[image/jpeg]}}{{~>schema name=(camelcase "Response" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[text/plain]}}{{~>schema name=(camelcase "Response" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[application/json]}}{{~>schema name=(camelcase "Response" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[application/yaml]}}{{~>schema name=(camelcase "Response" @../key) description=../description schema}}{{~/with}}
        {{~/if }}
        {{~#if (not content)}}
            {{~>schema name=(camelcase "Response" @key) description=description}}
        {{~/if }}
    {{~/each}}

    #[derive(Debug)]
    pub enum Success {
    {{~#each responses}}
        {{~#if (is_http_code_success @key)}}
        {{camelcase "Status" @key}}({{camelcase "Status" @key}}),
        {{~/if}}
    {{~/each}}
    }

    #[derive(Debug)]
    pub enum Error<T: std::fmt::Debug> {
    {{~#each responses}}
        {{~#if (not (or (eq @key "default") (is_http_code_success @key)))}}
        {{camelcase "Status" @key}}({{camelcase "Status" @key}}),
        {{~/if}}
    {{~/each}}
        Unknown(T),
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::fmt::Display for Error<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                {{~#each responses}}
                    {{~#if (not (or (eq @key "default") (is_http_code_success @key)))}}
                    Self::{{camelcase "Status" @key}}(status) => write!(f, "{{snakecase "Status"}} {{@key}}: {:?}", status),
                    {{~/if}}
                {{~/each}}
                Self::Unknown(response) => write!(f, "Unspecified response: `{}`", response),
            }
        }
    }

    impl<T: std::fmt::Debug + std::fmt::Display> std::error::Error for Error<T> {}

    {{#each responses}}
        {{~#if (not (eq @key "default"))}}
            {{~#with content.[image/png]}}{{~>schema name=(camelcase "Status" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[image/jpeg]}}{{~>schema name=(camelcase "Status" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[text/plain]}}{{~>schema name=(camelcase "Status" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[application/json]}}{{~>schema name=(camelcase "Status" @../key) description=../description schema}}{{~/with}}
            {{~#with content.[application/yaml]}}{{~>schema name=(camelcase "Status" @../key) description=../description schema}}{{~/with}}
        {{~/if }}
        {{~#if (not content)}}
            {{~>schema name=(camelcase "Status" @key) description=description}}
        {{~/if }}
    {{~/each}}
}
{{/if}}
