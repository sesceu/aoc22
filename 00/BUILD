load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

rust_library(
    name = "day_lib",
    srcs = ["day_lib.rs"],
    edition = "2018",
)

rust_binary(
    name = "example",
    srcs = ["main.rs"],
    edition = "2018",
    deps = [],
)

rust_test(
    name = "day_lib_tests",
    srcs = ["day_lib_tests.rs"],
    edition = "2018",
    deps = [":day_lib"],
)
