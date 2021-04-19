{{~#if (and (object.pattern) (patterns ""))}}
// Regex validation
    if !{{shoutysnakecase name key suffix}}_PATTERN.is_match(&{{var_name}}) {
        return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required pattern - passed value [{}]", &{{var_name}})));
    }
{{~/if}}

{{~#if (and
    (or
        (eq object.type "string")
        (or (eq object.type "integer") (eq object.type "number"))
    )
    (or
        (not_empty object.minLength)
        (not_empty object.maxLength)
    )
)}}
// minLength or maxLength
    {{~#if (not_empty object.minLength)}}
        if {{var_name}}.len() < {{object.minLength}} {
            return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required minLength - passed value [{}]", &{{var_name}})));
        }
    {{~/if}}

    {{~#if (not_empty object.maxLength)}}
        if {{var_name}}.len() > {{object.maxLength}} {
            return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required maxLength - passed value [{}]", &{{var_name}})));
        }
    {{~/if}}
{{~/if}}

{{~#if (and
    (or (eq object.type "integer") (eq object.type "number"))
    (or (not_empty object.minimum) (not_empty object.maximum))
)}}
// minimum and maximum
    {{~#if (not_empty object.minimum)}}
        if {{var_name}} < {{object.minimum}} {
            return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required minimum value - passed value [{}]", &{{var_name}})));
        }
    {{~/if}}

    {{~#if (not_empty object.maximum)}}
        if {{var_name}} > {{object.maximum}} {
            return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required maximum value - passed value [{}]", &{{var_name}})));
        }
    {{~/if}}
{{~/if}}

{{~#if (and
    (eq object.type "array")
    (or
        (not_empty (fetch_patterns object.items))
        (or (not_empty object.minItems) (not_empty object.maxItems))
    )
)}}
// array patterns and minItems/maxItems
    {{~#if (not_empty object.minItems)}}
        if {{var_name}}.len() < {{object.minItems}} {
            return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required minimum value - passed value [{} items]", {{var_name}}.len())));
        }
    {{~/if}}

    {{~#if (not_empty object.maxItems)}}
        if {{var_name}}.len() > {{object.maxItems}} {
            return Err(serde::de::Error::custom(format!("`{{name}}{{suffix}}`'s parameter `{{key}}` does not match its required maximum value - passed value [{} items]", {{var_name}}.len())));
        }
    {{~/if}}


    {{~#if (eq object.items.type "object")}}
        {{~>validation object=object.items.properties name=name key=key suffix=suffix var_name=(snakecase var_name "_" name "_inner")}}
    {{~else}}
        {{~#if (or (eq object.items.type "string") (or (eq object.items.type "number") (eq object.items.type "integer")))}}
            {
                for {{(snakecase var_name "_value")}} in {{var_name}} {
                    {{~>validation object=object.items name=name key=key suffix=suffix var_name=(snakecase var_name "_value")}}
                }
            }
        {{~/if}}
    {{~/if}}
{{~/if}}