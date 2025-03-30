// clang-format off
// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/qlatin1stringview.h"

#include <cxx-qt-lib/assertion_utils.h>

// QLatin1StringView has two members: char* and a size_t.
// https://code.qt.io/cgit/qt/qtbase.git/tree/src/corelib/text/qlatin1stringview.h
assert_alignment_and_size(QLatin1StringView, {
  ::std::size_t a0;
  const char* a1;
});

static_assert(::std::is_trivially_copy_assignable<QLatin1StringView>::value);
static_assert(::std::is_trivially_copy_constructible<QLatin1StringView>::value);

static_assert(::std::is_trivially_destructible<QLatin1StringView>::value);

static_assert(QTypeInfo<QLatin1StringView>::isRelocatable);
