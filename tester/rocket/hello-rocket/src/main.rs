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

// tester/rocker/hello-rocket/main

// ----------------------------------------------------------------

use rocket::fairing::AdHoc;

use autortr_rocket::prelude::*;

// ----------------------------------------------------------------

// Example of additional initialization
fn configure() -> AdHoc {
    AdHoc::on_ignite("Configure Rocket", |rocket| async {
        println!("Running additional initialization");
        rocket
    })
}

// ----------------------------------------------------------------

// #[rocket::launch]
// fn rocket() -> _ {
//     let app = app();
//     app.attach(configure())
// }

// ----------------------------------------------------------------

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let app = app();
    let _ = app.attach(configure()).launch().await?;

    Ok(())
}
