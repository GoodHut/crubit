#![feature(negative_impls, const_ptr_offset_from, const_maybe_uninit_as_ptr, const_raw_ptr_deref)]
// Part of the Crubit project, under the Apache License v2.0 with LLVM
// Exceptions. See /LICENSE for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception

use memoffset_unstable_const::offset_of;
use static_assertions::const_assert_eq;
/// Doc comment
///
///  * with three slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentSlashes {
    pub i: i32,
}
const_assert_eq!(std::mem::size_of::<DocCommentSlashes>(), 4usize);
const_assert_eq!(std::mem::align_of::<DocCommentSlashes>(), 4usize);
const_assert_eq!(offset_of!(DocCommentSlashes, i) * 8, 0usize);
/// Doc comment
///
///  * with slashes and bang
#[derive(Clone, Copy)]
#[repr(C)]
pub struct DocCommentBang {
    pub i: i32,
}
const_assert_eq!(std::mem::size_of::<DocCommentBang>(), 4usize);
const_assert_eq!(std::mem::align_of::<DocCommentBang>(), 4usize);
const_assert_eq!(offset_of!(DocCommentBang, i) * 8, 0usize);
/// Multiline comment
///
///  with two stars
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultilineCommentTwoStars {
    pub i: i32,
}
const_assert_eq!(std::mem::size_of::<MultilineCommentTwoStars>(), 4usize);
const_assert_eq!(std::mem::align_of::<MultilineCommentTwoStars>(), 4usize);
const_assert_eq!(offset_of!(MultilineCommentTwoStars, i) * 8, 0usize);
/// Line comment
///
///  * with two slashes
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LineComment {
    pub i: i32,
}
const_assert_eq!(std::mem::size_of::<LineComment>(), 4usize);
const_assert_eq!(std::mem::align_of::<LineComment>(), 4usize);
const_assert_eq!(offset_of!(LineComment, i) * 8, 0usize);
/// Multiline comment
///
///  with one star
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MultilineOneStar {
    pub i: i32,
}
const_assert_eq!(std::mem::size_of::<MultilineOneStar>(), 4usize);
const_assert_eq!(std::mem::align_of::<MultilineOneStar>(), 4usize);
const_assert_eq!(offset_of!(MultilineOneStar, i) * 8, 0usize);
