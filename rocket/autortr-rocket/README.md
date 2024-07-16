# `autortr-rocket`

`autortr-rocket` is a lightweight Rust library designed for automatic route mapping in web applications, particularly
tailored for seamless integration with the `Rocket` framework. It simplifies the process of defining and managing `HTTP`
routes by automatically registering functions annotated with custom attributes like #[request_mapping].

## 1.`Usage`

Add this to your `Cargo.toml`:

```toml
[dependencies]
autortr_rocket = "0.1"

# And
# If necessary
rocket = "${version}"
lazy_static = "${version}"
ctor = "${version}"
```

## 2.`APIs`

### 2.1.`Import`

```rust
use autortr_rocket::prelude::*;
```

### 2.2.`Controller`

`API` functions.

#### 2.2.1.`GET`

```rust
// Default mount base-path is: "/"
app.mount("/", routes![get_fn]);
```

```rust
// GET: method == get; path == "/get"
// -> rocket: #[get("/get")]
// -> HTTP: http://127.0.0.1:8000/get
#[request_mapping(method = "get", path = "/get")]
fn get_fn() -> &'static str {
    "Hello, get!"
}
```

#### 2.2.2.`POST`

```rust
// POST: method == post; path == "/post"
// -> rocket: #[post("/post")]
// -> HTTP: http://127.0.0.1:8000/post
#[request_mapping(method = "post", path = "/post")]
fn post_fn() -> &'static str {
    "Hello, post!"
}
```

#### 2.2.3.`PUT`

```rust
// PUT: method == put; path == "/put"
// -> rocket: #[put("/put")]
// -> HTTP: http://127.0.0.1:8000/put
#[request_mapping(method = "put", path = "/put")]
fn puf_fn() -> &'static str {
    "Hello, put!"
}
```

#### 2.2.4.`PATCH`

```rust
// PATCH: method == patch; path == "/patch"
// -> rocket: #[patch("/patch")]
// -> HTTP: http://127.0.0.1:8000/patch
#[request_mapping(method = "patch", path = "/patch")]
fn patch_fn() -> &'static str {
    "Hello, patch!"
}
```

#### 2.2.5.`DELETE`

```rust
// DELETE: method == delete; path == "/delete"
// -> rocket: #[delete("/delete")]
// -> HTTP: http://127.0.0.1:8000/delete
#[request_mapping(method = "delete", path = "/delete")]
fn delete_fn() -> &'static str {
    "Hello, delete!"
}
```

#### 2.2.5.`Namespace`

Custom mount base-path by `namespace` attribute.

- `Rocket's` base

```rust
app.mount("/namespace", routes![namespace_fn]);
```

```rust

// GET: namespace == rocket, method == get; path == "/namespace"
// -> rocket: #[get("/namespace")]
// -> rocket: mount: /rocket/namespace
// -> HTTP: http://127.0.0.1:8000/rocket/namespace
#[request_mapping(namespace = "/rocket", method = "get", path = "/namespace")]
fn namespace_fn() -> &'static str {
    "Hello, namespace!"
}
```

### 2.3.`App instance`

```rust
fn configure() -> AdHoc {
    AdHoc::on_ignite("Configure Rocket", |rocket| async {
        println!("Running additional initialization");
        rocket
    })
}

// ----------------------------------------------------------------


// 1.Populate Rocket instance.
// -> auto-mount by Autortr
let rocket_app = app(); // Rocket<Build>

// 2.Configure
// rocket_app.[...]

// 3.Launch
// let _ = app.attach(configure())[.xxx.yyy.zzz].launch().await?;
let _ = rocket_app.attach(configure()).launch().await?;

Ok(())
```

#### 2.3.1.`launch`

```rust
#[rocket::launch]
fn rocket() -> _ {
    let app = app();
    app.attach(configure())
}
```

#### 2.3.2.`main`

```rust
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = app();
    let _ = app.attach(configure()).launch().await?;

    Ok(())
}

```

2.4.`Next`

2.4.1.`Other's macros`

- `get_mapping`

    - `#[request_mapping(method="get")]`

- `post_mapping`

    - `#[request_mapping(method="post")]`

- `put_mapping`

    - `#[request_mapping(method="put")]`

- `patch_mapping`

    - `#[request_mapping(method="patch")]`

- `delete_mapping`

    - `#[request_mapping(method="delete")]`

- `head_mapping`

    - Supported or not?

2.4.2.`Other's web framework`

- …
