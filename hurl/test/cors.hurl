# clean up if necessary
DELETE http://localhost:3030/customer/rust@konze.de

# Preflight
OPTIONS http://localhost:3030

HTTP 204
[Asserts]
header "Access-Control-Allow-Origin" == "*"
header "Access-Control-Allow-Methods" == "*"
header "Access-Control-Allow-Headers" == "*"


# create
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "email": "rust@konze.de"
}

HTTP 201
[Asserts]
header "Access-Control-Allow-Origin" == "*"

# update
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "email": "rust@konze.de",
    "firstName": "Hello"
}

HTTP 200
[Asserts]
header "Access-Control-Allow-Origin" == "*"

# read
GET http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
header "Access-Control-Allow-Origin" == "*"

# delete
DELETE http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
header "Access-Control-Allow-Origin" == "*"
