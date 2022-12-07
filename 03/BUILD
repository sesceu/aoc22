load("@rules_rust//rust:defs.bzl", "rust_binary", "rust_library", "rust_test")

filegroup(
    name = "input_data",
    data = [
        "input.txt",
        "input2.txt",
    ],
)

rust_library(
    name = "day_03_lib",
    srcs = ["day_03_lib.rs"],
    edition = "2018",
)

rust_binary(
    name = "day_03_1",
    srcs = ["main.rs"],
    data = [":input_data"],
    edition = "2018",
    deps = [":day_03_lib"],
)

rust_binary(
    name = "day_03_2",
    srcs = ["main_2.rs"],
    data = [":input_data"],
    edition = "2018",
    deps = [":day_03_lib"],
)

rust_test(
    name = "day_03_tests",
    srcs = ["day_03_tests.rs"],
    edition = "2018",
    deps = [":day_03_lib"],
)
