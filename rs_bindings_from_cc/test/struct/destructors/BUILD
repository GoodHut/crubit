"""End-to-end test for destructors."""

load("@rules_rust//rust:defs.bzl", "rust_test")

licenses(["notice"])

cc_library(
    name = "field_destruction_order",
    srcs = ["field_destruction_order.cc"],
    hdrs = ["field_destruction_order.h"],
)

rust_test(
    name = "main",
    srcs = ["test.rs"],
    cc_deps = [":field_destruction_order"],
)
