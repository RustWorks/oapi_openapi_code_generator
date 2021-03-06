{{~#if operationId}}
pub mod {{snakecase operationId}} {
    use super::components;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, value::Value};

    {{#with requestBody}}
    {{>schema_example name="Body" description=description content.[application/json].schema}}
    {{>schema_example name="Body" description=description content.[application/x-www-form-urlencoded].schema}}
    {{~/with}}

    {{~#each responses}}
      {{~#if (not (eq @key "default"))}}
        {{>schema_example name=(camelcase "Response" @key) description=description content.[application/json].schema}}
      {{~/if}}
    {{~/each}}
}
{{/if}}