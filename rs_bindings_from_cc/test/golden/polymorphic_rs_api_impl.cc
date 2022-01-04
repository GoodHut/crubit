// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/support/cxx20_backports.h"
#include "rs_bindings_from_cc/test/golden/polymorphic.h"

extern "C" void __rust_thunk___ZN16PolymorphicClassD1Ev(
    PolymorphicClass* __this) {
  std ::destroy_at(__this);
}
extern "C" void __rust_thunk___ZN16PolymorphicClassC1Ev(
    PolymorphicClass* __this) {
  rs_api_impl_support ::construct_at(__this);
}

static_assert(sizeof(PolymorphicClass) == 8);
static_assert(alignof(PolymorphicClass) == 8);