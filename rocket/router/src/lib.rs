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

// rocket/router/lib

// ----------------------------------------------------------------

use rocket::{Build, Rocket};

use autortr_rocket_core::{clean_route_mappings, try_acquire_route_mappings};

// ----------------------------------------------------------------

// Bad function now.
// |- app cannot move?
#[doc(hidden)]
#[allow(dead_code)]
fn app_fn<F>(mut init: F) -> Rocket<Build>
where
    F: FnMut(&mut Rocket<Build>),
{
    let mut app = rocket::build();

    for mapping in try_acquire_route_mappings() {
        app = app.mount(mapping.namespace, mapping.routes.clone());
    }

    init(&mut app);

    clean_route_mappings();

    app
}

// ----------------------------------------------------------------

/// Constructs and returns a Rocket application with registered route mappings.
///
/// This function initializes a Rocket application, acquires route mappings,
/// mounts them to the application, and cleans up the mappings afterward.
///
/// # Returns
///
/// Returns a configured instance of `Rocket<Build>`.
///
/// # Example
///
/// ```rust
/// use rocket::{Build, Rocket};
/// use autortr_rocket_core::{clean_route_mappings, try_acquire_route_mappings};
/// // use autortr_rocket::prelude::*;
///
/// pub fn app() -> Rocket<Build> {
///     let mut app = rocket::build();
///
///     for mapping in try_acquire_route_mappings() {
///         app = app.mount(mapping.namespace, mapping.routes.clone());
///     }
///
///     clean_route_mappings();
///
///     app
/// }
/// ```
pub fn app() -> Rocket<Build> {
    let mut app = rocket::build();

    for mapping in try_acquire_route_mappings() {
        app = app.mount(mapping.namespace, mapping.routes.clone());
    }

    clean_route_mappings();

    app
}
