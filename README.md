# microservice_demo_rs

## ideas
* demonstrate clean build
* rust doesn't know a 'null' value, uses Option
* serde doesn't distinguish null and undefined
* serde toCamelCase
* if let, let else
* Result = Maybe (Left/Right) monad
* "rust has nice error messages": for a counterexample swap 'and' and 'map' in a route in main.rs
* debug vs release build
* unit tests
* linting
* formatting
* rust analyzer
* size of the target dir...
* warp
  * rejection handlings sucks: https://github.com/seanmonstar/warp/issues/77
  * no url decoding
  * confusing filters
  * axum is probably a better choice
* how to do
  * request validation
  * error handling
  * logging
  * spec generation: https://github.com/juhaku/utoipa

## run
```sh
export $(xargs < .env)
cargo run
```

## test
```sh
hurl --test hurl/test/**/*.hurl
```
