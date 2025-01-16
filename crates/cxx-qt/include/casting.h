// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group
// company <info@kdab.com> SPDX-FileContributor: Ben Ford <ben.ford@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0
#pragma once
#include <type_traits>

namespace rust::cxxqt1 {

template<typename Sub, typename Base>
const Base*
upcastPtr(const Sub* sub)
{
  static_assert(std::is_base_of_v<Base, Sub>);
  return static_cast<const Base*>(sub);
}

}
