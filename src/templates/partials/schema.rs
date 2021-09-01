{{~>subtypes name=name}}
{{~#if description~}}
/// {{description}}
{{/if}}
{{~#if [$ref]~}}
pub type {{camelcase name suffix}} = {{>data_type required="true"}};
{{~else}}
    {{~#if (eq type "object")~}}
        {{~#if properties~}}

            {{~#if (not_empty (fetch_patterns properties))}}
                lazy_static::lazy_static! {
                {{~#each (fetch_patterns properties)}}
                    static ref {{shoutysnakecase ../name @key ../suffix}}_PATTERN: regex::Regex
                            = regex::Regex::new("{{this}}").expect("Regex for `{{../name}}{{../suffix}}`'s parameter `{{@key}}`");
                {{~/each}}
                }
            {{~/if}}

            {{~#each properties}}

            {{~#if (or
                (or
                    (and (pattern) (patterns ""))
                    (and
                         (or
                            (eq type "string")
                            (or (eq type "integer") (eq type "number"))
                         )
                         (or
                            (not_empty minLength)
                            (not_empty maxLength)
                         )
                    )
                )
                (or
                    (and
                         (or (eq type "integer") (eq type "number"))
                         (or (not_empty minimum) (not_empty maximum))
                    )
                    (and
                         (eq type "array")
                         (or
                            (not_empty (fetch_patterns items))
                            (or (not_empty minItems) (not_empty maxItems))
                         )
                    )
                )
            )}}

            fn deserialize_{{snakecase ../name @key ../suffix}}<'de, D>(d: D)
            -> Result<
                {{~#if (not (has ../required @key))}}
                Option<
                {{~/if}}
                    {{~#if (eq type "string")}}String{{~/if}}
                    {{~#if (eq type "integer")}}i64{{~/if}}
                    {{~#if (eq type "number")}}i64{{~/if}}
                    {{~#if (eq type "array")}}{{>data_type name=(camelcase ../name @key) required="true"}}{{~/if}}
                {{~#if (not (has ../required @key))}}
                >
                {{~/if}}
                , D::Error>
            where
                D: serde::de::Deserializer<'de>,
            {
                {{~#if (has ../required @key)}}
                let res: {{>data_type name=(camelcase ../name @key) required="true"}} =
                            {{~#if (eq type "string")}}String{{~/if}}
                            {{~#if (eq type "integer")}}i64{{~/if}}
                            {{~#if (eq type "number")}}i64{{~/if}}
                            {{~#if (eq type "array")}}Vec::<_>{{~/if}}::deserialize(d)?;
                {{~else}}
                let res: {{>data_type name=(camelcase ../name @key)}} = Option::<
                {{~#if (eq type "string")}}String{{~/if}}
                {{~#if (eq type "integer")}}i64{{~/if}}
                {{~#if (eq type "number")}}i64{{~/if}}
                {{~#if (eq type "array")}}Vec<_>{{~/if}}
                >::deserialize(d)?;
                if let Some(res) = res.as_ref() {
                {{~/if}}
                {{>validation object=this name=../name key=@key suffix=../suffix var_name="res"}}
                {{~#if (not (has ../required @key))}}
                }
                {{~/if}}

                Ok(res)
            }
            {{~/if}}
            {{~/each}}

            #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
            #[serde(deny_unknown_fields)]
            pub struct {{camelcase name suffix}} {
            {{~#each properties}}
                #[serde(rename = "{{@key}}")]
                {{~#if (or
                    (or
                        (and (pattern) (patterns ""))
                        (and
                            (or
                                (eq type "string")
                                (or (eq type "integer") (eq type "number"))
                            )
                            (or
                                (not_empty minLength)
                                (not_empty maxLength)
                            )
                        )
                    )
                    (or
                        (and
                            (or (eq type "integer") (eq type "number"))
                            (or (not_empty minimum) (not_empty maximum))
                        )
                        (and
                            (eq type "array")
                            (or
                                (not_empty (fetch_patterns items))
                                (or (not_empty minItems) (not_empty maxItems))
                            )
                        )
                    )
                )}}
                {{~#if (not (has ../required @key))}}
                #[serde(default)]
                {{~/if}}
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
            #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
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
                #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
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
