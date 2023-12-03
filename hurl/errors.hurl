# Method not Allowed
PUT http://localhost:3030/customer
content-type:application/json

HTTP 405

# Bad Path
POST http://localhost:3030/customer1
content-type:application/json

HTTP 404
