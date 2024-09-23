GET http://localhost:5000/jobs/1
HTTP 200
[Asserts]
duration < 1000
jsonpath "$.id" >= 0
