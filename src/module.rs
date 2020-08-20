use crate::{
    utils::{Error, ErrorKind, LLVMBool, LLVMString, EMPTY_CSTR},
    Context, Result,
};
use llvm_sys::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyModule},
    core::{
        LLVMCloneModule, LLVMDisposeModule, LLVMGetModuleIdentifier, LLVMGetSourceFileName,
        LLVMModuleCreateWithNameInContext, LLVMSetModuleIdentifier, LLVMSetSourceFileName,
    },
    debuginfo::LLVMStripModuleDebugInfo,
    LLVMModule,
};
use std::{
    borrow::Cow,
    ffi::CString,
    fmt,
    ptr::{self, NonNull},
    slice,
};

pub struct Module<'a> {
    ptr: NonNull<LLVMModule>,
    context: &'a Context,
}

impl<'a> Module<'a> {
    pub fn new(context: &'a Context) -> Self {
        let ptr = unsafe {
            NonNull::new(LLVMModuleCreateWithNameInContext(
                EMPTY_CSTR,
                context.as_mut_ptr(),
            ))
            .expect("failed to create LLVM Module with no name")
        };

        Self { ptr, context }
    }

    pub fn with_name(context: &'a Context, name: &str) -> Self {
        let c_name = CString::new(name).expect("a Module name had an interior null byte");
        let ptr = unsafe {
            NonNull::new(LLVMModuleCreateWithNameInContext(
                c_name.as_ptr(),
                context.as_mut_ptr(),
            ))
            .unwrap_or_else(|| panic!("failed to create LLVM Module with the name {:?}", name))
        };

        Self { ptr, context }
    }

    pub fn name(&self) -> Cow<'_, str> {
        unsafe {
            let mut length = 0;
            let ptr =
                NonNull::new(LLVMGetModuleIdentifier(self.as_mut_ptr(), &mut length) as *mut u8)
                    .expect("failed to get a LLVM Module's name");

            let slice = slice::from_raw_parts(ptr.as_ptr(), length);
            String::from_utf8_lossy(slice)
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe {
            LLVMSetModuleIdentifier(self.as_mut_ptr(), name.as_ptr() as *const i8, name.len());
        }
        debug_assert_eq!(name, self.name().as_ref());
    }

    pub fn source_file(&self) -> Cow<'_, str> {
        unsafe {
            let mut length = 0;
            let ptr =
                NonNull::new(LLVMGetSourceFileName(self.as_mut_ptr(), &mut length) as *mut u8)
                    .expect("failed to get a LLVM Module's source file");

            let slice = slice::from_raw_parts(ptr.as_ptr(), length);
            String::from_utf8_lossy(slice)
        }
    }

    pub fn set_source_file(&mut self, source_file: &str) {
        unsafe {
            LLVMSetSourceFileName(
                self.as_mut_ptr(),
                source_file.as_ptr() as *const i8,
                source_file.len(),
            )
        };
        debug_assert_eq!(source_file, self.source_file().as_ref());
    }

    pub fn verify(&self) -> Result<()> {
        let mut err_message = ptr::null_mut();
        let succeeded = unsafe {
            LLVMBool::new(LLVMVerifyModule(
                self.as_mut_ptr(),
                LLVMVerifierFailureAction::LLVMReturnStatusAction,
                &mut err_message,
            ))
            .to_bool()
        };

        if succeeded {
            debug_assert!(err_message.is_null());

            Ok(())
        } else {
            debug_assert!(!err_message.is_null());

            Err(Error::new(
                unsafe { LLVMString::from_raw(err_message)? },
                ErrorKind::LLVMError,
            ))
        }
    }

    /// Strip all debug info from the module
    ///
    /// Returns `false` if no debug info was removed
    pub fn strip_debug_info(&mut self) -> bool {
        unsafe { LLVMBool::new(LLVMStripModuleDebugInfo(self.as_mut_ptr())).to_bool() }
    }
}

impl<'a> Module<'a> {
    pub(crate) fn as_mut_ptr(&self) -> *mut LLVMModule {
        self.ptr.as_ptr()
    }
}

impl fmt::Debug for Module<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Module")
            .field("source_file", &self.source_file())
            .finish()
    }
}

impl<'a> Clone for Module<'a> {
    fn clone(&self) -> Self {
        let ptr = unsafe {
            NonNull::new(LLVMCloneModule(self.as_mut_ptr())).expect("failed to clone LLVM Module")
        };

        Self {
            context: self.context,
            ptr,
        }
    }
}

impl<'a> Drop for Module<'a> {
    fn drop(&mut self) {
        unsafe { LLVMDisposeModule(self.as_mut_ptr()) }
    }
}

#[cfg(test)]
mod tests {
    use super::Module;
    use crate::Context;

    #[test]
    fn new_mod() {
        let ctx = Context::new(false);
        let _module = Module::new(&ctx);

        let ctx = Context::new(true);
        let _module = Module::new(&ctx);
    }

    #[test]
    fn mod_with_name() {
        let ctx = Context::new(false);
        let module = Module::with_name(&ctx, "i_am_a_module");
        assert_eq!(module.name().as_ref(), "i_am_a_module");

        let ctx = Context::new(true);
        let module = Module::with_name(&ctx, "i_am_a_module_too");
        assert_eq!(module.name().as_ref(), "i_am_a_module_too");
    }

    #[test]
    fn clone_mod() {
        let ctx = Context::new(false);
        let module = Module::new(&ctx);
        let _module = module.clone();
    }

    #[test]
    fn fails_verification() {
        let ctx = Context::new(false);
        let module = Module::new(&ctx);
        assert!(module.verify().is_err());
    }

    #[test]
    fn strip_debug_info_no_debug() {
        let ctx = Context::new(false);
        let mut module = Module::new(&ctx);
        assert!(!module.strip_debug_info());
    }

    #[test]
    fn set_module_name() {
        let ctx = Context::new(false);
        let mut module = Module::new(&ctx);
        assert!(module.name().is_empty());
        module.set_name("new name");
        assert_eq!(module.name().as_ref(), "new name");
    }

    #[test]
    fn set_module_source_file() {
        let ctx = Context::new(false);
        let mut module = Module::new(&ctx);
        assert!(module.source_file().is_empty());
        module.set_source_file("new name");
        assert_eq!(module.source_file().as_ref(), "new name");
    }

    #[test]
    fn set_module_name_and_source_file() {
        let ctx = Context::new(false);
        let mut module = Module::new(&ctx);
        assert!(module.source_file().is_empty());
        assert!(module.name().is_empty());

        module.set_name("I'm the name");
        module.set_source_file("I'm the source file");
        assert_eq!(module.name().as_ref(), "I'm the name");
        assert_eq!(module.source_file().as_ref(), "I'm the source file");
    }
}
