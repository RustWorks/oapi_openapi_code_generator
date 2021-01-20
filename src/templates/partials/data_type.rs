{{#unless required}}Option<{{/unless~}}
{{~#if (eq type "string")}}
    {{~#if (eq format "binary")~}}
Vec<u8>
    {{~else~}}
        {{~#if (eq format "byte")~}}
Vec<u8>
        {{~else~}}
            {{~#if enum~}}
{{camelcase name}}
            {{~else~}}
String
            {{~/if}}
        {{~/if}}
    {{~/if}}
{{~/if}}
{{~#if (eq type "number")~}}
    {{~#if (eq format "float")~}}
f32
    {{~else~}}
f64
    {{~/if}}
{{~/if}}
{{~#if (eq type "integer")~}}
    {{~#if (eq format "int32")~}}
i32
    {{~else~}}
i64
    {{~/if}}
{{~/if}}
{{~#if (eq type "boolean")~}}
bool
{{~/if}}
{{~#if (eq type "array")~}}
Vec<{{~>data_type name=(camelcase name "item") required="true" items}}>
{{~/if}}
{{~#if [$ref]}}
{{~component_path [$ref]}}
{{~/if}}
{{~#if (eq type "object")~}}
{{camelcase name}}
{{~/if}}
{{~#unless required}}>{{/unless}}