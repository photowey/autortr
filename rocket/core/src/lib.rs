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

// rocket/core/lib

// ----------------------------------------------------------------

use std::sync::Mutex;

use lazy_static::lazy_static;
use rocket::Route;

pub const GET: &str = "get";
pub const POST: &str = "post";
pub const PUT: &str = "put";
pub const PATCH: &str = "patch";
pub const DELETE: &str = "delete";
pub const HEAD: &str = "head";

// ----------------------------------------------------------------

pub const NAMESPACE: &str = "namespace";
pub const METHOD: &str = "method";
pub const PATH: &str = "path";
pub const DATA: &str = "data";

// ----------------------------------------------------------------

pub const ROOT: &str = "/";
pub const EMPTY: &str = "_";

// ----------------------------------------------------------------

#[derive(Clone)]
pub struct RouteMapping {
    pub function: String,
    pub namespace: String,
    pub method: String,
    pub path: String,
    pub data: String,
    pub routes: Vec<Route>,
}

// ----------------------------------------------------------------

lazy_static! {
    static ref ROUTE_MAPPINGS: Mutex<Vec<RouteMapping>> = Mutex::new(Vec::new());
}

// ----------------------------------------------------------------

pub fn register_route_mapping(mapping: RouteMapping) {
    let mut mappings = ROUTE_MAPPINGS.lock().unwrap();
    mappings.push(mapping);
}

// ----------------------------------------------------------------

pub fn try_acquire_route_mappings() -> Vec<RouteMapping> {
    let mappings = ROUTE_MAPPINGS.lock().unwrap();
    mappings.clone()
}

// ----------------------------------------------------------------

pub fn clean_route_mappings() {
    let mut mappings = ROUTE_MAPPINGS.lock().unwrap();
    mappings.clear();
}
