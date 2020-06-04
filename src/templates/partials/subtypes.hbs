{{~#if (eq type "object")~}}
    {{~#each properties}}
        {{~#if (eq type "object")}}
            {{~>schema name=(camelcase ../name @key) this}}

        {{/if}}
        {{~#if (eq type "array")}}
            {{~#if (eq items.type "object")}}
                {{~>schema name=(camelcase ../name @key "item") items}}

            {{/if}}
        {{~/if}}
        {{~#if (and (eq type "string") enum (not format))}}
            {{~>schema name=(camelcase ../name @key) this}}

        {{~/if}}
    {{~/each}}
{{~/if}}
{{~#if (eq type "array")}}
    {{~#if (eq items.type "object")}}
        {{~>schema name=(camelcase ../name @key "item") items}}

    {{/if}}
{{~/if}}