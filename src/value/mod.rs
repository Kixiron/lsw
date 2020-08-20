mod constant;
mod value_ext;
mod value_kind;

pub use value_ext::ValueExt;
pub(crate) use value_ext::ValuePtr;
pub use value_kind::ValueKind;

use llvm_sys::LLVMValue;
use std::{marker::PhantomData, ptr::NonNull};

struct Value<'a, T: ?Sized> {
    ptr: NonNull<LLVMValue>,
    __type: PhantomData<&'a T>,
}

impl<'a, T: ?Sized> ValueExt<'a> for Value<'a, T> {}

impl<'a, T: ?Sized> ValuePtr for Value<'a, T> {
    fn as_mut_ptr(&self) -> *mut LLVMValue {
        self.ptr.as_ptr()
    }
}
