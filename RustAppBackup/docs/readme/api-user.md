# REST API User

## Create a new User

### Endpoint

    /user

### HTTP Method
    
    POST

### Request

`POST /user/`

    curl -i -H 'Accept: application/json' -d 'name=Foo&status=new' http://localhost:7000/thing

### Response

    HTTP/1.1 201 Created
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {
        "name":"Foo",
        "status":"new"
    }

## Update an User

### Endpoint

    /user

### HTTP Method

    PUT

### Request

### Response

    HTTP/1.1 200 Success
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 201 Created
    Connection: close
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

## Delete an User

### Endpoint

    /user

### HTTP Method
    
    DELETE

### Request

### Response
    
    HTTP/1.1 200 Success
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 Created
    Connection: Success
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

## Get an User

### Endpoint
    
    /user

### Request

### Response

    HTTP/1.1 200 Success
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 Success
    Connection: close
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

### HTTP Method

    /GET

### Request

### Response

    HTTP/1.1 200 Success
    Date: Thu, 24 Feb 2011 12:36:30 GMT
    Status: 200 Success
    Connection: close
    Content-Type: application/json
    Location: /thing/1
    Content-Length: 36

    {"id":1,"name":"Foo","status":"new"}

