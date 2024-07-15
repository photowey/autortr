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

use autortr_rocket::prelude::*;

// ----------------------------------------------------------------

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

#[request_mapping(namespace = "/rocket", method = "get", path = "/namespace")]
fn namespace_fn() -> &'static str {
    "Hello, namespace!"
}
