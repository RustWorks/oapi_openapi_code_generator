/// {{camelcase type}} parameters for {{snakecase operationId}} operation
#[derive(Serialize, Deserialize)]
pub struct {{camelcase type}} {
{{~#each parameters}}
    {{~#if (eq in ../type)}}
    {{#if description}}/// {{description}}{{/if}}
    {{#if (and (schema.pattern) (patterns ""))}} /// {{schema.pattern}}{{/if}}
    pub {{snakecase name}}: {{>data_type name=name required=required schema}},
    {{~/if}}
{{~/each}}
}
