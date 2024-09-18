GET http://localhost:5000
HTTP 200
[Asserts]
jsonpath "$.message" == "Open Sesame"
