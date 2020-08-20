use crate::value::ValueKind;
pub(crate) use value_ptr::ValuePtr;

use crate::utils::LLVMBool;
use llvm_sys::core::{
    LLVMGetValueKind, LLVMGetValueName2, LLVMIsConstant, LLVMIsNull, LLVMIsUndef,
    LLVMReplaceAllUsesWith, LLVMSetValueName2,
};
use std::{borrow::Cow, ptr::NonNull, slice};

pub struct Type<'a, T: ?Sized>(std::marker::PhantomData<&'a T>);
pub trait TypeExt {}

pub trait ValueExt<'a>: ValuePtr {
    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#gad41082a5f0b15cce7fae46def98e4d68
    fn is_undefined(&self) -> bool {
        debug_assert!(self.value_kind() == ValueKind::Undef);
        unsafe { LLVMBool::new(LLVMIsUndef(self.as_mut_ptr())).is_true() }
    }

    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#ga41f305d0d0b5e0d66755f5c466029377
    fn is_constant(&self) -> bool {
        unsafe { LLVMBool::new(LLVMIsConstant(self.as_mut_ptr())).is_true() }
    }

    fn is_null(&self) -> bool {
        unsafe { LLVMBool::new(LLVMIsNull(self.as_mut_ptr())).is_true() }
    }

    fn is_instruction(&self) -> bool {
        self.value_kind() == ValueKind::Instruction
    }

    fn is_function(&self) -> bool {
        self.value_kind() == ValueKind::Function
    }

    fn is_argument(&self) -> bool {
        self.value_kind() == ValueKind::Argument
    }

    fn is_basic_block(&self) -> bool {
        self.value_kind() == ValueKind::BasicBlock
    }

    fn is_inline_asm(&self) -> bool {
        self.value_kind() == ValueKind::InlineAsm
    }

    fn is_metadata(&self) -> bool {
        self.value_kind() == ValueKind::MetadataAsValue
    }

    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#ga04768f6545009fb71119499c6735b544
    fn value_kind(&self) -> ValueKind {
        unsafe { ValueKind::from(LLVMGetValueKind(self.as_mut_ptr())) }
    }

    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#ga2b63b3a3acdcb11c7642980f91f223ab
    fn get_name(&self) -> Option<Cow<'a, str>> {
        unsafe {
            let mut length = 0;
            let ptr = NonNull::new(LLVMGetValueName2(self.as_mut_ptr(), &mut length) as *mut u8)?;

            let slice = slice::from_raw_parts(ptr.as_ptr(), length);
            Some(String::from_utf8_lossy(slice))
        }
    }

    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#gaf12962c3fbc9d30e373b5330279d38d0
    fn set_name(&self, name: &str) {
        unsafe { LLVMSetValueName2(self.as_mut_ptr(), name.as_ptr() as *mut i8, name.len()) }
    }

    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#ga5180328ab0b7fd00cd814304a33c0b0e
    fn replace_all_uses_with(&self, new: &dyn ValueExt<'a>) {
        assert_eq!(self.value_kind(), new.value_kind());
        unsafe { LLVMReplaceAllUsesWith(self.as_mut_ptr(), new.as_mut_ptr()) }
    }

    /// https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#ga12179f46b79de8436852a4189d4451e0
    fn type_of(&self) -> Type<'a, dyn TypeExt> {
        todo!()
    }

    // TODO:
    // - `LLVMDumpValue`: https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#gacd2559214baba06bbcdc40c0d4cd439c
    // - `LLVMPrintValueToString`: https://llvm.org/doxygen/group__LLVMCCoreValueGeneral.html#ga283f3487d0e658c58d3da00083a8d966
}

static_assertions::assert_obj_safe!(ValueExt);

mod value_ptr {
    use llvm_sys::LLVMValue;

    /// This trait has methods for getting the base pointer of llvm values, as well as functioning as a [sealed trait].
    ///
    /// [sealed trait]: (https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)
    /// [`ValueExt`]: crate::value::ValueExt
    pub trait ValuePtr {
        fn as_ptr(&self) -> *const LLVMValue {
            self.as_mut_ptr() as *const LLVMValue
        }

        fn as_mut_ptr(&self) -> *mut LLVMValue;
    }

    static_assertions::assert_obj_safe!(ValuePtr);
}
