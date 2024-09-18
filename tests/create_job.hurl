POST http://localhost:5000/jobs
{
    "title": "Lorem Ipsum",
    "description": "Lorem"
}
HTTP 201
[Asserts]
duration < 1000
jsonpath "$.id" >= 0
