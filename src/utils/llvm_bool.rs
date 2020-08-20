use std::num::NonZeroU32;

/// A wrapper around LLVM's boolean values. LLVM's native `LLVMBool` can either
/// represent a `0 == false`, `NonZero == true` boolean value or a `0 == Success`,
/// `NonZero == Error` value.
/// This struct represents the former, for the latter use [`LLVMStatus`]
///
/// [`LLVMStatus`]: crate::utils::LLVMStatus
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct LLVMBool(bool);

impl LLVMBool {
    pub fn new(code: i32) -> Self {
        match code {
            0 => Self(false),
            _ => Self(true),
        }
    }

    pub const fn to_bool(self) -> bool {
        self.0
    }

    pub const fn is_true(self) -> bool {
        self.0
    }

    pub const fn is_false(self) -> bool {
        !self.is_true()
    }
}

impl From<i32> for LLVMBool {
    fn from(code: i32) -> Self {
        Self::new(code)
    }
}

impl Into<i32> for LLVMBool {
    fn into(self) -> i32 {
        if self.0 {
            1
        } else {
            0
        }
    }
}

impl From<bool> for LLVMBool {
    fn from(b: bool) -> Self {
        Self(b)
    }
}

impl Into<bool> for LLVMBool {
    fn into(self) -> bool {
        self.0
    }
}

/// A wrapper around LLVM's boolean values. LLVM's native `LLVMBool` can either
/// represent a `0 == false`, `NonZero == true` boolean value or a `0 == Success`,
/// `NonZero == Error` value.
/// This enum represents the latter, for the former use [`LLVMBool`]
///
/// [`LLVMBool`]: crate::utils::LLVMBool
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LLVMStatus {
    Success,
    Failure(NonZeroU32),
}
