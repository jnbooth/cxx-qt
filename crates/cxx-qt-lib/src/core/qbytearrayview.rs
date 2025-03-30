// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <josh@redstrate.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::QByteArray;
use core::ffi::c_char;
use core::marker::PhantomData;
use cxx::{type_id, ExternType};
use std::cmp::Ordering;
use std::{fmt, ptr, slice};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearrayview.h");
        type QByteArrayView<'a> = super::QByteArrayView<'a>;

        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;

        /// Returns a deep copy of this byte array view's data as a `QByteArray`.
        ///
        /// The return value will be a null `QByteArray` if and only if this byte array view is null.
        #[rust_name = "to_qbytearray"]
        fn toByteArray(self: &QByteArrayView) -> QByteArray;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "QByteArrayView_init_from_qbytearray"]
        fn construct<'a>(bytes: &'a QByteArray) -> QByteArrayView<'a>;

        #[doc(hidden)]
        #[rust_name = "QByteArrayView_cmp"]
        fn operatorCmp(a: &QByteArrayView, b: &QByteArrayView) -> i8;

        #[doc(hidden)]
        #[rust_name = "QByteArrayView_eq"]
        fn operatorEq(a: &QByteArrayView, b: &QByteArrayView) -> bool;
    }
}

/// The `QByteArrayView` class provides a view on an array of bytes with a read-only subset of the `QByteArray` API.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QByteArrayView<'a> {
    size: isize,
    data: *const c_char,

    /// Needed to keep the lifetime in check
    _phantom: PhantomData<&'a usize>,
}

impl AsRef<[u8]> for QByteArrayView<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Default for QByteArrayView<'_> {
    /// Constructs a null byte array view.
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for QByteArrayView<'_> {
    fn eq(&self, other: &Self) -> bool {
        ffi::QByteArrayView_eq(self, other)
    }
}

impl Eq for QByteArrayView<'_> {}

impl PartialOrd for QByteArrayView<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QByteArrayView<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        ffi::QByteArrayView_cmp(self, other).cmp(&0)
    }
}

impl<'a> From<&'a QByteArray> for QByteArrayView<'a> {
    fn from(bytes: &'a QByteArray) -> Self {
        ffi::QByteArrayView_init_from_qbytearray(bytes)
    }
}

impl<'a> From<&'a [u8]> for QByteArrayView<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self::from_slice(slice)
    }
}

impl<'a> From<&'a str> for QByteArrayView<'a> {
    fn from(string: &'a str) -> Self {
        Self::from_slice(string.as_bytes())
    }
}

impl<'a> From<QByteArrayView<'a>> for &'a [u8] {
    fn from(view: QByteArrayView<'a>) -> Self {
        view.as_slice()
    }
}

impl<'a> From<QByteArrayView<'a>> for QByteArray {
    /// Returns a deep copy of this byte array view's data as a `QByteArray`.
    ///
    /// The return value will be a null `QByteArray` if and only if this byte array view is null.
    fn from(bytes: QByteArrayView<'a>) -> Self {
        bytes.to_qbytearray()
    }
}

impl<'a> QByteArrayView<'a> {
    /// Constructs a null view.
    pub const fn new() -> Self {
        Self {
            size: 0,
            data: ptr::null(),
            _phantom: PhantomData,
        }
    }

    /// Returns a pointer to the first byte in the byte array view.
    ///
    /// The returned pointer is only safe to use for accessing bytes at indices that are less than
    /// `self.len()`.
    pub const fn data(&self) -> *const u8 {
        self.data.cast()
    }

    /// Borrows the view's data as a slice of u8.
    #[inline]
    pub const fn as_slice(&self) -> &'a [u8] {
        if self.size == 0 {
            &[]
        } else {
            // SAFETY: `self.data` is non-null and valid for reads for `self.size` many bytes.
            unsafe { slice::from_raw_parts(self.data.cast(), self.size as usize) }
        }
    }

    /// Constructs a `QByteArrayView` from a slice of `u8`.
    pub const fn from_slice(slice: &'a [u8]) -> Self {
        if slice.is_empty() {
            Self::new()
        } else {
            Self {
                size: slice.len() as isize,
                data: slice.as_ptr().cast(),
                _phantom: PhantomData,
            }
        }
    }

    /// Returns `true` if this byte array view is empty - that is, `self.len() == 0`.
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns true if this byte array view is null - that is, `self.data() == std::ptr::null()`.
    pub fn is_null(&self) -> bool {
        self.data.is_null()
    }

    /// Returns the number of bytes in this byte array view.
    pub const fn len(&self) -> isize {
        self.size
    }
}

impl fmt::Debug for QByteArrayView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QByteArrayView<'_> {
    type Id = type_id!("QByteArrayView");
    type Kind = cxx::kind::Trivial;
}
