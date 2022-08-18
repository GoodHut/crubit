// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

#ifndef CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_H_
#define CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_H_

#include <utility>

#include "clang/AST/Expr.h"
#include "clang/Analysis/FlowSensitive/DataflowEnvironment.h"
#include "clang/Analysis/FlowSensitive/Value.h"

namespace clang {
namespace tidy {
namespace nullability {

/// Returns the `PointerValue` allocated to `PointerExpr` if available,
/// otherwise returns nullptr.
dataflow::PointerValue* getPointerValueFromExpr(
    const Expr* PointerExpr, const dataflow::Environment& Env);

/// Returns the properties representing the nullness information of a pointer.
///
/// The first boolean indicates if the pointer's nullability is known.
/// The second boolean indicates if the pointer's value is not null.
std::pair<dataflow::AtomicBoolValue&, dataflow::AtomicBoolValue&>
getPointerNullState(const dataflow::PointerValue& PointerVal,
                    const dataflow::Environment& Env);

/// Sets the nullness properties on `PointerVal` if not already initialised.
///
/// The boolean properties may be constrained by specifying `KnownConstraint`
/// and `NotNullConstraint`. Otherwise, the properties are set to freshly
/// created atomic booleans.
void initPointerNullState(dataflow::PointerValue& PointerVal,
                          dataflow::Environment& Env,
                          dataflow::BoolValue* KnownConstraint = nullptr,
                          dataflow::BoolValue* NotNullConstraint = nullptr);

/// Sets the nullness properties on `PointerVal` representing a nullptr if not
/// already initialised.
///
/// `Known` is constrained to true, `NotNull` is constrained to false.
inline void initNullPointer(dataflow::PointerValue& PointerVal,
                            dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true),
                       /*NotNullConstraint=*/&Env.getBoolLiteralValue(false));
}

/// Sets the nullness properties on `PointerVal` representing a pointer that is
/// not null if not already initialised.
///
/// `Known` is constrained to true, `NotNull` is constrained to true.
inline void initNotNullPointer(dataflow::PointerValue& PointerVal,
                               dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true),
                       /*NotNullConstraint=*/&Env.getBoolLiteralValue(true));
}

/// Sets the nullness properties on `PointerVal` representing a pointer that is
/// nullable if not already initialised.
///
/// `Known` is constrained to true, `NotNull` is unconstrained.
inline void initNullablePointer(dataflow::PointerValue& PointerVal,
                                dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(true));
}

/// Sets the nullness properties on `PointerVal` representing a pointer with
/// unknown nullability if not already initialised.
///
/// `Known` is constrained to false, `NotNull` is unconstrained.
inline void initUnknownPointer(dataflow::PointerValue& PointerVal,
                               dataflow::Environment& Env) {
  initPointerNullState(PointerVal, Env,
                       /*KnownConstraint=*/&Env.getBoolLiteralValue(false));
}

}  // namespace nullability
}  // namespace tidy
}  // namespace clang

#endif  // CRUBIT_NULLABILITY_VERIFICATION_POINTER_NULLABILITY_H_