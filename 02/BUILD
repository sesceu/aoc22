load("@rules_rust//rust:defs.bzl", "rust_binary")

filegroup(
    name = "input_data",
    data = ["input.txt"],
)

rust_binary(
    name = "day_02",
    srcs = ["main.rs"],
    data = [":input_data"],
    edition = "2018",
    deps = [],
)