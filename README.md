# autortr

`Autortr` (Auto Router)

`Autortr` is a lightweight Rust library designed for automatic route mapping in web applications, particularly tailored
for seamless integration with the `Rocket` framework. It simplifies the process of defining and managing HTTP routes by
automatically registering functions annotated with custom attributes like `#[request_mapping]`.

## Key Features:

- Automatically registers routes based on custom attributes like `#[request_mapping]`.
- Integrates seamlessly with Rocket's route management.
- Simplifies routing setup and maintenance in Rust web applications.
- Future plans include support for additional `HTTP` methods such
  as `#[get_mapping]`, `#[post_mapping]`, `#[patch_mapping]`, `#[put_mapping]`, and `#[delete_mapping]` macros.
