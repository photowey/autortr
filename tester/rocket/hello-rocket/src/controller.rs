/*
 * Copyright © 2024 the original author or authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

// tester/rocket/hello-rocket/controller

// ----------------------------------------------------------------

use rocket::form::Form;
use rocket::FromForm;

use autortr_rocket::prelude::*;

// ----------------------------------------------------------------

#[derive(Debug, FromForm)]
struct Account {
    account: String,
}

// ---------------------------------------------------------------- v0.1.0 #[request_mapping]

#[request_mapping(method = "get", path = "/get")]
fn get_fn() -> &'static str {
    "Hello, get!"
}

#[request_mapping(method = "post", path = "/post")]
fn post_fn() -> &'static str {
    "Hello, post!"
}

#[request_mapping(method = "put", path = "/put")]
fn puf_fn() -> &'static str {
    "Hello, put!"
}

#[request_mapping(method = "patch", path = "/patch")]
fn patch_fn() -> &'static str {
    "Hello, patch!"
}

#[request_mapping(method = "delete", path = "/delete")]
fn delete_fn() -> &'static str {
    "Hello, delete!"
}

// ----------------------------------------------------------------

#[request_mapping(namespace = "/rocket", method = "get", path = "/namespace")]
fn namespace_fn() -> &'static str {
    "Hello, namespace!"
}

// ----------------------------------------------------------------

/// @since 0.1.2
#[request_mapping(
    namespace = "/rocket",
    method = "post",
    path = "/data",
    data = "<form>"
)]
fn data_fn(form: Form<Account>) -> &'static str {
    let account: &str = &form.account;
    println!("data_fn: form.account: {}", account);

    "Hello, data!"
}

// ----------------------------------------------------------------

#[request_mapping(method = "head", path = "/head")]
fn head_fn() {}

// ---------------------------------------------------------------- v0.2.0 #[${http_method}_mapping]

#[get_mapping(namespace = "/http", path = "/get")]
fn http_get_mapping_fn() -> &'static str {
    "Hello, http.get mapping!"
}

#[post_mapping(namespace = "/http", path = "/post", data = "<form>")]
fn http_post_mapping_fn(form: Form<Account>) -> &'static str {
    let account: &str = &form.account;
    println!("http_post_mapping_fn: form.account: {}", account);

    "Hello, http.post mapping!"
}

#[put_mapping(namespace = "/http", path = "/put", data = "<form>")]
fn http_put_mapping_fn(form: Form<Account>) -> &'static str {
    let account: &str = &form.account;
    println!("http_put_mapping_fn: form.account: {}", account);

    "Hello, http.put mapping!"
}

#[patch_mapping(namespace = "/http", path = "/patch", data = "<form>")]
fn http_patch_mapping_fn(form: Form<Account>) -> &'static str {
    let account: &str = &form.account;
    println!("http_patch_mapping_fn: form.account: {}", account);

    "Hello, http.patch mapping!"
}

#[delete_mapping(namespace = "/http", path = "/delete", data = "<form>")]
fn http_delete_mapping_fn(form: Form<Account>) -> &'static str {
    let account: &str = &form.account;
    println!("http_delete_mapping_fn: form.account: {}", account);

    "Hello, http.delete mapping!"
}

#[head_mapping(namespace = "/http", path = "/head")]
fn http_head_mapping_fn() {}

// ---------------------------------------------------------------- v0.2.0 #[${http_method}_mapping("/path")]

#[get_mapping("/path/get")]
fn http_default_path_get_mapping_fn() -> &'static str {
    "Hello, http.default.path.get mapping!"
}

#[post_mapping("/path/post")]
fn http_default_path_post_mapping_fn() -> &'static str {
    "Hello, http.default.path.post mapping!"
}

#[put_mapping("/path/put")]
fn http_default_path_put_mapping_fn() -> &'static str {
    "Hello, http.default.path.put mapping!"
}

#[patch_mapping("/path/patch")]
fn http_default_path_patch_mapping_fn() -> &'static str {
    "Hello, http.default.path.patch mapping!"
}

#[delete_mapping("/path/delete")]
fn http_default_path_delete_mapping_fn() -> &'static str {
    "Hello, http.default.path.delete mapping!"
}

#[head_mapping("/path/head")]
fn http_default_path_head_mapping_fn() -> &'static str {
    "Hello, http.default.path.head mapping!"
}

// ----------------------------------------------------------------

pub fn __trigger_init__() {}
