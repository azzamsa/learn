POST {{base_url}}
Content-Type: application/json

```graphql
{
  encrypt(plain: "Hello, World!", rotation: 13) {
    secret
    rotation
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.encrypt.secret" == "Uryyb, Jbeyq!"


