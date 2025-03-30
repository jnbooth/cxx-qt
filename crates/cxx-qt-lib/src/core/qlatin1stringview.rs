// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Goins <josh@redstrate.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
use crate::{QByteArray, QByteArrayView, QString};
use core::ffi::c_char;
use core::marker::PhantomData;
use cxx::{type_id, ExternType};
use std::cmp::Ordering;
use std::{fmt, ptr, slice};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlatin1stringview.h");
        type QLatin1StringView<'a> = super::QLatin1StringView<'a>;

        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;

        /// Converts this Latin-1 string into a `QString`.
        #[rust_name = "to_qstring"]
        fn toString(self: &QLatin1StringView) -> QString;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "QLatin1StringView_init_from_qbytearray"]
        fn construct<'a>(bytes: &'a QByteArray) -> QLatin1StringView<'a>;

        #[doc(hidden)]
        #[rust_name = "QLatin1StringView_cmp"]
        fn operatorCmp(a: &QLatin1StringView, b: &QLatin1StringView) -> i8;

        #[doc(hidden)]
        #[rust_name = "QLatin1StringView_eq"]
        fn operatorEq(a: &QLatin1StringView, b: &QLatin1StringView) -> bool;
    }
}

/// The `QLatin1StringView` class provides a thin wrapper around a US-ASCII/Latin-1 encoded string literal.
///
/// **Note:** Qt provides no guarantees that the data contained in a `QLatin1StringView` is valid US-ASCII/Latin-1.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct QLatin1StringView<'a> {
    size: isize,
    data: *const c_char,

    /// Needed to keep the lifetime in check
    _phantom: PhantomData<&'a usize>,
}

impl AsRef<[u8]> for QLatin1StringView<'_> {
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl Default for QLatin1StringView<'_> {
    /// Constructs a `QLatin1StringView` object that stores a null pointer.
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for QLatin1StringView<'_> {
    fn eq(&self, other: &Self) -> bool {
        ffi::QLatin1StringView_eq(self, other)
    }
}

impl Eq for QLatin1StringView<'_> {}

impl PartialOrd for QLatin1StringView<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for QLatin1StringView<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        ffi::QLatin1StringView_cmp(self, other).cmp(&0)
    }
}

impl<'a> From<&'a QByteArray> for QLatin1StringView<'a> {
    fn from(bytes: &'a QByteArray) -> Self {
        ffi::QLatin1StringView_init_from_qbytearray(bytes)
    }
}

impl<'a> From<&'a [u8]> for QLatin1StringView<'a> {
    fn from(slice: &'a [u8]) -> Self {
        Self::from_slice(slice)
    }
}

impl<'a> From<&'a str> for QLatin1StringView<'a> {
    fn from(string: &'a str) -> Self {
        Self::from_slice(string.as_bytes())
    }
}

impl<'a> From<&QByteArrayView<'a>> for QLatin1StringView<'a> {
    fn from(value: &QByteArrayView<'a>) -> Self {
        Self::from_slice(value.as_slice())
    }
}

impl<'a> From<QLatin1StringView<'a>> for &'a [u8] {
    fn from(string: QLatin1StringView<'a>) -> Self {
        string.as_slice()
    }
}

impl<'a> From<QLatin1StringView<'a>> for QString {
    /// Converts this Latin-1 string into a `QString`.
    fn from(string: QLatin1StringView<'a>) -> Self {
        string.to_qstring()
    }
}

impl<'a> QLatin1StringView<'a> {
    /// Constructs a `QLatin1StringView` object that stores a null pointer.
    pub const fn new() -> Self {
        Self {
            size: 0,
            data: ptr::null(),
            _phantom: PhantomData,
        }
    }

    /// Constructs a slice of u8 from a QLatin1StringView.
    #[inline]
    pub const fn as_slice(&self) -> &'a [u8] {
        if self.size == 0 {
            &[]
        } else {
            // SAFETY: `self.data` is non-null and valid for reads for `self.size` many bytes.
            unsafe { slice::from_raw_parts(self.data.cast(), self.size as usize) }
        }
    }

    /// Returns the start of the Latin-1 string referenced by this object.
    ///
    /// The returned pointer is only safe to use for accessing bytes at indices that are less than
    /// this byte array view's `len()`.
    pub const fn data(&self) -> *const u8 {
        self.data.cast()
    }

    /// Constructs a `QLatin1StringView` from a slice of `u8`.
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

    /// Returns whether the Latin-1 string referenced by this object is empty `(self.len() == 0)` or not.
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns whether the Latin-1 string referenced by this object is null `(self.data() == std::ptr::null())` or not.
    pub fn is_null(&self) -> bool {
        self.data.is_null()
    }

    /// Returns the size of the Latin-1 string referenced by this object.
    pub const fn len(&self) -> isize {
        self.size
    }
}

impl fmt::Debug for QLatin1StringView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.as_slice())
    }
}

// Safety:
//
// Static checks on the C++ side to ensure the size is the same.
unsafe impl ExternType for QLatin1StringView<'_> {
    type Id = type_id!("QLatin1StringView");
    type Kind = cxx::kind::Trivial;
}
