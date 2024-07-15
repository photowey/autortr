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

// rocket/codegen/lib

// ----------------------------------------------------------------

extern crate proc_macro;
extern crate rocket;

// ----------------------------------------------------------------

use proc_macro::TokenStream;

use quote::{format_ident, quote};
use syn::{parse_macro_input, AttributeArgs, ItemFn, Meta, NestedMeta};

use autortr_rocket_core::{METHOD, NAMESPACE, PATH, ROOT};

// ----------------------------------------------------------------

/// A macro for simplifying route mapping and registration in Rocket applications.
///
/// This macro can be applied to functions to automatically generate Rocket route attributes
/// based on provided method and path arguments. It also registers the route mappings
/// using the `inventory` crate for dynamic route handling.
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};;
/// use autortr_rocket_codegen::request_mapping;
/// // use autortr_rocket::prelude::*;
///
/// #[request_mapping(method = "get", path = "/get")]
/// fn get_fn() -> &'static str {
///     "Hello, get!"
/// }
///
/// // with `Namespace`
/// #[request_mapping(namespace = "/rocket", method = "get", path = "/namespace")]
/// fn namespace_fn() -> &'static str {
///     "Hello, namespace!"
/// }
/// ```
///
/// Additionally, it registers the route mapping using `lazy_static` for runtime route management.
/// If the `namespace` attribute is not provided, it defaults to "/".
///
/// Note: This macro requires the `rocket` and `lazy_static` crates to be included in your project.
///
#[proc_macro_attribute]
pub fn request_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_args = parse_macro_input!(args as AttributeArgs);
    let function = parse_macro_input!(item as ItemFn);

    let function_ident = &function.sig.ident;
    let function_name = function_ident.to_string();

    let (namespace, method, path) = match parse_request_mapping_args(parsed_args) {
        Ok((n, m, p)) => (n, m, p),
        Err(_) => panic!("Invalid arguments to `#[request_mapping]`"),
    };

    let namespace = namespace.unwrap_or_else(|| ROOT.to_string());

    let route = match method.as_str() {
        "get" => quote! { #[rocket::get(#path)] },
        "post" => quote! { #[rocket::post(#path)] },
        "put" => quote! { #[rocket::put(#path)] },
        "patch" => quote! { #[rocket::patch(#path)] },
        "delete" => quote! { #[rocket::delete(#path)] },
        "head" => quote! { #[rocket::head(#path)] },
        _ => panic!("Unsupported HTTP method"),
    };

    let register_fn_name = format_ident!("_register_{}_", function_ident);

    let register_fn = quote! {
        #[ctor::ctor]
        fn #register_fn_name() {
            register_route_mapping(RouteMapping {
                function: #function_name.to_string(),
                namespace: #namespace.to_string(),
                method: #method.to_string(),
                route: #path.to_string(),
                routes: rocket::routes![#function_ident],
            });
        }
    };

    let expanded = quote! {
        #route
        #function

        #register_fn
    };

    expanded.into()
}

fn parse_request_mapping_args(args: AttributeArgs) -> Result<(Option<String>, String, String), ()> {
    let mut namespace = None;
    let mut method = None;
    let mut path = None;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident(NAMESPACE) {
                    if let syn::Lit::Str(n) = nv.lit {
                        namespace = Some(n.value());
                    }
                } else if nv.path.is_ident(METHOD) {
                    if let syn::Lit::Str(m) = nv.lit {
                        method = Some(m.value());
                    }
                } else if nv.path.is_ident(PATH) {
                    if let syn::Lit::Str(p) = nv.lit {
                        path = Some(p.value());
                    }
                }
            }
            _ => return Err(()),
        }
    }

    match (namespace, method, path) {
        (Some(n), Some(m), Some(p)) => Ok((Some(n), m, p)),
        (None, Some(m), Some(p)) => Ok((None, m, p)),
        _ => Err(()),
    }
}
