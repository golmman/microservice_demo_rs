# clean up if necessary
DELETE http://localhost:3030/customer/rust@konze.de
DELETE http://localhost:3030/customer/rust@bise.de

# create with addresses
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [
        {
            "streetName": "Goethestr. 1",
            "city": "Jena",
            "postalCode": "1234"
        },
        {
            "city": "Leipzig"
        },
        {
            "city": "Berlin",
            "streetName": "Hardenbergstr. 9"
        }
    ],
    "dateOfBirth": "2024-01-01",
    "email": "rust@konze.de",
    "firstName": "hello",
    "lastName": "world"
}

HTTP 201
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 3
jsonpath "$.addresses[0].streetName" == "Goethestr. 1"
jsonpath "$.addresses[0].city" == "Jena"
jsonpath "$.addresses[0].postalCode" == "1234"
jsonpath "$.addresses[1].city" == "Leipzig"
jsonpath "$.addresses[2].city" == "Berlin"
jsonpath "$.addresses[2].streetName" == "Hardenbergstr. 9"
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "hello"
jsonpath "$.lastName" == "world"

# delete
DELETE http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 3
jsonpath "$.addresses[0].streetName" == "Goethestr. 1"
jsonpath "$.addresses[0].city" == "Jena"
jsonpath "$.addresses[0].postalCode" == "1234"
jsonpath "$.addresses[1].city" == "Leipzig"
jsonpath "$.addresses[2].city" == "Berlin"
jsonpath "$.addresses[2].streetName" == "Hardenbergstr. 9"
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "hello"
jsonpath "$.lastName" == "world"

# create without addresses
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
header "Content-Type" == "application/json"
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "hello"
jsonpath "$.lastName" == "world"

# read
GET http://localhost:3030/customer/rust@konze.de

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
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
header "Content-Type" == "application/json"
jsonpath "$.addresses" isEmpty
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Napoleon"
jsonpath "$.lastName" == "Dynamite"

# update with added addresses
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [
        {
            "streetName": "Goethestr. 1",
            "city": "Jena",
            "postalCode": "1234"
        },
        {
            "city": "Leipzig"
        },
        {
            "city": "Berlin",
            "streetName": "Hardenbergstr. 9"
        }
    ],
    "dateOfBirth": "2024-01-01",
    "email": "rust@konze.de",
    "firstName": "Napoleon",
    "lastName": "Dynamite"
}

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 3
jsonpath "$.addresses[0].streetName" == "Goethestr. 1"
jsonpath "$.addresses[0].city" == "Jena"
jsonpath "$.addresses[0].postalCode" == "1234"
jsonpath "$.addresses[1].city" == "Leipzig"
jsonpath "$.addresses[2].city" == "Berlin"
jsonpath "$.addresses[2].streetName" == "Hardenbergstr. 9"
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Napoleon"
jsonpath "$.lastName" == "Dynamite"

# update with removed address
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [
        {
            "streetName": "Goethestr. 1",
            "city": "Jena",
            "postalCode": "1234"
        },
        {
            "city": "Leipzig"
        }
    ],
    "dateOfBirth": "2024-01-01",
    "email": "rust@konze.de",
    "firstName": "Napoleon",
    "lastName": "Dynamite"
}

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 2
jsonpath "$.addresses[0].streetName" == "Goethestr. 1"
jsonpath "$.addresses[0].city" == "Jena"
jsonpath "$.addresses[0].postalCode" == "1234"
jsonpath "$.addresses[1].city" == "Leipzig"
jsonpath "$.dateOfBirth" == "2024-01-01"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Napoleon"
jsonpath "$.lastName" == "Dynamite"

# update with different data except email
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [
        {
            "city": "Dresden"
        }
    ],
    "dateOfBirth": "2025-08-26",
    "email": "rust@konze.de",
    "firstName": "Bonjour",
    "lastName": "Adieu"
}

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 1
jsonpath "$.addresses[0].city" == "Dresden"
jsonpath "$.dateOfBirth" == "2025-08-26"
jsonpath "$.email" == "rust@konze.de"
jsonpath "$.firstName" == "Bonjour"
jsonpath "$.lastName" == "Adieu"

# update  email
POST http://localhost:3030/customer/rust@konze.de
content-type:application/json
{
    "addresses": [
        {
            "city": "Dresden"
        }
    ],
    "dateOfBirth": "2025-08-26",
    "email": "rust@bise.de",
    "firstName": "Bonjour",
    "lastName": "Adieu"
}

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 1
jsonpath "$.addresses[0].city" == "Dresden"
jsonpath "$.dateOfBirth" == "2025-08-26"
jsonpath "$.email" == "rust@bise.de"
jsonpath "$.firstName" == "Bonjour"
jsonpath "$.lastName" == "Adieu"

# read old email should fail
GET http://localhost:3030/customer/rust@konze.de

HTTP 404

# read
GET http://localhost:3030/customer/rust@bise.de

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 1
jsonpath "$.addresses[0].city" == "Dresden"
jsonpath "$.dateOfBirth" == "2025-08-26"
jsonpath "$.email" == "rust@bise.de"
jsonpath "$.firstName" == "Bonjour"
jsonpath "$.lastName" == "Adieu"

# delete
DELETE http://localhost:3030/customer/rust@bise.de

HTTP 200
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.addresses" count == 1
jsonpath "$.addresses[0].city" == "Dresden"
jsonpath "$.dateOfBirth" == "2025-08-26"
jsonpath "$.email" == "rust@bise.de"
jsonpath "$.firstName" == "Bonjour"
jsonpath "$.lastName" == "Adieu"

# read
GET http://localhost:3030/customer/rust@bise.de

HTTP 404
[Asserts]
header "Content-Type" == "application/json"
jsonpath "$.code" == "CustomerNotFound"
