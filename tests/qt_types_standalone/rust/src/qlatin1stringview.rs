// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

// Seems to be a Clippy false positive, we need these lifetime declarations
#![allow(clippy::needless_lifetimes)]

use cxx_qt_lib::{QByteArray, QLatin1StringView};

#[cxx::bridge]
mod qlatin1stringview_cxx {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qlatin1stringview.h");
        type QLatin1StringView<'a> = cxx_qt_lib::QLatin1StringView<'a>;

        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = cxx_qt_lib::QByteArray;
    }

    extern "Rust" {
        fn construct_qlatin1stringview(slice: &[u8]) -> QLatin1StringView;
    }

    // This method must be unsafe otherwise we hit
    // must be `unsafe fn` in order to expose explicit lifetimes to C++
    //
    // But then Rust complains about unused unsafe so we need to allow for this
    #[allow(unused_unsafe)]
    extern "Rust" {
        unsafe fn construct_qlatin1stringview_qbytearray<'a>(
            str: &'a QByteArray,
        ) -> QLatin1StringView<'a>;
        unsafe fn clone_qlatin1stringview<'a>(l: &QLatin1StringView<'a>) -> QLatin1StringView<'a>;
        unsafe fn slice_qlatin1stringview<'a>(view: &QLatin1StringView<'a>) -> &'a [u8];
    }
}

fn construct_qlatin1stringview(slice: &[u8]) -> QLatin1StringView {
    QLatin1StringView::from(slice)
}

fn slice_qlatin1stringview<'a>(view: &QLatin1StringView<'a>) -> &'a [u8] {
    view.as_slice()
}

fn construct_qlatin1stringview_qbytearray(str: &QByteArray) -> QLatin1StringView {
    QLatin1StringView::from(str)
}

#[allow(clippy::clone_on_copy)]
fn clone_qlatin1stringview<'a>(l: &QLatin1StringView<'a>) -> QLatin1StringView<'a> {
    l.clone()
}
