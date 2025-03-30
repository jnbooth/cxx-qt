// clang-format off
// SPDX-FileCopyrightText: 2024 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Joshua Goins <joshua.goins@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once

#include <QtCore/QAnyStringView>
#include <QtTest/QTest>

#include "qt_types_standalone/src/qbytearrayview.cxx.h"

class QByteArrayViewTest : public QObject
{
  Q_OBJECT

private Q_SLOTS:
  void construct()
  {
    const auto s = construct_qbytearrayview("View constructed by Rust");
    QCOMPARE(s, QByteArrayLiteral("View constructed by Rust"));
  }

  void construct_qbytearray()
  {
    const auto s = construct_qbytearrayview_qbytearray(
      QByteArrayLiteral("View constructed by Rust"));
    QCOMPARE(s, QByteArrayLiteral("View constructed by Rust"));
  }

  void clone()
  {
    const auto l = QByteArrayView("Test");
    const auto c = clone_qbytearrayview(l);
    QCOMPARE(c, l);
  }

  void slice_qbytearray()
  {
    const auto s =
      slice_qbytearrayview(QByteArrayLiteral("Slice constructed by Rust"));
    QCOMPARE(s, QByteArrayLiteral("Slice constructed by Rust"));
  }
};
