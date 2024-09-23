GET http://localhost:5000
HTTP 200
[Asserts]
jsonpath "$.message" == "Open Sesame 1.1.0"

GET http://localhost:5000/version
HTTP 200
[Asserts]
jsonpath "$.message" == "Open Sesame 1.1.0"
