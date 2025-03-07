// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

// Automatically @generated Rust bindings for the following C++ target:
// //rs_bindings_from_cc/test/golden:operators_cc

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

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=12
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableConstMember {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableConstMember"),
    crate::AddableConstMember
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=12
impl Default for AddableConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=12
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableConstMemberC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=12
// Error while generating bindings for item 'AddableConstMember::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=12
// Error while generating bindings for item 'AddableConstMember::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=14
impl<'a, 'b> ::std::ops::Add<&'b crate::AddableConstMember> for &'a crate::AddableConstMember {
    type Output = crate::AddableConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMember) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK18AddableConstMemberplERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=20
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableNonConstMember {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableNonConstMember"),
    crate::AddableNonConstMember
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=20
impl Default for AddableNonConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=20
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableNonConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN21AddableNonConstMemberC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=20
// Error while generating bindings for item 'AddableNonConstMember::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=20
// Error while generating bindings for item 'AddableNonConstMember::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=22
impl<'a, 'b> ::std::ops::Add<&'b crate::AddableNonConstMember>
    for &'a mut crate::AddableNonConstMember
{
    type Output = crate::AddableNonConstMember;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableNonConstMember) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZN21AddableNonConstMemberplERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=28
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableFriend {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(forward_declare::symbol!("AddableFriend"), crate::AddableFriend);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=28
impl Default for AddableFriend {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=28
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFriend {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13AddableFriendC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=28
// Error while generating bindings for item 'AddableFriend::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=28
// Error while generating bindings for item 'AddableFriend::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=30
impl<'a, 'b> ::std::ops::Add<&'b crate::AddableFriend> for &'a crate::AddableFriend {
    type Output = crate::AddableFriend;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableFriend) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK13AddableFriendS1_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=37
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddableFree {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("AddableFree"), crate::AddableFree);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=37
impl Default for AddableFree {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11AddableFreeC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=37
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableFree {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN11AddableFreeC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=37
// Error while generating bindings for item 'AddableFree::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=37
// Error while generating bindings for item 'AddableFree::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=38
impl<'a, 'b> ::std::ops::Add<&'b crate::AddableFree> for &'a crate::AddableFree {
    type Output = crate::AddableFree;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableFree) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK11AddableFreeS1_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=39
impl<'a, 'b> ::std::ops::Add<&'b mut crate::AddableFree> for &'a mut crate::AddableFree {
    type Output = crate::AddableFree;
    #[inline(always)]
    fn add(self, rhs: &'b mut crate::AddableFree) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplR11AddableFreeS0_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=40
impl ::std::ops::Add<Self> for AddableFree {
    type Output = crate::AddableFree;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___Zpl11AddableFreeS_(self, rhs) }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=41
// Error while generating bindings for item 'operator+':
// Not yet supported for rvalue references (b/219826128)

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=43
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Overloaded {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("Overloaded"), crate::Overloaded);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=43
impl Default for Overloaded {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=43
impl<'b> From<::ctor::RvalueReference<'b, Self>> for Overloaded {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN10OverloadedC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=43
// Error while generating bindings for item 'Overloaded::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=43
// Error while generating bindings for item 'Overloaded::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=44
impl<'a> ::std::ops::Add<i16> for &'a crate::Overloaded {
    type Output = i32;
    #[inline(always)]
    fn add(self, rhs: i16) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadeds(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=45
impl<'a> ::std::ops::Add<i32> for &'a crate::Overloaded {
    type Output = i32;
    #[inline(always)]
    fn add(self, rhs: i32) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZplRK10Overloadedi(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=47
#[derive(Clone, Copy)]
#[repr(C)]
pub struct IncompatibleLHS {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("IncompatibleLHS"),
    crate::IncompatibleLHS
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=47
impl Default for IncompatibleLHS {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=47
impl<'b> From<::ctor::RvalueReference<'b, Self>> for IncompatibleLHS {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN15IncompatibleLHSC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=47
// Error while generating bindings for item 'IncompatibleLHS::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=47
// Error while generating bindings for item 'IncompatibleLHS::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=48
// Error while generating bindings for item 'operator+':
// Expected first parameter to be a record or reference

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=49
// Error while generating bindings for item 'operator+':
// Expected first parameter referent to be a record

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=51
#[derive(Clone, Copy)]
#[repr(C, align(4))]
pub struct AddableReturnsVoid {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableReturnsVoid"),
    crate::AddableReturnsVoid
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=51
impl Default for AddableReturnsVoid {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=51
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddableReturnsVoid {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddableReturnsVoidC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=51
// Error while generating bindings for item 'AddableReturnsVoid::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=51
// Error while generating bindings for item 'AddableReturnsVoid::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=53
impl<'a, 'b> ::std::ops::Add<&'b crate::AddableReturnsVoid> for &'a crate::AddableReturnsVoid {
    type Output = ();
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableReturnsVoid) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK18AddableReturnsVoidplERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=59
#[::ctor::recursively_pinned(PinnedDrop)]
#[repr(C, align(4))]
pub struct AddableConstMemberNonunpin {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 0],
    /// Reason for representing this field as a blob of bytes:
    /// Types of non-public C++ fields can be elided away
    pub(crate) field_: [::std::mem::MaybeUninit<u8>; 4],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddableConstMemberNonunpin"),
    crate::AddableConstMemberNonunpin
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=59
impl ::ctor::CtorNew<()> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self>;
    #[inline(always)]
    fn ctor_new(args: ()) -> Self::CtorType {
        let () = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1Ev(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                );
            })
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=59
impl<'b> ::ctor::CtorNew<&'b Self> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: &'b Self) -> Self::CtorType {
        let __param_0 = args;
        unsafe {
            ::ctor::FnCtor::new(move |dest: ::std::pin::Pin<&mut ::std::mem::MaybeUninit<Self>>| {
                crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_(
                    ::std::pin::Pin::into_inner_unchecked(dest),
                    __param_0,
                );
            })
        }
    }
}
impl<'b> ::ctor::CtorNew<(&'b Self,)> for AddableConstMemberNonunpin {
    type CtorType = impl ::ctor::Ctor<Output = Self> + ::ctor::Captures<'b>;
    #[inline(always)]
    fn ctor_new(args: (&'b Self,)) -> Self::CtorType {
        let (arg,) = args;
        <Self as ::ctor::CtorNew<&'b Self>>::ctor_new(arg)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=59
impl<'b> ::ctor::Assign<&'b Self> for AddableConstMemberNonunpin {
    #[inline(always)]
    fn assign<'a>(self: ::std::pin::Pin<&'a mut Self>, __param_0: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_(self, __param_0);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=61
impl<'a, 'b> ::std::ops::Add<&'b crate::AddableConstMemberNonunpin>
    for &'a crate::AddableConstMemberNonunpin
{
    type Output = impl ::ctor::Ctor<Output = crate::AddableConstMemberNonunpin>
        + ::ctor::Captures<'a>
        + ::ctor::Captures<'b>;
    #[inline(always)]
    fn add(self, rhs: &'b crate::AddableConstMemberNonunpin) -> Self::Output {
        unsafe {
            ::ctor::FnCtor::new(
                move |dest: ::std::pin::Pin<
                    &mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
                >| {
                    crate::detail::__rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_(
                        ::std::pin::Pin::into_inner_unchecked(dest),
                        self,
                        rhs,
                    );
                },
            )
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=63
impl ::ctor::PinnedDrop for AddableConstMemberNonunpin {
    #[inline(always)]
    unsafe fn pinned_drop<'a>(self: ::std::pin::Pin<&'a mut Self>) {
        crate::detail::__rust_thunk___ZN26AddableConstMemberNonunpinD1Ev(self)
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=69
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignMemberInt {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignMemberInt"),
    crate::AddAssignMemberInt
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=69
impl Default for AddAssignMemberInt {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=69
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignMemberInt {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=69
// Error while generating bindings for item 'AddAssignMemberInt::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=69
// Error while generating bindings for item 'AddAssignMemberInt::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=70
impl ::std::ops::AddAssign<i32> for AddAssignMemberInt {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: i32) {
        unsafe {
            crate::detail::__rust_thunk___ZN18AddAssignMemberIntpLEi(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=73
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignMemberByConstRef {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignMemberByConstRef"),
    crate::AddAssignMemberByConstRef
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=73
impl Default for AddAssignMemberByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=73
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=73
// Error while generating bindings for item 'AddAssignMemberByConstRef::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=73
// Error while generating bindings for item 'AddAssignMemberByConstRef::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=74
impl<'b> ::std::ops::AddAssign<&'b Self> for AddAssignMemberByConstRef {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignMemberByConstRefpLERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=77
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignFreeByConstRef {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFreeByConstRef"),
    crate::AddAssignFreeByConstRef
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=77
impl Default for AddAssignFreeByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=77
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN23AddAssignFreeByConstRefC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=77
// Error while generating bindings for item 'AddAssignFreeByConstRef::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=77
// Error while generating bindings for item 'AddAssignFreeByConstRef::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=78
// Error while generating bindings for item 'operator+=':
// Not yet supported for pointers with unknown lifetime (b/219826128)

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=81
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignFreeByValue {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFreeByValue"),
    crate::AddAssignFreeByValue
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=81
impl Default for AddAssignFreeByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=81
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFreeByValue {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN20AddAssignFreeByValueC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=81
// Error while generating bindings for item 'AddAssignFreeByValue::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=81
// Error while generating bindings for item 'AddAssignFreeByValue::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=82
impl ::std::ops::AddAssign<Self> for AddAssignFreeByValue {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR20AddAssignFreeByValueS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=85
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignFriendByConstRef {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFriendByConstRef"),
    crate::AddAssignFriendByConstRef
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=85
impl Default for AddAssignFriendByConstRef {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=85
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByConstRef {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN25AddAssignFriendByConstRefC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=85
// Error while generating bindings for item 'AddAssignFriendByConstRef::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=85
// Error while generating bindings for item 'AddAssignFriendByConstRef::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=90
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignFriendByValue {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignFriendByValue"),
    crate::AddAssignFriendByValue
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=90
impl Default for AddAssignFriendByValue {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=90
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignFriendByValue {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN22AddAssignFriendByValueC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=90
// Error while generating bindings for item 'AddAssignFriendByValue::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=90
// Error while generating bindings for item 'AddAssignFriendByValue::operator=':
// operator= for Unpin types is not yet supported.

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=91
impl ::std::ops::AddAssign<Self> for AddAssignFriendByValue {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: Self) {
        unsafe {
            crate::detail::__rust_thunk___ZpLR22AddAssignFriendByValueS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=95
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignProhibitedConstMember {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignProhibitedConstMember"),
    crate::AddAssignProhibitedConstMember
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=95
impl Default for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=95
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedConstMember {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN30AddAssignProhibitedConstMemberC1EOS_(
                &mut tmp, __param_0,
            );
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=95
// Error while generating bindings for item 'AddAssignProhibitedConstMember::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=95
// Error while generating bindings for item 'AddAssignProhibitedConstMember::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=96
// Error while generating bindings for item 'AddAssignProhibitedConstMember::operator+=':
// Compound assignment with const left-hand side is not supported

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=99
#[derive(Clone, Copy)]
#[repr(C)]
pub struct AddAssignProhibitedFriendConstLhs {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(
    forward_declare::symbol!("AddAssignProhibitedFriendConstLhs"),
    crate::AddAssignProhibitedFriendConstLhs
);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=99
impl Default for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=99
impl<'b> From<::ctor::RvalueReference<'b, Self>> for AddAssignProhibitedFriendConstLhs {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1EOS_(
                &mut tmp, __param_0,
            );
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=99
// Error while generating bindings for item 'AddAssignProhibitedFriendConstLhs::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=99
// Error while generating bindings for item 'AddAssignProhibitedFriendConstLhs::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=100
// Error while generating bindings for item 'operator+=':
// Compound assignment with const left-hand side is not supported

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=103
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ManyOperators {
    __non_field_data: [::std::mem::MaybeUninit<u8>; 1],
}
forward_declare::unsafe_define!(forward_declare::symbol!("ManyOperators"), crate::ManyOperators);

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=103
impl Default for ManyOperators {
    #[inline(always)]
    fn default() -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsC1Ev(&mut tmp);
            tmp.assume_init()
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=103
impl<'b> From<::ctor::RvalueReference<'b, Self>> for ManyOperators {
    #[inline(always)]
    fn from(__param_0: ::ctor::RvalueReference<'b, Self>) -> Self {
        let mut tmp = ::std::mem::MaybeUninit::<Self>::zeroed();
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsC1EOS_(&mut tmp, __param_0);
            tmp.assume_init()
        }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=103
// Error while generating bindings for item 'ManyOperators::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=103
// Error while generating bindings for item 'ManyOperators::operator=':
// operator= for Unpin types is not yet supported.

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=104
// Error while generating bindings for item 'ManyOperators::operator+':
// Bindings for this kind of operator (operator + with 1 parameter(s)) are not supported

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=105
impl<'a> ::std::ops::Neg for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn neg(self) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsngEv(self) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=106
impl<'a> ::std::ops::Not for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn not(self) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsntEv(self) }
    }
}

// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=107
// Error while generating bindings for item 'ManyOperators::operator~':
// Bindings for this kind of operator (operator ~ with 1 parameter(s)) are not supported

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=109
impl<'a, 'b> ::std::ops::Add<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn add(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsplERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=110
impl<'a, 'b> ::std::ops::Sub<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn sub(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsmiERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=111
impl<'a, 'b> ::std::ops::Mul<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn mul(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsmlERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=112
impl<'a, 'b> ::std::ops::Div<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn div(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsdvERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=113
impl<'a, 'b> ::std::ops::Rem<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn rem(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsrmERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=114
impl<'a, 'b> ::std::ops::BitAnd<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitand(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsanERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=115
impl<'a, 'b> ::std::ops::BitOr<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitor(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsorERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=116
impl<'a, 'b> ::std::ops::BitXor<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn bitxor(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorseoERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=117
impl<'a, 'b> ::std::ops::Shl<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn shl(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorslsERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=118
impl<'a, 'b> ::std::ops::Shr<&'b crate::ManyOperators> for &'a crate::ManyOperators {
    type Output = crate::ManyOperators;
    #[inline(always)]
    fn shr(self, rhs: &'b crate::ManyOperators) -> Self::Output {
        unsafe { crate::detail::__rust_thunk___ZNK13ManyOperatorsrsERKS_(self, rhs) }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=120
impl<'b> ::std::ops::AddAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn add_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorspLERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=121
impl<'b> ::std::ops::SubAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn sub_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsmIERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=122
impl<'b> ::std::ops::MulAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn mul_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsmLERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=123
impl<'b> ::std::ops::DivAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn div_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsdVERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=124
impl<'b> ::std::ops::RemAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn rem_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsrMERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=125
impl<'b> ::std::ops::BitAndAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn bitand_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsaNERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=126
impl<'b> ::std::ops::BitOrAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn bitor_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsoRERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=127
impl<'b> ::std::ops::BitXorAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn bitxor_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorseOERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=128
impl<'b> ::std::ops::ShlAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn shl_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorslSERKS_(self, rhs);
        }
    }
}

/// Generated from: rs_bindings_from_cc/test/golden/operators.h;l=129
impl<'b> ::std::ops::ShrAssign<&'b Self> for ManyOperators {
    #[inline(always)]
    fn shr_assign<'a>(&'a mut self, rhs: &'b Self) {
        unsafe {
            crate::detail::__rust_thunk___ZN13ManyOperatorsrSERKS_(self, rhs);
        }
    }
}

// CRUBIT_RS_BINDINGS_FROM_CC_TEST_GOLDEN_OPERATORS_H_

mod detail {
    #[allow(unused_imports)]
    use super::*;
    extern "C" {
        pub(crate) fn __rust_thunk___ZN18AddableConstMemberC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMember>,
        );
        pub(crate) fn __rust_thunk___ZN18AddableConstMemberC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMember>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableConstMember>,
        );
        #[link_name = "_ZNK18AddableConstMemberplERKS_"]
        pub(crate) fn __rust_thunk___ZNK18AddableConstMemberplERKS_<'a, 'b>(
            __this: &'a crate::AddableConstMember,
            rhs: &'b crate::AddableConstMember,
        ) -> crate::AddableConstMember;
        pub(crate) fn __rust_thunk___ZN21AddableNonConstMemberC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableNonConstMember>,
        );
        pub(crate) fn __rust_thunk___ZN21AddableNonConstMemberC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableNonConstMember>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableNonConstMember>,
        );
        #[link_name = "_ZN21AddableNonConstMemberplERKS_"]
        pub(crate) fn __rust_thunk___ZN21AddableNonConstMemberplERKS_<'a, 'b>(
            __this: &'a mut crate::AddableNonConstMember,
            rhs: &'b crate::AddableNonConstMember,
        ) -> crate::AddableNonConstMember;
        pub(crate) fn __rust_thunk___ZN13AddableFriendC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFriend>,
        );
        pub(crate) fn __rust_thunk___ZN13AddableFriendC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFriend>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFriend>,
        );
        #[link_name = "_ZplRK13AddableFriendS1_"]
        pub(crate) fn __rust_thunk___ZplRK13AddableFriendS1_<'a, 'b>(
            lhs: &'a crate::AddableFriend,
            rhs: &'b crate::AddableFriend,
        ) -> crate::AddableFriend;
        pub(crate) fn __rust_thunk___ZN11AddableFreeC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFree>,
        );
        pub(crate) fn __rust_thunk___ZN11AddableFreeC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableFree>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableFree>,
        );
        #[link_name = "_ZplRK11AddableFreeS1_"]
        pub(crate) fn __rust_thunk___ZplRK11AddableFreeS1_<'a, 'b>(
            lhs: &'a crate::AddableFree,
            rhs: &'b crate::AddableFree,
        ) -> crate::AddableFree;
        #[link_name = "_ZplR11AddableFreeS0_"]
        pub(crate) fn __rust_thunk___ZplR11AddableFreeS0_<'a, 'b>(
            lhs: &'a mut crate::AddableFree,
            rhs: &'b mut crate::AddableFree,
        ) -> crate::AddableFree;
        #[link_name = "_Zpl11AddableFreeS_"]
        pub(crate) fn __rust_thunk___Zpl11AddableFreeS_(
            lhs: crate::AddableFree,
            rhs: crate::AddableFree,
        ) -> crate::AddableFree;
        pub(crate) fn __rust_thunk___ZN10OverloadedC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Overloaded>,
        );
        pub(crate) fn __rust_thunk___ZN10OverloadedC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::Overloaded>,
            __param_0: ::ctor::RvalueReference<'b, crate::Overloaded>,
        );
        #[link_name = "_ZplRK10Overloadeds"]
        pub(crate) fn __rust_thunk___ZplRK10Overloadeds<'a>(
            lhs: &'a crate::Overloaded,
            rhs: i16,
        ) -> i32;
        #[link_name = "_ZplRK10Overloadedi"]
        pub(crate) fn __rust_thunk___ZplRK10Overloadedi<'a>(
            lhs: &'a crate::Overloaded,
            rhs: i32,
        ) -> i32;
        pub(crate) fn __rust_thunk___ZN15IncompatibleLHSC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::IncompatibleLHS>,
        );
        pub(crate) fn __rust_thunk___ZN15IncompatibleLHSC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::IncompatibleLHS>,
            __param_0: ::ctor::RvalueReference<'b, crate::IncompatibleLHS>,
        );
        pub(crate) fn __rust_thunk___ZN18AddableReturnsVoidC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableReturnsVoid>,
        );
        pub(crate) fn __rust_thunk___ZN18AddableReturnsVoidC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableReturnsVoid>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddableReturnsVoid>,
        );
        #[link_name = "_ZNK18AddableReturnsVoidplERKS_"]
        pub(crate) fn __rust_thunk___ZNK18AddableReturnsVoidplERKS_<'a, 'b>(
            __this: &'a crate::AddableReturnsVoid,
            rhs: &'b crate::AddableReturnsVoid,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinC1ERKS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
            __param_0: &'b crate::AddableConstMemberNonunpin,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinaSERKS_<'a, 'b>(
            __this: ::std::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
            __param_0: &'b crate::AddableConstMemberNonunpin,
        ) -> ::std::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>;
        pub(crate) fn __rust_thunk___ZNK26AddableConstMemberNonunpinplERKS_<'a, 'b>(
            __return: &mut ::std::mem::MaybeUninit<crate::AddableConstMemberNonunpin>,
            __this: &'a crate::AddableConstMemberNonunpin,
            rhs: &'b crate::AddableConstMemberNonunpin,
        );
        pub(crate) fn __rust_thunk___ZN26AddableConstMemberNonunpinD1Ev<'a>(
            __this: ::std::pin::Pin<&'a mut crate::AddableConstMemberNonunpin>,
        );
        pub(crate) fn __rust_thunk___ZN18AddAssignMemberIntC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignMemberInt>,
        );
        pub(crate) fn __rust_thunk___ZN18AddAssignMemberIntC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignMemberInt>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignMemberInt>,
        );
        #[link_name = "_ZN18AddAssignMemberIntpLEi"]
        pub(crate) fn __rust_thunk___ZN18AddAssignMemberIntpLEi<'a>(
            __this: &'a mut crate::AddAssignMemberInt,
            rhs: i32,
        ) -> i32;
        pub(crate) fn __rust_thunk___ZN25AddAssignMemberByConstRefC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignMemberByConstRef>,
        );
        pub(crate) fn __rust_thunk___ZN25AddAssignMemberByConstRefC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignMemberByConstRef>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignMemberByConstRef>,
        );
        #[link_name = "_ZN25AddAssignMemberByConstRefpLERKS_"]
        pub(crate) fn __rust_thunk___ZN25AddAssignMemberByConstRefpLERKS_<'a, 'b>(
            __this: &'a mut crate::AddAssignMemberByConstRef,
            rhs: &'b crate::AddAssignMemberByConstRef,
        ) -> &'a mut crate::AddAssignMemberByConstRef;
        pub(crate) fn __rust_thunk___ZN23AddAssignFreeByConstRefC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFreeByConstRef>,
        );
        pub(crate) fn __rust_thunk___ZN23AddAssignFreeByConstRefC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFreeByConstRef>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFreeByConstRef>,
        );
        pub(crate) fn __rust_thunk___ZN20AddAssignFreeByValueC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFreeByValue>,
        );
        pub(crate) fn __rust_thunk___ZN20AddAssignFreeByValueC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFreeByValue>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFreeByValue>,
        );
        #[link_name = "_ZpLR20AddAssignFreeByValueS_"]
        pub(crate) fn __rust_thunk___ZpLR20AddAssignFreeByValueS_<'a>(
            lhs: &'a mut crate::AddAssignFreeByValue,
            rhs: crate::AddAssignFreeByValue,
        ) -> &'a mut crate::AddAssignFreeByValue;
        pub(crate) fn __rust_thunk___ZN25AddAssignFriendByConstRefC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFriendByConstRef>,
        );
        pub(crate) fn __rust_thunk___ZN25AddAssignFriendByConstRefC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFriendByConstRef>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFriendByConstRef>,
        );
        pub(crate) fn __rust_thunk___ZN22AddAssignFriendByValueC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFriendByValue>,
        );
        pub(crate) fn __rust_thunk___ZN22AddAssignFriendByValueC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignFriendByValue>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignFriendByValue>,
        );
        #[link_name = "_ZpLR22AddAssignFriendByValueS_"]
        pub(crate) fn __rust_thunk___ZpLR22AddAssignFriendByValueS_<'a>(
            lhs: &'a mut crate::AddAssignFriendByValue,
            rhs: crate::AddAssignFriendByValue,
        ) -> &'a mut crate::AddAssignFriendByValue;
        pub(crate) fn __rust_thunk___ZN30AddAssignProhibitedConstMemberC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignProhibitedConstMember>,
        );
        pub(crate) fn __rust_thunk___ZN30AddAssignProhibitedConstMemberC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignProhibitedConstMember>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignProhibitedConstMember>,
        );
        pub(crate) fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignProhibitedFriendConstLhs>,
        );
        pub(crate) fn __rust_thunk___ZN33AddAssignProhibitedFriendConstLhsC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::AddAssignProhibitedFriendConstLhs>,
            __param_0: ::ctor::RvalueReference<'b, crate::AddAssignProhibitedFriendConstLhs>,
        );
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsC1Ev<'a>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ManyOperators>,
        );
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsC1EOS_<'a, 'b>(
            __this: &'a mut ::std::mem::MaybeUninit<crate::ManyOperators>,
            __param_0: ::ctor::RvalueReference<'b, crate::ManyOperators>,
        );
        #[link_name = "_ZNK13ManyOperatorsngEv"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsngEv<'a>(
            __this: &'a crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsntEv"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsntEv<'a>(
            __this: &'a crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsplERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsplERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsmiERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsmiERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsmlERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsmlERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsdvERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsdvERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsrmERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsrmERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsanERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsanERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsorERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsorERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorseoERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorseoERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorslsERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorslsERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZNK13ManyOperatorsrsERKS_"]
        pub(crate) fn __rust_thunk___ZNK13ManyOperatorsrsERKS_<'a, 'b>(
            __this: &'a crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorspLERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorspLERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsmIERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsmIERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsmLERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsmLERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsdVERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsdVERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsrMERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsrMERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsaNERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsaNERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsoRERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsoRERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorseOERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorseOERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorslSERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorslSERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
        #[link_name = "_ZN13ManyOperatorsrSERKS_"]
        pub(crate) fn __rust_thunk___ZN13ManyOperatorsrSERKS_<'a, 'b>(
            __this: &'a mut crate::ManyOperators,
            rhs: &'b crate::ManyOperators,
        ) -> &'a mut crate::ManyOperators;
    }
}

const _: () = assert!(::std::mem::size_of::<Option<&i32>>() == ::std::mem::size_of::<&i32>());

const _: () = assert!(::std::mem::size_of::<crate::AddableConstMember>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableConstMember>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableConstMember: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableConstMember: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableConstMember: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::AddableConstMember, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableNonConstMember>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableNonConstMember>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableNonConstMember: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableNonConstMember: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableNonConstMember: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::AddableNonConstMember, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableFriend>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableFriend>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFriend: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFriend: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableFriend: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::AddableFriend, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableFree>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddableFree>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFree: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableFree: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableFree: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::Overloaded>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::Overloaded>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::Overloaded: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::Overloaded: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::Overloaded: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::IncompatibleLHS>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::IncompatibleLHS>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::IncompatibleLHS: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::IncompatibleLHS: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::IncompatibleLHS: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddableReturnsVoid>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableReturnsVoid>() == 4);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableReturnsVoid: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableReturnsVoid: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableReturnsVoid: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::AddableReturnsVoid, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddableConstMemberNonunpin>() == 4);
const _: () = assert!(::std::mem::align_of::<crate::AddableConstMemberNonunpin>() == 4);
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddableConstMemberNonunpin: Copy);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddableConstMemberNonunpin: Drop);
};
const _: () = assert!(memoffset::offset_of!(crate::AddableConstMemberNonunpin, field_) == 0);

const _: () = assert!(::std::mem::size_of::<crate::AddAssignMemberInt>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignMemberInt>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignMemberInt: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignMemberInt: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignMemberInt: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignMemberByConstRef>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignMemberByConstRef>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignMemberByConstRef: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignMemberByConstRef: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignMemberByConstRef: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignFreeByConstRef>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignFreeByConstRef>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFreeByConstRef: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFreeByConstRef: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignFreeByConstRef: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignFreeByValue>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignFreeByValue>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFreeByValue: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFreeByValue: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignFreeByValue: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignFriendByConstRef>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignFriendByConstRef>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFriendByConstRef: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFriendByConstRef: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignFriendByConstRef: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignFriendByValue>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignFriendByValue>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFriendByValue: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignFriendByValue: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignFriendByValue: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignProhibitedConstMember>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignProhibitedConstMember>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedConstMember: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedConstMember: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignProhibitedConstMember: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::AddAssignProhibitedFriendConstLhs>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::AddAssignProhibitedFriendConstLhs>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedFriendConstLhs: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::AddAssignProhibitedFriendConstLhs: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::AddAssignProhibitedFriendConstLhs: Drop);
};

const _: () = assert!(::std::mem::size_of::<crate::ManyOperators>() == 1);
const _: () = assert!(::std::mem::align_of::<crate::ManyOperators>() == 1);
const _: () = {
    static_assertions::assert_impl_all!(crate::ManyOperators: Clone);
};
const _: () = {
    static_assertions::assert_impl_all!(crate::ManyOperators: Copy);
};
const _: () = {
    static_assertions::assert_not_impl_any!(crate::ManyOperators: Drop);
};
