POST {{base_url}}
Content-Type: application/json

```graphql
{
  decrypt(secret: "Uryyb, Jbeyq!", rotation: 13) {
    plain
    rotation
  }
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.decrypt.plain" == "Hello, World!"


