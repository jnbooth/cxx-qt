use crate::QString;
use cxx::{type_id, ExternType};
use std::fmt;
#[cfg(feature = "uuid")]
use uuid::Uuid;

#[cxx::bridge]
mod ffi {
    #[repr(i32)]
    enum QUuidVariant {
        /// Variant is unknown
        VarUnknown = -1,
        /// Reserved for NCS (Network Computing System) backward compatibility
        NCS = 0,
        /// Distributed Computing Environment, the scheme used by QUuid
        DCE = 2,
        /// Reserved for Microsoft backward compatibility (GUID)
        Microsoft = 6,
        /// Reserved for future definition
        Reserved = 7,
    }

    #[repr(i32)]
    enum QUuidVersion {
        /// Version is unknown
        VerUnknown = -1,
        /// Time-based, by using timestamp, clock sequence, and MAC network card address (if
        /// available) for the node sections
        Time = 1,
        /// DCE Security version, with embedded POSIX UUIDs
        EmbeddedPOSIX = 2,
        /// Name-based, by using values from a name for all sections
        Md5 = 3,
        /// Random-based, by using random numbers for all sections
        Random = 4,
        Sha1 = 5,
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qbytearray.h");
        type QByteArray = crate::QByteArray;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = crate::QString;
    }

    unsafe extern "C++" {
        include!("cxx-qt-lib/quuid.h");
        type QUuid = super::QUuid;
        type QUuidVariant;
        type QUuidVersion;

        /// Returns true if this is the null UUID `{00000000-0000-0000-0000-000000000000}``;
        /// otherwise returns false.
        #[rust_name = "is_null"]
        fn isNull(self: &QUuid) -> bool;

        /// Returns the value in the variant field of the UUID. If the return value is
        /// `QUuidVariant::DCE`, call `version()` to see which layout it uses. The null UUID is
        /// considered to be of an unknown variant.
        fn variant(self: &QUuid) -> QUuidVariant;

        /// Returns the version field of the UUID, if the UUID's variant field is `QUuidVariant::DCE`.
        /// Otherwise it returns `QUuidVariant::VerUnknown`.
        fn version(self: &QUuid) -> QUuidVersion;

        /// Returns the binary representation of this UUID. The byte array is in big endian format,
        /// and formatted according to RFC 4122, section 4.1.2 - "Layout and byte order".
        fn toRfc4122(self: &QUuid) -> QByteArray;
    }

    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        include!("cxx-qt-lib/common.h");

        #[doc(hidden)]
        #[rust_name = "quuid_init_default"]
        fn construct() -> QUuid;

        // QUuid::QUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8)
        #[doc(hidden)]
        #[rust_name = "quuid_init_fields"]
        fn construct(
            l: u32,
            w1: u16,
            w2: u16,
            b1: u8,
            b2: u8,
            b3: u8,
            b4: u8,
            b5: u8,
            b6: u8,
            b7: u8,
            b8: u8,
        ) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_to_qstring"]
        fn toQString(value: &QUuid) -> QString;
    }
    #[namespace = "rust::cxxqtlib1"]
    unsafe extern "C++" {
        #[doc(hidden)]
        #[rust_name = "quuid_new_v3"]
        fn quuidNewV3(ns: &QUuid, data: &[u8]) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_new_v4"]
        fn quuidNewV4() -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_new_v5"]
        fn quuidNewV5(ns: &QUuid, data: &[u8]) -> QUuid;
        #[doc(hidden)]
        #[rust_name = "quuid_from_string"]
        fn quuidFromString(string: &QString) -> QUuid;
        #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_3))]
        #[doc(hidden)]
        #[rust_name = "quuid_from_str"]
        fn quuidFromStr(string: &str) -> QUuid;
    }
}

pub use ffi::{QUuidVariant, QUuidVersion};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct QUuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl Default for QUuid {
    /// Creates the null UUID. `to_string()` will output the null UUID as
    /// "{00000000-0000-0000-0000-000000000000}".
    fn default() -> Self {
        ffi::quuid_init_default()
    }
}

impl fmt::Display for QUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", ffi::quuid_to_qstring(self))
    }
}

impl fmt::Debug for QUuid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
}

impl QUuid {
    /// Creates a UUID with the value specified by the parameters.
    pub fn from_fields(d1: u32, d2: u16, d3: u16, d4: &[u8; 8]) -> Self {
        ffi::quuid_init_fields(
            d1, d2, d3, d4[0], d4[1], d4[2], d4[3], d4[4], d4[5], d4[6], d4[7],
        )
    }

    pub fn as_fields(&self) -> (u32, u16, u16, &[u8; 8]) {
        (self.data1, self.data2, self.data3, &self.data4)
    }

    /// This function returns a new UUID with variant `QUuidVariant::DCE` and version
    /// `QUuidVersion::Md5`. `namespace` is the namespace and `data` is the basic data as described
    /// by RFC 4122.
    pub fn new_v3(namespace: &Self, data: &[u8]) -> Self {
        ffi::quuid_new_v3(namespace, data)
    }

    /// On any platform other than Windows, this function returns a new UUID with variant
    /// `QUuidVariant::DCE` and version `QUuidVersion::Random`. On Windows, a GUID is generated using
    /// the Windows API and will be of the type that the API decides to create.
    pub fn new_v4() -> Self {
        ffi::quuid_new_v4()
    }

    /// This function returns a new UUID with variant `QUuidVariant::DCE` and version
    /// `QUuidVersion::Sha1`. `namespace` is the namespace and `data` is the basic data as described
    /// by RFC 4122.
    pub fn new_v5(namespace: &Self, data: &[u8]) -> Self {
        ffi::quuid_new_v5(namespace, data)
    }

    /// Creates a QUuid object from the string text, which must be formatted as five hex fields
    /// separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex
    /// digit. The curly braces shown here are optional, but it is normal to include them.
    pub fn from_string(uuid: &QString) -> Option<Self> {
        let id = ffi::quuid_from_string(uuid);
        if !id.is_null() || is_null_uuid(&String::from(uuid)) {
            Some(id)
        } else {
            None
        }
    }

    #[cfg(any(cxxqt_qt_version_at_least_7, cxxqt_qt_version_at_least_6_3))]
    /// Creates a QUuid object from the string text, which must be formatted as five hex fields
    /// separated by '-', e.g., "{xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx}" where each 'x' is a hex
    /// digit. The curly braces shown here are optional, but it is normal to include them.
    pub fn from_str(s: &str) -> Option<Self> {
        let id = ffi::quuid_from_str(s);
        if !id.is_null() || is_null_uuid(s) {
            Some(id)
        } else {
            None
        }
    }
}

unsafe impl ExternType for QUuid {
    type Id = type_id!("QUuid");
    type Kind = cxx::kind::Trivial;
}

fn is_null_uuid(uuid: &str) -> bool {
    uuid == "00000000-0000-0000-0000-000000000000"
        || uuid == "{00000000-0000-0000-0000-000000000000}"
}

#[cfg(feature = "uuid")]
impl From<Uuid> for QUuid {
    fn from(value: Uuid) -> Self {
        let (data1, data2, data3, data4) = value.as_fields();
        Self::from_fields(data1, data2, data3, data4)
    }
}

#[cfg(feature = "uuid")]
impl From<QUuid> for Uuid {
    fn from(value: QUuid) -> Self {
        let (data1, data2, data3, data4) = value.as_fields();
        Self::from_fields(data1, data2, data3, data4)
    }
}
