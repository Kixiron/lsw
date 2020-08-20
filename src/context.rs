use crate::{utils::LLVMBool, Module};
use llvm_sys::{
    core::{
        LLVMContextCreate, LLVMContextDispose, LLVMContextSetDiscardValueNames,
        LLVMContextShouldDiscardValueNames,
    },
    LLVMContext,
};
use std::{fmt, ptr::NonNull};

#[repr(transparent)]
pub struct Context {
    ptr: NonNull<LLVMContext>,
}

impl Context {
    pub fn new(discard_names: bool) -> Self {
        let ptr =
            unsafe { NonNull::new(LLVMContextCreate()).expect("failed to create LLVM Context") };

        let mut this = Self { ptr };
        this.discard_names(discard_names);

        this
    }

    pub fn discard_names(&mut self, discard_names: bool) {
        unsafe { LLVMContextSetDiscardValueNames(self.as_mut_ptr(), discard_names as i32) };
        debug_assert_eq!(self.will_discard_names(), discard_names);
    }

    pub fn will_discard_names(&self) -> bool {
        unsafe { LLVMBool::new(LLVMContextShouldDiscardValueNames(self.as_mut_ptr())).to_bool() }
    }

    pub fn create_module(&self) -> Module<'_> {
        Module::new(self)
    }

    pub fn create_module_with_name(&self, name: &str) -> Module<'_> {
        Module::with_name(self, name)
    }
}

impl Context {
    pub(crate) fn as_mut_ptr(&self) -> *mut LLVMContext {
        self.ptr.as_ptr()
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new(false)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe { LLVMContextDispose(self.ptr.as_ptr()) }
    }
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Context")
            .field("discard_names", &self.will_discard_names())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::Context;

    #[test]
    fn discarding_context() {
        let _context = Context::new(true);
    }

    #[test]
    fn non_discarding_context() {
        let _context = Context::new(false);
    }

    #[test]
    fn debug_context() {
        let context = Context::new(false);
        println!("{:?}", context);

        let context = Context::new(true);
        println!("{:?}", context);
    }

    #[test]
    fn discard_names() {
        let context = Context::new(false);
        assert!(!context.will_discard_names());

        let mut context = Context::new(true);
        assert!(context.will_discard_names());

        context.discard_names(false);
        assert!(!context.will_discard_names());

        context.discard_names(true);
        assert!(context.will_discard_names());
    }
}
