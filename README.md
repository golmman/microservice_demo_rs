# microservice_demo_rs

## ideas
* demonstrate clean build
* toCamelCase
* rust doesn't know a 'null' value, uses Option
* serde doesn't distinguish null and undefined
* if let, let else
* Result = Maybe (Left/Right) monad

## hurl
```
hurl --test --glob hurl/**/*.hurl
```

## run
```sh
export $(xargs < .env)
```
