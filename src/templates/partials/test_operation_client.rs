#[test]
fn test_{{snakecase operationId}}_{{status}}() {
  let api_client = Client::new(&mockito::server_url());
  let uri = format!("{{uri}}"
    {{~#each parameters}}
      {{~#if (eq in "path")}}, {{name}} = {{>example}}.unwrap(){{/if}}
    {{~/each~}}
  );
  let mock = mock("{{shoutysnakecase operation_verb}}", Matcher::Exact(uri))
    {{~#if parameters}}
    .match_query(Matcher::AllOf(vec![
      {{~#each parameters}}
        {{~#if (eq in "query")}}
        Matcher::UrlEncoded("{{name}}".into(), ({{>example}}.unwrap()).serialize().unwrap()),
        {{~/if}}
      {{~/each~}}
    ]))
    {{~/if}}
    .with_status({{status}})
    {{~#with response.content.[application/json]}}
    .with_body(serde_json::to_string(&{{>example}}.unwrap()).unwrap())
    {{~/with}}
    {{~#with response.content.[application/x-www-form-urlencoded]}}
    .with_body(&{{>example}}.unwrap())
    {{~/with}}
    .create();
  {{~#if parameters}}
  let parameters = {{snakecase operationId}}::Parameters {
    {{~#each parameters}}
      {{snakecase name}}: {{>example}}.unwrap(),
    {{~/each~}}
  };
  {{~/if}}
  let result = api_client.{{snakecase operationId}}(
    {{~#if parameters}}
    &parameters,
    {{~/if}}
    {{~#with requestBody.content.[application/json]}}
    &{{>example required="true"}}.unwrap()
    {{~/with}});
    {{~#with requestBody.content.[application/x-www-form-urlencoded]}}
    &{{>example required="true"}}.unwrap()
    {{~/with}});
  mock.assert();
  let data = result.unwrap();
  use {{snakecase operationId}}::Response::*;
  if let {{camelcase "Response" status}}(object) = data {
    {{~#with response.content.[application/json]}}
    assert_eq!(object, {{>example required="false"}}.unwrap());
    {{~/with}}
    {{~#with response.content.[application/x-www-form-urlencoded]}}
    assert_eq!(object, {{>example required="false"}}.unwrap());
    {{~/with}}
  } else {
    panic!("invalid type");
  }
}