{{~#if (eq type "object")~}}
    {{~#each properties}}
        {{~#if (eq type "object")}}
            {{~>schema_example name=(camelcase ../name @key) this}}

        {{~/if}}
        {{~#if (eq type "array")}}
            {{~#if (eq items.type "object")}}
                {{~>schema_example name=(camelcase ../name @key "item") items}}

            {{~/if}}
        {{~/if}}
    {{~/each}}
{{~/if}}