use std::ffi::NulError;

use derive_more::{Display, From};

pub type Result<T, E = Zip7Error> = std::result::Result<T, E>;

#[derive(Debug, From, Display)]
pub enum Zip7Error {
    NulByte(NulError),
    NullHandle,
    UnsupportedMethod,
    DataError,
    CrcError,
    Unavailable,
    UnexpectedEnd,
    DataAfterEnd,
    IsNotArc,
    HeadersError,
    WrongPassword,
    NotImplemented,
    NoInterface,
    Abort,
    Fail,
    InvalidFunction,
    ClassNotAvailable,
    OutOfMemory,
    InvalidArg,
}

impl From<i32> for Zip7Error {
    fn from(value: i32) -> Self {
        match value as u32 {
            zip7_sys::kUnsupportedMethod => Self::UnsupportedMethod,
            zip7_sys::kDataError => Self::DataError,
            zip7_sys::kCRCError => Self::CrcError,
            zip7_sys::kUnavailable => Self::Unavailable,
            zip7_sys::kUnexpectedEnd => Self::UnexpectedEnd,
            zip7_sys::kDataAfterEnd => Self::DataAfterEnd,
            zip7_sys::kIsNotArc => Self::IsNotArc,
            zip7_sys::kHeadersError => Self::HeadersError,
            zip7_sys::kWrongPassword => Self::WrongPassword,
            0x80004001 => Self::NotImplemented,
            0x80004002 => Self::NoInterface,
            0x80004004 => Self::Abort,
            0x80030001 => Self::InvalidFunction,
            0x80040111 => Self::ClassNotAvailable,
            0x8007000E => Self::OutOfMemory,
            0x80070057 => Self::InvalidArg,
            _ => Self::Fail,
        }
    }
}

impl std::error::Error for Zip7Error {}
