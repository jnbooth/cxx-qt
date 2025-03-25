// SPDX-FileCopyrightText: 2023 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
// SPDX-FileContributor: Andrew Hayzen <andrew.hayzen@kdab.com>
//
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::unsafe_impl_qflag;

#[cxx::bridge(namespace = "Qt")]
mod ffi {
    /// This enum type defines what happens to the aspect ratio when scaling an rectangle.
    #[repr(i32)]
    enum AspectRatioMode {
        /// The size is scaled freely. The aspect ratio is not preserved.
        IgnoreAspectRatio,
        /// The size is scaled to a rectangle as large as possible inside a given rectangle, preserving the aspect ratio.
        KeepAspectRatio,
        /// The size is scaled to a rectangle as small as possible outside a given rectangle, preserving the aspect ratio.
        KeepAspectRatioByExpanding,
    }

    #[repr(i32)]
    enum CaseSensitivity {
        CaseInsensitive,
        CaseSensitive,
    }

    #[repr(i32)]
    enum DateFormat {
        TextDate = 0,
        ISODateWithMs = 9,
        ISODate = 1,
        RFC2822Date = 8,
    }

    #[repr(i32)]
    enum SplitBehaviorFlags {
        KeepEmptyParts,
        SkipEmptyParts,
    }

    #[repr(i32)]
    enum TimeSpec {
        /// Local time, controlled by a system time-zone setting.
        LocalTime,
        /// Coordinated Universal Time.
        UTC,
        /// An offset in seconds from Coordinated Universal Time.
        OffsetFromUTC,
        /// A named time zone.
        TimeZone,
    }

    /// This enum type defines whether image transformations (e.g., scaling) should be smooth or not.
    #[repr(i32)]
    enum TransformationMode {
        /// The transformation is performed quickly, with no smoothing.
        FastTransformation,
        /// The resulting image is transformed using bilinear filtering.
        SmoothTransformation,
    }

    /// This enum type defines the pen styles that can be drawn using QPainter.
    #[repr(i32)]
    enum PenStyle {
        /// no line at all. For example, QPainter::drawRect() fills but does not draw any boundary line.
        NoPen,
        /// A plain line.
        SolidLine,
        /// Dashes separated by a few pixels.
        DashLine,
        /// Dots separated by a few pixels.
        DotLine,
        /// Alternate dots and dashes.
        DashDotLine,
        /// One dash, two dots, one dash, two dots.
        DashDotDotLine,
        /// A custom pattern defined using QPainterPathStroker::setDashPattern().
        CustomDashLine,
    }

    /// This enum type defines the line endcap style
    #[repr(i32)]
    enum PenCapStyle {
        FlatCap = 0x00,
        SquareCap = 0x10,
        RoundCap = 0x20,
        MPenCapStyle = 0x30,
    }

    /// This enum type defines the line join style.
    #[repr(i32)]
    enum PenJoinStyle {
        MiterJoin = 0x00,
        BevelJoin = 0x40,
        RoundJoin = 0x80,
        SvgMiterJoin = 0x100,
        MPenJoinStyle = 0x1c0,
    }

    #[repr(i32)]
    enum FillRule {
        /// Specifies that the region is filled using the odd even fill rule.
        /// With this rule, we determine whether a point is inside the shape by using
        /// the following method. Draw a horizontal line from the point to a location
        /// outside the shape, and count the number of intersections. If the number of
        /// intersections is an odd number, the point is inside the shape. This mode is the default.
        OddEvenFill,
        /// Specifies that the region is filled using the non zero winding rule.
        /// With this rule, we determine whether a point is inside the shape by using the following method.
        /// Draw a horizontal line from the point to a location outside the shape. Determine whether
        /// the direction of the line at each intersection point is up or down. The winding number is determined
        /// by summing the direction of each intersection. If the number is non zero, the point is inside the shape.
        /// This fill mode can also in most cases be considered as the intersection of closed shapes.
        WindingFill,
    }

    /// This enum type specifies the direction of Qt's layouts and text handling.
    #[repr(i32)]
    enum LayoutDirection {
        LeftToRight,
        RightToLeft,
        LayoutDirectionAuto,
    }

    /// This enum type specifies the background mode
    #[repr(i32)]
    enum BGMode {
        TransparentMode,
        OpaqueMode,
    }

    #[repr(i32)]
    enum ClipOperation {
        NoClip,
        ReplaceClip,
        IntersectClip,
    }

    /// This enum is used by QPainter::drawRoundedRect() and QPainterPath::addRoundedRect()
    /// functions to specify the radii of rectangle corners with respect to the dimensions
    /// of the bounding rectangles specified.
    #[repr(i32)]
    enum SizeMode {
        /// Specifies the size using absolute measurements.
        AbsoluteSize,
        /// Specifies the size relative to the bounding rectangle, typically using percentage measurements.
        RelativeSize,
    }

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

    unsafe extern "C++" {
        include!("cxx-qt-lib/qt.h");
        type AspectRatioMode;
        type CaseSensitivity;
        type DateFormat;
        type SplitBehaviorFlags;
        type TimeSpec;
        type TransformationMode;
        type PenStyle;
        type PenCapStyle;
        type PenJoinStyle;
        type FillRule;
        type LayoutDirection;
        type BGMode;
        type ClipOperation;
        type SizeMode;
        type MouseButton;
        type KeyboardModifier;
    }
}

pub use ffi::{
    AspectRatioMode, BGMode, CaseSensitivity, ClipOperation, DateFormat, FillRule,
    KeyboardModifier, LayoutDirection, MouseButton, PenCapStyle, PenJoinStyle, PenStyle, SizeMode,
    SplitBehaviorFlags, TimeSpec, TransformationMode,
};

// Reexport ConnectionType from cxx-qt
pub use cxx_qt::ConnectionType;

pub type MouseButtons = crate::QFlags<MouseButton>;
pub type KeyboardModifiers = crate::QFlags<KeyboardModifier>;

unsafe_impl_qflag!(MouseButton, "Qt::MouseButtons", u32);
unsafe_impl_qflag!(KeyboardModifier, "Qt::KeyboardModifiers", u32);
