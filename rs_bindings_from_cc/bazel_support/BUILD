"""Disclaimer: This project is experimental, under heavy development, and should not
be used yet."""

load(
    "//rs_bindings_from_cc/bazel_support:deps_for_bindings.bzl",
    "deps_for_bindings",
)
load("@bazel_skylib//:bzl_library.bzl", "bzl_library")
load("@bazel_skylib//rules:common_settings.bzl", "bool_flag")

licenses(["notice"])

bzl_library(
    name = "rust_bindings_from_cc_aspect",
    srcs = ["rust_bindings_from_cc_aspect.bzl"],
    visibility = [
        "//:__subpackages__",
    ],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_utils_bzl",
    ],
)

bzl_library(
    name = "with_cc_toolchain_flags",
    srcs = ["with_cc_toolchain_flags.bzl"],
    visibility = ["//fuzzer:__pkg__"],
    deps = [
        "@bazel_tools//tools/build_defs/cc:action_names",
        "@bazel_tools//tools/cpp:toolchain_utils",
    ],
)

bzl_library(
    name = "toolchain_headers_bzl",
    srcs = ["toolchain_headers.bzl"],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_utils_bzl",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_transition",
    srcs = ["rust_bindings_from_cc_transition.bzl"],
)

bzl_library(
    name = "rust_bindings_from_cc_binary_bzl",
    srcs = ["rust_bindings_from_cc_binary.bzl"],
    deps = [":rust_bindings_from_cc_transition"],
)

bzl_library(
    name = "deps_for_bindings_bzl",
    srcs = ["deps_for_bindings.bzl"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":providers_bzl",
        ":rust_bindings_from_cc_transition",
        "@rules_rust//rust/private:bzl_lib",
    ],
)

bzl_library(
    name = "rust_bindings_from_cc_utils_bzl",
    srcs = ["rust_bindings_from_cc_utils.bzl"],
    visibility = ["//:__subpackages__"],
    deps = [
        ":compile_cc_bzl",
        ":compile_rust_bzl",
        ":generate_bindings_bzl",
        ":providers_bzl",
        "@bazel_tools//tools/cpp:toolchain_utils",
    ],
)

bzl_library(
    name = "providers_bzl",
    srcs = ["providers.bzl"],
)

bzl_library(
    name = "compile_cc_bzl",
    srcs = ["compile_cc.bzl"],
)

bzl_library(
    name = "generate_bindings_bzl",
    srcs = ["generate_bindings.bzl"],
    deps = ["@bazel_tools//tools/build_defs/cc:action_names"],
)

bzl_library(
    name = "compile_rust_bzl",
    srcs = ["compile_rust.bzl"],
    deps = ["@rules_rust//rust/private:bzl_lib"],
)

alias(
    name = "rust_bindings_from_cc_target",
    actual = select({
        ":use_prebuilt_rs_bindings_from_cc_setting": "prebuilt_rs_bindings_from_cc",
        ":use_fake_bindings_generator_setting": ":fake_rust_bindings_from_cc",
        "//conditions:default": "//rs_bindings_from_cc:rs_bindings_from_cc",
    }),
    visibility = ["//visibility:public"],
)

config_setting(
    name = "use_fake_bindings_generator_setting",
    flag_values = {
        ":use_actual_bindings_generator": "False",
    },
)

config_setting(
    name = "use_prebuilt_rs_bindings_from_cc_setting",
    flag_values = {
        ":use_prebuilt_rs_bindings_from_cc": "True",
    },
)

bool_flag(
    name = "use_actual_bindings_generator",
    build_setting_default = True,
    visibility = ["//visibility:public"],
)

bool_flag(
    name = "use_prebuilt_rs_bindings_from_cc",
    build_setting_default = False,
    visibility = ["//visibility:public"],
)

sh_binary(
    name = "fake_rust_bindings_from_cc",
    srcs = ["fake_rust_bindings_from_cc.sh"],
)

alias(
    name = "deps_for_bindings",
    actual = select({
        ":remove_deps": ":empty_deps",
        "//conditions:default": "//rs_bindings_from_cc:deps_for_bindings",
    }),
    visibility = ["//visibility:public"],
)

config_setting(
    name = "remove_deps",
    flag_values = {
        ":use_actual_deps": "False",
    },
)

bool_flag(
    name = "use_actual_deps",
    build_setting_default = True,
    visibility = ["//visibility:public"],
)

deps_for_bindings(
    name = "empty_deps",
)

bool_flag(
    name = "generate_error_report",
    build_setting_default = False,
    visibility = ["//visibility:public"],
)
