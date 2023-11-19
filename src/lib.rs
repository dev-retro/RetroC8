use core::metadata::Metadata;
pub mod core;

pub extern fn metadata() -> Metadata {
    Metadata {
        platform_name: String::from("CHIP-8"),
        platform_description: String::from("CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker"),
        platform_release_date: String::from("1975")
    }
}

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type Metadata;

        fn metadata();
    }
}