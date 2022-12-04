load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

filegroup(
    name = "input_data",
    data = ["input.txt"],
)

rust_library(
    name = "day_02_lib",
    srcs = ["day_02_lib.rs"],
    edition = "2018",
)

rust_binary(
    name = "day_02",
    srcs = ["main.rs"],
    data = [":input_data"],
    edition = "2018",
    deps = [":day_02_lib"],
)