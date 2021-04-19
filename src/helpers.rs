use anyhow::Result;
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, JsonRender, JsonValue, Output, RenderContext,
    RenderError,
};
use json_pointer::JsonPointer;
use serde_json::value::Value as Json;
use serde_json::{Map, Value};

macro_rules! case_helper {
    ($name:ident, $function:ident) => {
        pub(crate) fn $name(
            helper: &Helper,
            _: &Handlebars,
            _: &Context,
            _: &mut RenderContext,
            out: &mut dyn Output,
        ) -> Result<(), RenderError> {
            use heck::*;
            let values = helper
                .params()
                .iter()
                .map(|v| v.value().render())
                .collect::<Vec<_>>();
            let rendered = values.as_slice().join(" ").$function();
            out.write(rendered.as_ref())?;
            Ok(())
        }
    };
}

case_helper!(mixedcase, to_mixed_case);
case_helper!(camelcase, to_camel_case);
case_helper!(snakecase, to_snake_case);
case_helper!(shoutysnakecase, to_shouty_snake_case);
handlebars_helper!(component_path: |ref_path: str| parse_component_path(ref_path));
handlebars_helper!(sanitize: |word: str| apply_sanitize(word));
handlebars_helper!(json: |data: Json| apply_json(data));
handlebars_helper!(is_http_code_success: |http_status: str| http_status.starts_with("1") || http_status.starts_with("2") || http_status.starts_with("3"));
handlebars_helper!(fetch_patterns: |data: Json| fetch_patterns_recursive(&data));
handlebars_helper!(patterns: |_s: str| cfg!(feature = "patterns"));
handlebars_helper!(is_empty: |data: Json| obj_is_empty(&data));
handlebars_helper!(not_empty: |data: Json| obj_not_empty(&data));

pub(crate) fn parse_component_path(ref_path: &str) -> String {
    use heck::{CamelCase, SnekCase};

    let mut path = Vec::new();
    let (filename, ref_path) = {
        let mut split = ref_path.split('#');
        (
            split.next().filter(|x| !x.is_empty()),
            split.next().unwrap_or(""),
        )
    };

    let mut pointer = ref_path.parse::<JsonPointer<_, _>>().unwrap();
    while let Some(segment) = pointer.pop() {
        path.push(segment);
    }
    if let Some(name) = path.first_mut() {
        *name = name.to_camel_case()
    }
    if let Some(filename) = filename {
        let name = std::path::Path::new(filename)
            .file_stem()
            .and_then(|x| x.to_str())
            .expect("couldn't get the $ref file")
            .to_snek_case();

        if name.contains('.') {
            panic!("Invalid module name `{}`", name);
        }

        path.push(name);

        // FIXME: this way only components/schemas can have $refs from other files
        path.push("super::super::super".to_owned());
    }
    path.reverse();
    path.join("::")
}

const KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv",
    "typeof", "unsized", "virtual", "yield", "async", "await", "try",
];

pub(crate) fn apply_sanitize(word: &str) -> String {
    if KEYWORDS.iter().any(|&keyword| word == keyword) {
        format!("r#{}", word)
    } else {
        word.to_string()
    }
}

pub(crate) fn has(
    helper: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let data = helper
        .param(0)
        .ok_or_else(|| RenderError::new("data not found"))?
        .value();
    let field = helper
        .param(1)
        .ok_or_else(|| RenderError::new("field not found"))?
        .value()
        .as_str()
        .ok_or_else(|| RenderError::new("field is not a string"))?;
    let value = helper.param(2);
    let result = match data {
        serde_json::Value::Array(data) => {
            if let Some(value) = value {
                let value_converted = value
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderError::new("value is not a string"))?;
                data.iter()
                    .any(|list_elem| list_elem[field] == value_converted)
            } else {
                data.iter().any(|list_elem| list_elem == field)
            }
        }
        serde_json::Value::Object(data) => {
            if let Some(value) = value {
                let field_value = data
                    .get(field)
                    .ok_or_else(|| RenderError::new("field does not exist"))?;
                let value_converted = value
                    .value()
                    .as_str()
                    .ok_or_else(|| RenderError::new("value is not a string"))?;
                field_value == value_converted
            } else {
                data.get(field).is_some()
            }
        }
        _ => false,
    };
    out.write(if result { "true" } else { "" })?;
    Ok(())
}

pub(crate) fn fetch_patterns_recursive(map: &JsonValue) -> Map<String, Value> {
    let map = match map.as_object() {
        None => return Map::new(),
        Some(map) => map,
    };

    let mut collected_patterns = Map::new();

    fn search_patterns_field(
        prefix_name: &str,
        field: &Map<String, Value>,
        mut collected_patterns: &mut Map<String, Value>,
    ) {
        for (field_name, field_value) in field {
            match &field_value {
                value
                    if value
                        .get("type")
                        .map_or(false, |v| v.as_str().map_or(false, |txt| txt == "string")) =>
                {
                    if let Some(pattern) = value.get("pattern") {
                        collected_patterns.insert(
                            if prefix_name.is_empty() {
                                field_name.clone()
                            } else {
                                format!("{}", prefix_name)
                            },
                            pattern.clone(),
                        );
                    }
                }
                value @ Value::Object(_) => {
                    let array = field.get("items").and_then(|items| items.as_object());

                    // Ensuring we'll be parsing arrays only, as otherwise we get enums as well
                    if let Some(map) = array {
                        search_patterns_field(field_name, map, &mut collected_patterns);
                    } else if let Some(map) = value.as_object() {
                        search_patterns_field(field_name, map, &mut collected_patterns);
                        // dbg!(&field_name, &field_value, &collected_patterns);
                    }
                }
                _ => (),
            }
        }
    }

    search_patterns_field("", map, &mut collected_patterns);

    collected_patterns
}

pub(crate) fn obj_is_empty(array: &JsonValue) -> bool {
    match array {
        Value::Null => true,
        Value::Bool(_) => false,
        Value::Number(v) => v.to_string().is_empty(),
        Value::String(v) => v.is_empty(),
        Value::Array(v) => v.is_empty(),
        Value::Object(v) => v.is_empty(),
    }
}

pub(crate) fn obj_not_empty(array: &JsonValue) -> bool {
    !obj_is_empty(array)
}

pub(crate) fn apply_json(data: &Json) -> String {
    data.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_pattern() {
        let map = serde_json::json!({
                "string_field": {
                "type": "string",
                "pattern": "[a-z]*"
            },
            "list": {
                "type": "array",
                "items": {
                    "type": "string",
                    "pattern": ".*",
                }
            }
        });

        let rec_pattern = fetch_patterns_recursive(&map);

        assert!(rec_pattern.get("string_field").is_some());
        assert!(rec_pattern.get("list").is_some());
    }

    #[test]
    fn test_parse_component_path() {
        assert_eq!(
            parse_component_path("#/components/schemas/Pet"),
            "components::schemas::Pet".to_string()
        )
    }
}
