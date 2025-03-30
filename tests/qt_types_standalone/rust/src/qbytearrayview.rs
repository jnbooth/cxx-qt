// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Seems to be a Clippy false positive, we need these lifetime declarations
#![allow(clippy::needless_lifetimes)]

use cxx_qt_lib::{QByteArray, QByteArrayView};

#[cxx::bridge]
mod qbytearrayview_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearrayview.h");
        type QByteArrayView<'a> = cxx_qt_lib::QByteArrayView<'a>;

        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
    }

    extern "Rust" {
        fn construct_qbytearrayview(slice: &[u8]) -> QByteArrayView;
    }

    // This method must be unsafe otherwise we hit
    // must be `unsafe fn` in order to expose explicit lifetimes to C++
    //
    // But then Rust complains about unused unsafe so we need to allow for this
    #[allow(unused_unsafe)]
    extern "Rust" {
        unsafe fn construct_qbytearrayview_qbytearray<'a>(
            str: &'a QByteArray,
        ) -> QByteArrayView<'a>;
        unsafe fn clone_qbytearrayview<'a>(l: &QByteArrayView<'a>) -> QByteArrayView<'a>;
        unsafe fn slice_qbytearrayview<'a>(view: &QByteArrayView<'a>) -> &'a [u8];
    }
}

fn construct_qbytearrayview(slice: &[u8]) -> QByteArrayView {
    QByteArrayView::from(slice)
}

fn slice_qbytearrayview<'a>(view: &QByteArrayView<'a>) -> &'a [u8] {
    view.as_slice()
}

fn construct_qbytearrayview_qbytearray(str: &QByteArray) -> QByteArrayView {
    QByteArrayView::from(str)
}

#[allow(clippy::clone_on_copy)]
fn clone_qbytearrayview<'a>(l: &QByteArrayView<'a>) -> QByteArrayView<'a> {
    l.clone()
}
