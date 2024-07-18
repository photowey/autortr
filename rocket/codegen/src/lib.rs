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
use syn::{parse_macro_input, AttributeArgs, ItemFn, Lit, Meta, NestedMeta};

use autortr_rocket_core::{DATA, EMPTY, METHOD, NAMESPACE, PATH, ROOT};

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
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::request_mapping;
/// // use autortr_rocket::prelude::*;
///
/// use rocket::form::Form;
/// use rocket::FromForm;
///
/// #[derive(Debug, FromForm)]
/// struct Account {
///     account: String,
/// }
///
/// // request_mapping
/// // - namespace
/// // - method
/// // - path
/// // - data
///
/// #[request_mapping(method = "get", path = "/get")]
/// fn get_fn() -> &'static str {
///     "Hello, get!"
/// }
///
/// // with namespace
/// #[request_mapping(namespace = "/rocket", method = "get", path = "/namespace")]
/// fn namespace_fn() -> &'static str {
///     "Hello, namespace!"
/// }
///
/// // with data
/// #[request_mapping(
///     namespace = "/rocket",
///     method = "post",
///     path = "/data",
///     data = "<form>"
/// )]
///
/// fn data_fn(form: Form<Account>) -> &'static str {
///     let account: &str = &form.account;
///     println!("form.account: {}", account);
///
///     "Hello, data!"
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

    let (namespace, method, path, data) = match parse_request_mapping_args(parsed_args) {
        Ok((n, m, p, d)) => (n, m, p, d),
        Err(_) => panic!("Invalid arguments to `#[request_mapping]`"),
    };

    let namespace = namespace.unwrap_or_else(|| ROOT.to_string());
    let data = data.unwrap_or_else(|| EMPTY.to_string());

    let route = match method.as_str() {
        "get" => match data.as_str() {
            "_" => quote! { #[rocket::get(#path)] },
            _ => quote! { #[rocket::get(#path, data = #data)] },
        },
        "post" => match data.as_str() {
            "_" => quote! { #[rocket::post(#path)] },
            _ => quote! { #[rocket::post(#path, data = #data)] },
        },
        "put" => match data.as_str() {
            "_" => quote! { #[rocket::put(#path)] },
            _ => quote! { #[rocket::put(#path, data = #data)] },
        },
        "patch" => match data.as_str() {
            "_" => quote! { #[rocket::patch(#path)] },
            _ => quote! { #[rocket::patch(#path, data = #data)] },
        },
        "delete" => match data.as_str() {
            "_" => quote! { #[rocket::delete(#path)] },
            _ => quote! { #[rocket::delete(#path, data = #data)] },
        },
        "head" => match data.as_str() {
            "_" => quote! { #[rocket::head(#path)] },
            _ => quote! { #[rocket::head(#path, data = #data)] },
        },
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
                path: #path.to_string(),
                data: #data.to_string(),
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

// ----------------------------------------------------------------

/// HTTP GET method request mapping. Equivalent to `#[request_mapping(method = "get",...)]`
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::get_mapping;
/// // use autortr_rocket::prelude::*;
///
/// use rocket::form::Form;
/// use rocket::FromForm;
///
/// #[derive(Debug, FromForm)]
/// struct Account {
///     account: String,
/// }
///
/// // get_mapping
/// // - namespace
/// // - path
/// // - data
///
/// // e.g.: 1
/// #[get_mapping("/get")]
/// fn default_get_fn() -> &'static str {
///     "Hello, get!"
/// }
///
/// // e.g.: 2
/// #[get_mapping(path = "/get")]
/// fn get_fn() -> &'static str {
///     "Hello, get!"
/// }
///
/// // e.g.: 3
/// #[get_mapping(
///     namespace = "/rocket",
///     path = "/data",
///     data = "<form>"
/// )]
/// fn data_fn(form: Form<Account>) -> &'static str {
///     let account: &str = &form.account;
///     println!("form.account: {}", account);
///
///     "Hello, data!"
/// }
/// ```
/// @since 0.2.0
#[proc_macro_attribute]
pub fn get_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    http_mapping("get", args, item)
}

/// HTTP POST method request mapping. Equivalent to `#[request_mapping(method = "post",...)]`
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::post_mapping;
/// // use autortr_rocket::prelude::*;
///
/// use rocket::form::Form;
/// use rocket::FromForm;
///
/// #[derive(Debug, FromForm)]
/// struct Account {
///     account: String,
/// }
///
/// // post_mapping
/// // - namespace
/// // - path
/// // - data
///
/// // e.g.: 1
/// #[post_mapping("/post")]
/// fn default_post_fn() -> &'static str {
///     "Hello, post!"
/// }
///
/// // e.g.: 2
/// #[post_mapping(path = "/post")]
/// fn post_fn() -> &'static str {
///     "Hello, post!"
/// }
///
/// // e.g.: 3
/// #[post_mapping(
///     namespace = "/rocket",
///     path = "/data",
///     data = "<form>"
/// )]
/// fn data_fn(form: Form<Account>) -> &'static str {
///     let account: &str = &form.account;
///     println!("form.account: {}", account);
///
///     "Hello, data!"
/// }
/// ```
/// @since 0.2.0
#[proc_macro_attribute]
pub fn post_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    http_mapping("post", args, item)
}

/// HTTP PUT method request mapping. Equivalent to `#[request_mapping(method = "put",...)]`
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::put_mapping;
/// // use autortr_rocket::prelude::*;
///
/// use rocket::form::Form;
/// use rocket::FromForm;
///
/// #[derive(Debug, FromForm)]
/// struct Account {
///     account: String,
/// }
///
/// // put_mapping
/// // - namespace
/// // - path
/// // - data
///
/// // e.g.: 1
/// #[put_mapping("/put")]
/// fn default_put_fn() -> &'static str {
///     "Hello, put!"
/// }
///
/// // e.g.: 2
/// #[put_mapping(path = "/put")]
/// fn put_fn() -> &'static str {
///     "Hello, put!"
/// }
///
/// // e.g.: 3
/// #[put_mapping(
///     namespace = "/rocket",
///     path = "/data",
///     data = "<form>"
/// )]
/// fn data_fn(form: Form<Account>) -> &'static str {
///     let account: &str = &form.account;
///     println!("form.account: {}", account);
///
///     "Hello, data!"
/// }
/// ```
/// @since 0.2.0
#[proc_macro_attribute]
pub fn put_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    http_mapping("put", args, item)
}

/// HTTP PATCH method request mapping. Equivalent to `#[request_mapping(method = "patch",...)]`
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::patch_mapping;
/// // use autortr_rocket::prelude::*;
///
/// use rocket::form::Form;
/// use rocket::FromForm;
///
/// #[derive(Debug, FromForm)]
/// struct Account {
///     account: String,
/// }
///
/// // patch_mapping
/// // - namespace
/// // - path
/// // - data
///
/// // e.g.: 1
/// #[patch_mapping("/patch")]
/// fn default_patch_fn() -> &'static str {
///     "Hello, patch!"
/// }
///
/// // e.g.: 2
/// #[patch_mapping(path = "/patch")]
/// fn patch_fn() -> &'static str {
///     "Hello, patch!"
/// }
///
/// // e.g.: 3
/// #[patch_mapping(
///     namespace = "/rocket",
///     path = "/data",
///     data = "<form>"
/// )]
/// fn data_fn(form: Form<Account>) -> &'static str {
///     let account: &str = &form.account;
///     println!("form.account: {}", account);
///
///     "Hello, data!"
/// }
/// ```
/// @since 0.2.0
#[proc_macro_attribute]
pub fn patch_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    http_mapping("patch", args, item)
}

/// HTTP DELETE method request mapping. Equivalent to `#[request_mapping(method = "delete",...)]`
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::delete_mapping;
/// // use autortr_rocket::prelude::*;
///
/// use rocket::form::Form;
/// use rocket::FromForm;
///
/// #[derive(Debug, FromForm)]
/// struct Account {
///     account: String,
/// }
///
/// // delete_mapping
/// // - namespace
/// // - path
/// // - data
///
/// // e.g.: 1
/// #[delete_mapping("/delete")]
/// fn default_delete_fn() -> &'static str {
///     "Hello, delete!"
/// }
///
/// // e.g.: 2
/// #[delete_mapping(path = "/delete")]
/// fn delete_fn() -> &'static str {
///     "Hello, delete!"
/// }
///
/// // e.g.: 3
/// #[delete_mapping(
///     namespace = "/rocket",
///     path = "/data",
///     data = "<form>"
/// )]
/// fn data_fn(form: Form<Account>) -> &'static str {
///     let account: &str = &form.account;
///     println!("form.account: {}", account);
///
///     "Hello, data!"
/// }
/// ```
/// @since 0.2.0
#[proc_macro_attribute]
pub fn delete_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    http_mapping("delete", args, item)
}

/// HTTP HEAD method request mapping. Equivalent to `#[request_mapping(method = "head",...)]`
///
/// # Examples
///
/// ```rust
/// use autortr_rocket_core::{register_route_mapping, RouteMapping};
/// use autortr_rocket_codegen::head_mapping;
/// // use autortr_rocket::prelude::*;
///
/// // head_mapping
/// // - namespace
/// // - path
///
/// // e.g.: 1
/// #[head_mapping("/head")]
/// fn default_head_fn() -> &'static str {
///     "Hello, head!"
/// }
///
/// // e.g.: 2
/// #[head_mapping(path = "/head")]
/// fn head_fn() -> &'static str {
///     "Hello, head!"
/// }
/// ```
/// @since 0.2.0
#[proc_macro_attribute]
pub fn head_mapping(args: TokenStream, item: TokenStream) -> TokenStream {
    http_mapping("head", args, item)
}

// ----------------------------------------------------------------

#[doc(hidden)]
fn http_mapping(method: &str, args: TokenStream, item: TokenStream) -> TokenStream {
    let parsed_args = parse_macro_input!(args as AttributeArgs);
    let parsed_args_clone = parsed_args.clone();
    let function = parse_macro_input!(item as ItemFn);

    let function_ident = &function.sig.ident;
    let function_name = function_ident.to_string();

    let (namespace, path, data) = match parse_http_mapping_named_args(parsed_args) {
        Ok((n, Some(p), d)) => (n, p, d),
        Ok((n, None, d)) => {
            // #[get_mapping("/get")] ...
            match parse_http_mapping_args(parsed_args_clone) {
                Ok(p) => (n, p, d),
                Err(_) => {
                    let message = format!("Invalid arguments to `#[{}_mapping]`", method);
                    panic!("{}", message)
                }
            }
        }
        Err(_) => {
            let message = format!("Invalid arguments to `#[{}_mapping]`", method);
            panic!("{}", message)
        }
    };

    let namespace = namespace.unwrap_or_else(|| ROOT.to_string());
    let data = data.unwrap_or_else(|| EMPTY.to_string());

    let rkt_method = format_ident!("{}", method);

    let route = match data.as_str() {
        "_" => quote! { #[rocket::#rkt_method(#path)] },
        _ => quote! { #[rocket::#rkt_method(#path, data = #data)] },
    };

    let register_fn_name = format_ident!("_register_{}_{}_", method, function_ident);

    let register_fn = quote! {
        #[ctor::ctor]
        fn #register_fn_name() {
            register_route_mapping(RouteMapping {
                function: #function_name.to_string(),
                namespace: #namespace.to_string(),
                method: #method.to_string(),
                path: #path.to_string(),
                data: #data.to_string(),
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

// ----------------------------------------------------------------

#[doc(hidden)]
fn parse_request_mapping_args(
    args: AttributeArgs,
) -> Result<(Option<String>, String, String, Option<String>), ()> {
    let mut namespace = None;
    let mut method = None;
    let mut path = None;
    let mut data = None;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident(NAMESPACE) {
                    if let Lit::Str(n) = nv.lit {
                        namespace = Some(n.value());
                    }
                } else if nv.path.is_ident(METHOD) {
                    if let Lit::Str(m) = nv.lit {
                        method = Some(m.value());
                    }
                } else if nv.path.is_ident(PATH) {
                    if let Lit::Str(p) = nv.lit {
                        path = Some(p.value());
                    }
                } else if nv.path.is_ident(DATA) {
                    if let Lit::Str(d) = nv.lit {
                        data = Some(d.value());
                    }
                }
            }
            _ => return Err(()),
        }
    }

    match (method, path) {
        (Some(m), Some(p)) => Ok((namespace, m, p, data)),
        _ => Err(()),
    }
}

// ----------------------------------------------------------------

#[doc(hidden)]
fn parse_http_mapping_named_args(
    args: AttributeArgs,
) -> Result<(Option<String>, Option<String>, Option<String>), ()> {
    let mut namespace = None;
    let mut path = None;
    let mut data = None;

    for arg in args {
        match arg {
            NestedMeta::Meta(Meta::NameValue(nv)) => {
                if nv.path.is_ident(NAMESPACE) {
                    if let Lit::Str(n) = nv.lit {
                        namespace = Some(n.value());
                    }
                } else if nv.path.is_ident(PATH) {
                    if let Lit::Str(p) = nv.lit {
                        path = Some(p.value());
                    }
                } else if nv.path.is_ident(DATA) {
                    if let Lit::Str(d) = nv.lit {
                        data = Some(d.value());
                    }
                }
            }
            _ => {}
        }
    }

    Ok((namespace, path, data))
}

#[doc(hidden)]
fn parse_http_mapping_args(args: AttributeArgs) -> Result<String, ()> {
    match args.first() {
        Some(NestedMeta::Lit(Lit::Str(path))) => Ok(path.value()),
        _ => Err(()),
    }
}
