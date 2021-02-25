{{~>subtypes name=name}}
{{~#if description~}}
/// {{description}}
{{/if}}
{{~#if [$ref]~}}
pub type {{camelcase name suffix}} = {{>data_type required="true"}};
{{~else}}
    {{~#if (eq type "object")~}}
        {{~#if properties~}}

            {{~#each properties}}
            {{~#if (and (pattern) (patterns ""))}}

            lazy_static::lazy_static! {
                static ref {{shoutysnakecase ../name @key ../suffix}}_PATTERN: regex::Regex
                    = regex::Regex::new("{{pattern}}").expect("Regex for `{{../name}}{{../suffix}}`'s parameter `{{@key}}`");
            }

            fn deserialize_{{snakecase ../name @key ../suffix}}<'de, D>(d: D)
            {{~#if (has ../required @key)}}
            -> Result<String, D::Error>
            {{~else}}
            -> Result<Option<String>, D::Error>
            {{~/if}}
            where
                D: serde::de::Deserializer<'de>,
            {
                {{~#if (has ../required @key)}}
                let res = String::deserialize(d)?;

                if !{{shoutysnakecase ../name @key ../suffix}}_PATTERN.is_match(&res) {
                    return Err(serde::de::Error::custom("`{{../name}}{{../suffix}}`'s parameter `{{@key}}` does not match its required pattern"));
                }
                {{~else}}
                let res = Option::<String>::deserialize(d)?;

                if let Some(res) = res.as_ref() {
                    if !{{shoutysnakecase ../name @key ../suffix}}_PATTERN.is_match(&res) {
                        return Err(serde::de::Error::custom("`{{../name}}{{../suffix}}`'s optional parameter `{{@key}}` is present but does not match its required pattern"));
                    }
                }
                {{~/if}}

                Ok(res)
            }
            {{~/if}}
            {{~/each}}

            #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
            pub struct {{camelcase name suffix}} {
            {{~#each properties}}
                #[serde(rename = "{{@key}}")]
                {{#if (and (pattern) (patterns ""))}}
                #[serde(default)]
                #[serde(deserialize_with = "deserialize_{{snakecase ../name @key ../suffix}}")]
                {{/if}}
                {{~#if (has ../required @key)}}
                pub {{sanitize (snakecase @key)}}: {{>data_type name=(camelcase ../name @key) required="true"}},
                {{~else}}
                #[serde(skip_serializing_if = "Option::is_none")]
                pub {{sanitize (snakecase @key)}}: {{>data_type name=(camelcase ../name @key)}},
                {{~/if}}
            {{~/each}}
            {{~#if additionalProperties}}
                pub properties: serde_json::Map<String, serde_json::Value>,
            {{~/if}}
            }

            {{~#if (has this.[x-tags] "multipart")}}
            impl TryFrom<HashMap<String, Vec<u8>>> for {{camelcase name suffix}} {
                type Error = &'static str;

                fn try_from(mut data: HashMap<String, Vec<u8>>) -> Result<Self, Self::Error> {
                    Ok({{camelcase name suffix}} {
                        {{~#each properties}}
                            {{sanitize (snakecase @key)}}: data.remove("{{snakecase @key}}").ok_or_else(|| "missing field {{sanitize (snakecase @key)}}")?,
                        {{~/each}}
                    })
                }
            }
            {{~/if}}

        {{~else~}}
            {{~#if additionalProperties}}
                pub type {{camelcase name suffix}} = serde_json::Map<String, serde_json::Value>;
            {{~/if}}
        {{~/if}}
    {{~else~}}
        {{~#if (and (eq type "string") enum (not format))}}
            #[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
            pub enum {{camelcase ../name @key suffix}} {
                {{~#each enum}}
                #[serde(rename = "{{this}}")]
                {{camelcase this}},
                {{~/each}}
            }

            impl std::fmt::Display for {{camelcase ../name @key suffix}} {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}",
                        match self {
                            {{~#each enum}}
                            {{camelcase ../name @key suffix}}::{{camelcase this}} => "{{this}}",
                            {{~/each}}
                        }
                    )
                }
            }

        {{~else~}}
            {{~#if [oneOf]~}}
                #[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
                #[serde(untagged)]
                pub enum {{camelcase name suffix}} {
                {{~#each oneOf}}
                    Option{{@index}}({{>data_type required="true"}}),
                {{~/each}}
                }
            {{~else~}}
                pub type {{camelcase name suffix}} =
                    {{~#if type}} {{>data_type required="true"}}
                    {{~else}} ()
                    {{~/if}};
            {{/if}}
        {{/if}}
    {{~/if}}
{{~/if}}
