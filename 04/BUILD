load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

filegroup(
    name = "input_data",
    data = [
        "input.txt",
    ],
)

rust_library(
    name = "day_lib",
    srcs = ["day_lib.rs"],
    edition = "2018",
)

rust_binary(
    name = "day_04",
    srcs = ["main.rs"],
    data = [":input_data"],
    edition = "2018",
    deps = [
        ":day_lib",
        "@crate_index//:regex",
    ],
)

rust_test(
    name = "day_lib_tests",
    srcs = ["day_lib_tests.rs"],
    edition = "2018",
    deps = [":day_lib"],
)
