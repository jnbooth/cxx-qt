// SPDX-FileCopyrightText: 2025 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Joshua Booth <joshua.n.booth@gmail.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cxx::bridge(namespace = "Qt")]
pub mod ffi {
    #[derive(Debug)]
    #[repr(u32)]
    enum KeyboardModifier {
        /// No modifier key is pressed.
        NoModifier = 0x00000000,
        /// A Shift key on the keyboard is pressed.
        ShiftModifier = 0x02000000,
        /// A Ctrl key on the keyboard is pressed.
        ControlModifier = 0x04000000,
        /// An Alt key on the keyboard is pressed.
        AltModifier = 0x08000000,
        /// A Meta key on the keyboard is pressed.
        MetaModifier = 0x10000000,
        /// A keypad button is pressed.
        KeypadModifier = 0x20000000,
        /// X11 only (unless activated on Windows by a command line argument).
        /// A Mode_switch key on the keyboard is pressed.
        GroupSwitchModifier = 0x40000000,
    }

    #[derive(Debug)]
    #[repr(u32)]
    enum MouseButton {
        /// The button state does not refer to any button.
        NoButton = 0x00000000,
        /// This value corresponds to a mask of all possible mouse buttons. Use to set the
        /// 'acceptedButtons' property of a MouseArea to accept ALL mouse buttons.
        AllButtons = 0x07ffffff,
        /// The left button is pressed, or an event refers to the left button. (The left button may
        /// be the right button on left-handed mice.)
        LeftButton = 0x00000001,
        /// The right button.
        RightButton = 0x00000002,
        /// The middle button.
        MiddleButton = 0x00000004,
        /// The 'Back' button. (Typically present on the 'thumb' side of a mouse with extra buttons.
        /// This is NOT the tilt wheel.)
        BackButton = 0x00000008,
        /// The 'Forward' button. (Typically present beside the 'Back' button, and also pressed by
        /// the thumb.)
        ForwardButton = 0x00000010,
        /// The 'Task' button.
        TaskButton = 0x00000020,
        ExtraButton4 = 0x00000040,
        ExtraButton5 = 0x00000080,
        ExtraButton6 = 0x00000100,
        ExtraButton7 = 0x00000200,
        ExtraButton8 = 0x00000400,
        ExtraButton9 = 0x00000800,
        ExtraButton10 = 0x00001000,
        ExtraButton11 = 0x00002000,
        ExtraButton12 = 0x00004000,
        ExtraButton13 = 0x00008000,
        ExtraButton14 = 0x00010000,
        ExtraButton15 = 0x00020000,
        ExtraButton16 = 0x00040000,
        ExtraButton17 = 0x00080000,
        ExtraButton18 = 0x00100000,
        ExtraButton19 = 0x00200000,
        ExtraButton20 = 0x00400000,
        ExtraButton21 = 0x00800000,
        ExtraButton22 = 0x01000000,
        ExtraButton23 = 0x02000000,
        ExtraButton24 = 0x04000000,
    }

    extern "C++" {
        include!(<QtCore/QtCore>);
        type MouseButton;
        type KeyboardModifier;
    }
}

pub use ffi::{KeyboardModifier, MouseButton};
