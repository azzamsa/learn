# REST API health
GET http://127.0.0.1:8000/health

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.status" == "running"

# GraphQL health
POST http://127.0.0.1:8000/graphql

```graphql
query {
	health {
		status
	}
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.health.status" == "running"


# meta
POST http://127.0.0.1:8000/graphql

```graphql
query {
	meta {
		build
		version
	}
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.meta.build" == "unknown"


# encrypt
POST http://127.0.0.1:8000/graphql

```graphql
query {
	encrypt(plain: "Hello, World!", rotation: 13) {
		secret
	}
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.encrypt.secret" == "Uryyb, Jbeyq!"


# decrypt
POST http://127.0.0.1:8000/graphql

```graphql
query {
	decrypt(secret: "Uryyb, Jbeyq!", rotation: 13) {
		plain
	}
}
```

HTTP/1.1 200
[Asserts]
status == 200
jsonpath "$.data.decrypt.plain" == "Hello, World!"
