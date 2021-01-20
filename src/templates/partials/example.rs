{{~#if (eq schema.type "array")~}}
Some(vec![example::{{~component_path schema.items.[$ref]}}Example::default().unwrap()])
{{~else~}}
    {{~#if schema.[$ref]~}}
Some(example::{{~component_path schema.[$ref]~}}Example::default().unwrap())
    {{~else~}}
        {{~#unless required~}}Some({{/unless~}}
        {{~#if example includeZero=true~}}
            {{~#if (eq schema.type "string")~}}
Some("{{example}}".to_string())
            {{~/if~}}
            {{~#if (eq schema.type "number")~}}
                {{~#if (eq format "float")~}}
Some({{example}}_f32)
                {{~else~}}
Some({{example}}_f64)
                {{~/if~}}
            {{~/if~}}
            {{~#if (eq schema.type "integer")~}}
                {{~#if (eq format "int32")~}}
Some({{example}}_i32)
                {{~else~}}
Some({{example}}_i64)
                {{~/if~}}
            {{~/if~}}
            {{~#if (eq schema.type "boolean")~}}
Some({{example}})
            {{~/if~}}
        {{~else~}}
Option::<{{>data_type schema required=true}}>::None
        {{~/if~}}
        {{~#unless required~}}){{~/unless~}}
    {{~/if~}}
{{~/if~}}