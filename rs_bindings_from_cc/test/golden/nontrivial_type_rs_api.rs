// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:nontrivial_type_cc

#![rustfmt::skip]
#![feature(custom_inner_attributes, negative_impls, type_alias_impl_trait)]
#![allow(stable_features)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![deny(warnings)]

// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

/// Nontrivial due to (declared, but not yet defined) user-specified constructor
/// and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
///
/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=15
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct Nontrivial {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nontrivial"), crate::Nontrivial);

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=16
impl ::ctor::CtorNew<()> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=17
impl ::ctor::CtorNew<i32> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Ei(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(i32,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=18
impl ::ctor::CtorNew<(i32, i32)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1Eii(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    field,
                    unused,
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=19
impl<'b> ::ctor::CtorNew<&'b Self> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=20
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NontrivialC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for Nontrivial {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=21
impl<'b> ::ctor::Assign<&'b Self> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=22
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=23
impl ::ctor::Assign<i32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN10NontrivialaSEi(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=25
impl ::ctor::Assign<f32> for Nontrivial {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: f32) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN10NontrivialaSEf(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        self,
                        __param_0,
                    );
                }
            ));
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=26
impl ::ctor::PinnedDrop for Nontrivial {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NontrivialD1Ev(self)
    }
}

impl Nontrivial {
    /// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=28
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nontrivial14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to (inline) user-specified constructor and destructor.
///
/// This makes it nontrivial for calls (so not trivially relocatable), as well
/// as specifically giving it a nontrivial move constructor and destructor.
///
/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=37
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct NontrivialInline {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialInline"),
    crate::NontrivialInline
);

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=38
impl ::ctor::CtorNew<()> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=39
impl ::ctor::CtorNew<i32> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: i32) -> Self::CtorType {
        let field = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Ei(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    field,
                );
            })
        }
    }
}
impl ::ctor::CtorNew<(i32,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<i32>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=40
impl ::ctor::CtorNew<(i32, i32)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: (i32, i32)) -> Self::CtorType {
        let (field, unused) = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1Eii(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    field,
                    unused,
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=41
impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=42
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN16NontrivialInlineC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialInline {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=43
impl<'b> ::ctor::Assign<&'b Self> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=44
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEOS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=45
impl ::ctor::Assign<i32> for NontrivialInline {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN16NontrivialInlineaSEi(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=46
impl ::ctor::PinnedDrop for NontrivialInline {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN16NontrivialInlineD1Ev(self)
    }
}

impl NontrivialInline {
    /// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=48
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN16NontrivialInline14MemberFunctionEv(self) }
    }
}

/// Nontrivial due to member variables.
///
/// This changes how the destructor / drop impl work -- instead of calling
/// the destructor for NontrivialMembers, it just calls the destructors for
/// each field.
///
/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct NontrivialMembers {
    pub nontrivial_member: ::std::mem::ManuallyDrop<crate::Nontrivial>,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialMembers"),
    crate::NontrivialMembers
);

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
impl ::ctor::CtorNew<()> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN17NontrivialMembersC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialMembers {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
impl ::ctor::PinnedDrop for NontrivialMembers {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN17NontrivialMembersD1Ev(self)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
impl<'b> ::ctor::Assign<&'b Self> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=58
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialMembers {
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        __param_0: ::ctor::RvalueReference<'b, Self>,
    ) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialMembersaSEOS_(self, __param_0);
        }
    }
}

/// Nontrivial, but trivially relocatable and final (and therefore Unpin).
///
/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=63
#[repr(C)]
pub struct NontrivialUnpin {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    pub field: i32,
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialUnpin"),
    crate::NontrivialUnpin
);

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=64
impl Default for NontrivialUnpin {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=65
// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// Not yet supported type of constructor parameter

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=66
// Error while generating bindings for item 'NontrivialUnpin::NontrivialUnpin':
// More than 1 constructor parameter is not supported yet

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=67
impl Clone for NontrivialUnpin {
    #[inline(always)]
    fn clone<'b>(&'b self) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1ERKS_(&mut tmp, self);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=68
impl<'b> From<::ctor::RvalueReference<'b, Self>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=69
impl<'b> From<::ctor::RvalueReference<'b, crate::Nontrivial>> for NontrivialUnpin {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=70
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=71
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=72
// Error while generating bindings for item 'NontrivialUnpin::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=73
impl Drop for NontrivialUnpin {
    #[inline(always)]
    fn drop<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpinD1Ev(self) }
    }
}

impl NontrivialUnpin {
    /// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=75
    #[inline(always)]
    pub fn MemberFunction<'a>(&'a mut self) {
        unsafe { crate::detail::__rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv(self) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=80
#[inline(always)]
pub fn TakesByValue(nontrivial: impl ::ctor::Ctor<Output = crate::Nontrivial>) {
    unsafe {
        crate::detail::__rust_thunk___Z12TakesByValue10Nontrivial(
            ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=81
#[inline(always)]
pub fn TakesByValueInline(nontrivial: impl ::ctor::Ctor<Output = crate::NontrivialInline>) {
    unsafe {
        crate::detail::__rust_thunk___Z18TakesByValueInline16NontrivialInline(
            ::std::pin::Pin::into_inner_unchecked(::ctor::emplace!(nontrivial)),
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=82
#[inline(always)]
pub fn TakesByValueUnpin(nontrivial: crate::NontrivialUnpin) {
    unsafe { crate::detail::__rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(nontrivial) }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=84
#[inline(always)]
pub fn ReturnsByValue() -> impl ::ctor::Ctor<Output = crate::Nontrivial> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nontrivial>>| {
                crate::detail::__rust_thunk___Z14ReturnsByValuev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=85
#[inline(always)]
pub fn ReturnsByValueUnpin() -> crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z19ReturnsByValueUnpinv() }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=87
#[inline(always)]
pub fn TakesByConstReference<'a>(nontrivial: &'a crate::Nontrivial) -> &'a crate::Nontrivial {
    unsafe { crate::detail::__rust_thunk___Z21TakesByConstReferenceRK10Nontrivial(nontrivial) }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=88
#[inline(always)]
pub fn TakesByReference<'a>(
    nontrivial: ::std::pin::Pin<&'a mut crate::Nontrivial>,
) -> ::std::pin::Pin<&'a mut crate::Nontrivial> {
    unsafe { crate::detail::__rust_thunk___Z16TakesByReferenceR10Nontrivial(nontrivial) }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=90
#[inline(always)]
pub fn TakesByConstReferenceUnpin<'a>(
    nontrivial: &'a crate::NontrivialUnpin,
) -> &'a crate::NontrivialUnpin {
    unsafe {
        crate::detail::__rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin(nontrivial)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=92
#[inline(always)]
pub fn TakesByReferenceUnpin<'a>(
    nontrivial: &'a mut crate::NontrivialUnpin,
) -> &'a mut crate::NontrivialUnpin {
    unsafe { crate::detail::__rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin(nontrivial) }
}

/// Finally, testing for strange by-value APIs.
///
/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=95
#[::ctor::recursively_pinned]
#[repr(C)]
pub struct NontrivialByValue {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("NontrivialByValue"),
    crate::NontrivialByValue
);

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=96
impl<'b> ::ctor::CtorNew<&'b Self> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let other = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN17NontrivialByValueC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    other,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=97
impl<'b> ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: ::ctor::RvalueReference<'b, Self>) -> Self::CtorType {
        let other = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN17NontrivialByValueC1EOS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    other,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(::ctor::RvalueReference<'b, Self>,)> for NontrivialByValue {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (::ctor::RvalueReference<'b, Self>,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<::ctor::RvalueReference<'b, Self>>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=98
impl<'b> ::ctor::Assign<&'b Self> for NontrivialByValue {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, other: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSERKS_(self, other);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=99
impl<'b> ::ctor::Assign<::ctor::RvalueReference<'b, Self>> for NontrivialByValue {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, other: ::ctor::RvalueReference<'b, Self>) {
        unsafe {
            crate::detail::__rust_thunk___ZN17NontrivialByValueaSEOS_(self, other);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=101
impl<'other> ::ctor::Assign<::ctor::RvalueReference<'other, crate::Nontrivial>>
    for NontrivialByValue
{
    #[inline(always)]
    fn assign<'a>(
        self: ::std::pin::Pin<&'a mut Self>,
        other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
    ) {
        unsafe {
            let _ = ::ctor::emplace!(::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                    crate::detail::__rust_thunk___ZN17NontrivialByValueaSE10Nontrivial(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        self,
                        other,
                    );
                }
            ));
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=102
// Error while generating bindings for item 'NontrivialByValue::operator==':
// operator== where lhs operand is not record nor const reference to record

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=105
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C)]
pub struct Nonmovable {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Nonmovable"), crate::Nonmovable);

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=106
impl ::ctor::CtorNew<()> for Nonmovable {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN10NonmovableC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=109
impl ::ctor::PinnedDrop for Nonmovable {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN10NonmovableD1Ev(self)
    }
}

impl Nonmovable {
    /// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=111
    #[inline(always)]
    pub fn MemberFunction<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        unsafe { crate::detail::__rust_thunk___ZN10Nonmovable14MemberFunctionEv(self) }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=114
// Error while generating bindings for item 'TakesNonmovableByValue':
// Non-movable, non-trivial_abi type 'crate :: Nonmovable' is not supported by value as parameter #0

/// Generated from: rs_bindings_from_cc/test/golden/nontrivial_type.h;l=115
#[inline(always)]
pub fn ReturnsNonmovableByValue() -> impl ::ctor::Ctor<Output = crate::Nonmovable> {
    unsafe {
        ::ctor::FnCtor::new(
            move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<crate::Nonmovable>>| {
                crate::detail::__rust_thunk___Z24ReturnsNonmovableByValuev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            },
        )
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_NONTRIVIAL_TYPE_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        #[link_name = "_ZN10NontrivialC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialC1Ei"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Ei<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
        );
        #[link_name = "_ZN10NontrivialC1Eii"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1Eii<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            field: i32,
            unused: i32,
        );
        #[link_name = "_ZN10NontrivialC1ERKS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        );
        #[link_name = "_ZN10NontrivialC1EOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NontrivialaSERKS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: &'b crate::Nontrivial,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEOS_"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_ZN10NontrivialaSEi"]
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEi<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: i32,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        pub(crate) fn __rust_thunk___ZN10NontrivialaSEf<'a>(
            __return: &mut ::std::mem::MaybeUninit<crate::Nontrivial>,
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
            __param_0: f32,
        );
        #[link_name = "_ZN10NontrivialD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NontrivialD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
        );
        #[link_name = "_ZN10Nontrivial14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nontrivial14MemberFunctionEv<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nontrivial>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Ei<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1Eii<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            field: i32,
            unused: i32,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: &'b crate::NontrivialInline,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialInline>,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineaSEi<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
            __param_0: i32,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialInline>;
        pub(crate) fn __rust_thunk___ZN16NontrivialInlineD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN16NontrivialInline14MemberFunctionEv<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialInline>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialMembers>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: &'b crate::NontrivialMembers,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialMembers>;
        pub(crate) fn __rust_thunk___ZN17NontrivialMembersaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialMembers>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialMembers>,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialMembers>;
        #[link_name = "_ZN15NontrivialUnpinC1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1ERKS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: &'b crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EOS_"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ::ctor::RvalueReference<'b, crate::NontrivialUnpin>,
        );
        #[link_name = "_ZN15NontrivialUnpinC1EO10Nontrivial"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinC1EO10Nontrivial<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialUnpin>,
            __param_0: ::ctor::RvalueReference<'b, crate::Nontrivial>,
        );
        #[link_name = "_ZN15NontrivialUnpinD1Ev"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpinD1Ev<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        #[link_name = "_ZN15NontrivialUnpin14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN15NontrivialUnpin14MemberFunctionEv<'a>(
            __this: &'a mut crate::NontrivialUnpin,
        );
        pub(crate) fn __rust_thunk___Z12TakesByValue10Nontrivial(
            nontrivial: &mut crate::Nontrivial,
        );
        pub(crate) fn __rust_thunk___Z18TakesByValueInline16NontrivialInline(
            nontrivial: &mut crate::NontrivialInline,
        );
        #[link_name = "_Z17TakesByValueUnpin15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z17TakesByValueUnpin15NontrivialUnpin(
            nontrivial: crate::NontrivialUnpin,
        );
        pub(crate) fn __rust_thunk___Z14ReturnsByValuev(
            __return: &mut ::std::mem::MaybeUninit<crate::Nontrivial>,
        );
        #[link_name = "_Z19ReturnsByValueUnpinv"]
        pub(crate) fn __rust_thunk___Z19ReturnsByValueUnpinv() -> crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByConstReferenceRK10Nontrivial"]
        pub(crate) fn __rust_thunk___Z21TakesByConstReferenceRK10Nontrivial<'a>(
            nontrivial: &'a crate::Nontrivial,
        ) -> &'a crate::Nontrivial;
        #[link_name = "_Z16TakesByReferenceR10Nontrivial"]
        pub(crate) fn __rust_thunk___Z16TakesByReferenceR10Nontrivial<'a>(
            nontrivial: ::std::pin::Pin<&'a mut crate::Nontrivial>,
        ) -> ::std::pin::Pin<&'a mut crate::Nontrivial>;
        #[link_name = "_Z26TakesByConstReferenceUnpinRK15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z26TakesByConstReferenceUnpinRK15NontrivialUnpin<'a>(
            nontrivial: &'a crate::NontrivialUnpin,
        ) -> &'a crate::NontrivialUnpin;
        #[link_name = "_Z21TakesByReferenceUnpinR15NontrivialUnpin"]
        pub(crate) fn __rust_thunk___Z21TakesByReferenceUnpinR15NontrivialUnpin<'a>(
            nontrivial: &'a mut crate::NontrivialUnpin,
        ) -> &'a mut crate::NontrivialUnpin;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
            other: &'b crate::NontrivialByValue,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        );
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: &'b crate::NontrivialByValue,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialByValue>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSEOS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'b, crate::NontrivialByValue>,
        ) -> ::std::pin::Pin<&'a mut crate::NontrivialByValue>;
        pub(crate) fn __rust_thunk___ZN17NontrivialByValueaSE10Nontrivial<'a, 'other>(
            __return: &mut ::std::mem::MaybeUninit<crate::NontrivialByValue>,
            __this: ::std::pin::Pin<&'a mut crate::NontrivialByValue>,
            other: ::ctor::RvalueReference<'other, crate::Nontrivial>,
        );
        #[link_name = "_ZN10NonmovableC1Ev"]
        pub(crate) fn __rust_thunk___ZN10NonmovableC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Nonmovable>,
        );
        #[link_name = "_ZN10NonmovableD1Ev"]
        pub(crate) fn __rust_thunk___ZN10NonmovableD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nonmovable>,
        );
        #[link_name = "_ZN10Nonmovable14MemberFunctionEv"]
        pub(crate) fn __rust_thunk___ZN10Nonmovable14MemberFunctionEv<'a>(
            __this: ::std::pin::Pin<&'a mut crate::Nonmovable>,
        );
        pub(crate) fn __rust_thunk___Z24ReturnsNonmovableByValuev(
            __return: &mut ::std::mem::MaybeUninit<crate::Nonmovable>,
        );
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::Nontrivial>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::Nontrivial>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nontrivial: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nontrivial: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::Nontrivial, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NontrivialInline>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialInline>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialInline: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialInline: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialInline, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NontrivialMembers>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialMembers>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialMembers: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialMembers: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialMembers, nontrivial_member) == 0);

const _: () = assert!(::std::mem::size_of::<crate::NontrivialUnpin>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialUnpin>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialUnpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::NontrivialUnpin: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::NontrivialUnpin, field) == 0);
const _: () = {
    static_assertions::assert_impl_all!(i32: Copy);
};

const _: () = assert!(::std::mem::size_of::<crate::NontrivialByValue>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::NontrivialByValue>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::NontrivialByValue: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::Nonmovable>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::Nonmovable>() == 1);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Nonmovable: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Nonmovable: Drop);
};
