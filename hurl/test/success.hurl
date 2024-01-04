# clean up if necessary
DELETE http://localhost:3030/customer/rust@konze.de

# create
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [],
    "dateOfBirth": "2024-01-01",
    "email": "rust@konze.de",
    "firstName": "hello",
    "lastName": "world"
}

HTTP 201
[Asserts]
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "hello"
jsonpath "$.lastName" == "world"

# read
GET http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "hello"
jsonpath "$.lastName" == "world"

# update
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [],
    "dateOfBirth": "2024-01-01",
    "email": "rust@konze.de",
    "firstName": "Napoleon",
    "lastName": "Dynamite"
}

HTTP 200
[Asserts]
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Napoleon"
jsonpath "$.lastName" == "Dynamite"

# read
GET http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Napoleon"
jsonpath "$.lastName" == "Dynamite"

# delete
DELETE http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Napoleon"
jsonpath "$.lastName" == "Dynamite"

# read
GET http://localhost:3030/customer/rust@konze.de

HTTP 404
[Asserts]
jsonpath "$.code" == "CustomerNotFound"
