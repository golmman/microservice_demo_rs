# Method not Allowed
PUT http://localhost:3030/customer/a@b.com
content-type:application/json

HTTP 405
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.code" == "MethodNotAllowed"
jsonpath "$.message" exists


# bad body
POST http://localhost:3030/customer/a@b.com
content-type:application/json
{
    "nonsense": []
}

HTTP 400
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.code" == "DeserializationFailed"
jsonpath "$.message" contains "missing field `email`"

# not found
GET http://localhost:3030/customer/a@b.com

HTTP 404
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.code" == "CustomerNotFound"
jsonpath "$.message" exists
