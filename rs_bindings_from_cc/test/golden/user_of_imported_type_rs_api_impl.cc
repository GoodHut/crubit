// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#include <cstddef>
#include <memory>
#include "rs_bindings_from_cc/test/golden/user_of_imported_type.h"

namespace {
template <class T, class... Args>
constexpr T* construct_at(T* p, Args&&... args) {
  return ::new (const_cast<void*>(static_cast<const volatile void*>(p)))
      T(std ::forward<Args>(args)...);
}
}  // namespace
extern "C" void __rust_thunk___ZN18UserOfImportedTypeC1Ev(
    UserOfImportedType* __this) {
  construct_at(__this);
}
extern "C" void __rust_thunk___ZN18UserOfImportedTypeD1Ev(
    UserOfImportedType* __this) {
  std ::destroy_at(__this);
}

static_assert(sizeof(UserOfImportedType) == 8);
static_assert(alignof(UserOfImportedType) == 8);
static_assert(offsetof(UserOfImportedType, trivial) * 8 == 0);
