{{~>subtypes_example name=name}}
{{!-- https://swagger.io/docs/specification/adding-examples/ --}}
pub struct {{camelcase name suffix}}Example;
{{#if type~}}
    {{~#if example}}
    const {{shoutysnakecase name suffix}}: &str = r#"{{json example}}"#;

    impl {{camelcase name suffix}}Example {
        pub fn default() -> Option<{{camelcase name suffix}}> {
            {{~#if example}}
            Some(serde_json::from_str({{shoutysnakecase name suffix}}).unwrap())
            {{~else~}}
            None
            {{~/if}}
        }
    }
    {{~else~}}
    impl {{camelcase name suffix}}Example {
        pub fn default() -> Option<{{camelcase name suffix}}> {
            {{~#if properties}}
            let {{snakecase name suffix}}: String = json!({
            {{~#each properties}}
                {{~#if (eq type "object")}}
                "{{snakecase @key}}": {{(camelcase ../name @key)}}Example::default().unwrap(),
                {{~/if}}
                {{~#if (eq type "array")}}
                {{~#if (eq items.type "object")}}
                "{{snakecase @key}}": vec![{{(camelcase ../name @key)}}Example::default().unwrap()],
                {{/if}}
                {{~/if}}
                {{~#if example}}
                {{~#if (and (not (eq type "object")) (not (eq type "array")))}}
                "{{snakecase @key}}": {{> example schema=this}},
                {{~/if}}
                {{~/if}}
            {{~/each}}
            }).to_string();
            Some(serde_json::from_str(&{{snakecase name suffix}}).unwrap())
            {{~else~}}
            None
            {{~/if}}
        }
    }
    {{~/if}}
{{~/if}}
