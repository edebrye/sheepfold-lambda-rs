GET {{base_url}}/sheep
HTTP 200

POST {{base_url}}/sheep?name=John+Doe
HTTP 200
[Asserts]
jsonpath "$.name" == "John Doe"
[Captures]
id: jsonpath "$.id"

GET {{base_url}}/sheep/{{id}}
HTTP 200
[Asserts]
jsonpath "$.name" == "John Doe"

DELETE {{base_url}}/sheep/{{id}}
HTTP 200
[Asserts]
jsonpath "$.deleted" == true
jsonpath "$.old.name" == "John Doe"
