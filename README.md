```
cargo run
```

Navigate to [http://127.0.0.1:8080/graphiql](http://127.0.0.1:8080/graphiql).

Use this query:

```
query {
  u1: user(id: "1") {
    id
    friend { id }
  }
  u2: user(id: "2") {
    id
    friend { id }
  }
  u3: user(id: "3") {
    id
    friend { id }
  }
}
```

Logs output:

```
DEBUG dataloader_bug::graphql > load batch ["1", "3", "2"]
DEBUG dataloader_bug::graphql > load batch ["friend of 1"]
DEBUG dataloader_bug::graphql > load batch ["friend of 2"]
DEBUG dataloader_bug::graphql > load batch ["friend of 3"]
```