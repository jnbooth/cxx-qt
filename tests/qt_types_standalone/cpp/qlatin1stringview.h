// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAnyStringView>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qlatin1stringview.cxx.h"

class QLatin1StringViewTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto s = construct_qlatin1stringview("String constructed by Rust");
    QCOMPARE(s, QByteArrayLiteral("String constructed by Rust"));
  }

  void construct_qbytearray()
  {
    const auto s = construct_qlatin1stringview_qbytearray(
      QByteArrayLiteral("String constructed by Rust"));
    QCOMPARE(s, QByteArrayLiteral("String constructed by Rust"));
  }

  void clone()
  {
    const auto l = QLatin1StringView("Test");
    const auto c = clone_qlatin1stringview(l);
    QCOMPARE(c, l);
  }

  void slice_qlatin1stringview()
  {
    const auto s =
      slice_qlatin1stringview(QByteArrayLiteral("Slice constructed by Rust"));
    QCOMPARE(s, QByteArrayLiteral("Slice constructed by Rust"));
  }
};
