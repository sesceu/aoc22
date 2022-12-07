load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

http_archive(
    name = "rules_rust",
    sha256 = "324c2a86a8708d30475f324846b35965c432b63a35567ed2b5051b86791ce345",
    urls = ["https://github.com/bazelbuild/rules_rust/releases/download/0.13.0/rules_rust-v0.13.0.tar.gz"],
)
load("@rules_rust//rust:repositories.bzl", "rules_rust_dependencies", "rust_register_toolchains")

rules_rust_dependencies()

rust_register_toolchains()

load("@rules_rust//crate_universe:defs.bzl", "crate", "crates_repository", "render_config")

crates_repository(
    name = "crate_index",
    cargo_lockfile = "//:Cargo.lock",
    lockfile = "//:Cargo.Bazel.lock",
    packages = {
        "regex": crate.spec(
            version = "1.7.0",
        ),
    },
    # Setting the default package name to `""` forces the use of the macros defined in this repository
    # to always use the root package when looking for dependencies or aliases. This should be considered
    # optional as the repository also exposes alises for easy access to all dependencies.
    render_config = render_config(
        default_package_name = "",
    ),
)
load("@crate_index//:defs.bzl", "crate_repositories")

crate_repositories()