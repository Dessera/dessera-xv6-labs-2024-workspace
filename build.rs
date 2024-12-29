use std::env;

fn main() {
    // get LINKER_TEMPLATE from environment
    let linker_template = env::var("LINKER_TEMPLATE").unwrap_or_default();
    // get ULIBS from environment
    let ulibs = env::var("ULIBS").unwrap_or_default();
    // get LINKER_ARGS from environment
    let linker_args = env::var("LINKER_ARGS").unwrap_or_default();

    // pass LINKER_TEMPLATE to linker
    if !linker_template.is_empty() {
        println!("cargo:rustc-link-arg-bins=-T{}", linker_template);
    }
    // pass ULIBS to linker
    for ulib in ulibs.split_whitespace() {
        println!("cargo:rustc-link-arg-bins={}", ulib);
    }
    // pass LINKER_ARGS to linker
    for linker_arg in linker_args.split_whitespace() {
        println!("cargo:rustc-link-arg-bins={}", linker_arg);
    }
}
