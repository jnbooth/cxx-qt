// clang-format off
// SPDX-FileCopyrightText: 2022 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// clang-format on
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#include "cxx-qt-lib/quuid.h"

#include <cxx-qt-lib/assertion_utils.h>

assert_alignment_and_size(QUuid,
                          alignof(::std::uint32_t),
                          // data1 (uint)
                          sizeof(::std::uint32_t) +
                              // data2 (ushort)
                              sizeof(::std::uint16_t) +
                              // data3 (ushort)
                              sizeof(::std::uint16_t) +
                              // data4 (uchar[8])
                              sizeof(::std::uint8_t[8]));

static_assert(::std::is_trivially_copyable<QUuid>::value,
              "QUuid must be trivially copyable!");

#if QT_VERSION >= QT_VERSION_CHECK(6, 8, 0)
#define byteView(slice) QByteArrayView(slice.data(), slice.length())
#elif QT_VERSION >= QT_VERSION_CHECK(6, 0, 0)
#define byteView(slice)                                                        \
  QByteArray::fromRawData(reinterpret_cast<const char*>(slice.data()),         \
                          static_cast<qsizetype>(slice.size()))
#else
#define byteView(slice)                                                        \
  QByteArray::fromRawData(reinterpret_cast<const char*>(slice.data()),         \
                          static_cast<int>(slice.size()))
#endif
// reinterpret_cast<const char*>(slice.data()

namespace rust {
namespace cxxqtlib1 {
QUuid
quuidNewV3(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice)
{
  return QUuid::createUuidV3(ns, byteView(slice));
}

QUuid
quuidNewV4()
{
  return QUuid::createUuid();
}

QUuid
quuidNewV5(const QUuid& ns, ::rust::Slice<const ::std::uint8_t> slice)
{
  return QUuid::createUuidV5(ns, byteView(slice));
}

QUuid
quuidFromString(const QString& string)
{
  return QUuid::fromString(string);
}

#if QT_VERSION >= QT_VERSION_CHECK(6, 3, 0)
QUuid
quuidFromStr(::rust::Str string)
{
  return QUuid::fromString(QLatin1StringView(string.data(), string.length()));
}
#endif
}
}
