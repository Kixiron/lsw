use llvm_sys::LLVMIntPredicate;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum IntegerPredicate {
    Equal,
    NotEqual,
    UnsignedGreaterThan,
    UnsignedGreaterThanEqual,
    UnsignedLessThan,
    UnsignedLessThanEqual,
    SignedGreaterThan,
    SignedGreaterThanEqual,
    SignedLessThan,
    SignedLessThanEqual,
}

#[rustfmt::skip]
impl From<LLVMIntPredicate> for IntegerPredicate {
    fn from(pred: LLVMIntPredicate) -> Self {
        match pred {
            LLVMIntPredicate::LLVMIntEQ  => Self::Equal,
            LLVMIntPredicate::LLVMIntNE  => Self::NotEqual,
            LLVMIntPredicate::LLVMIntUGT => Self::UnsignedGreaterThan,
            LLVMIntPredicate::LLVMIntUGE => Self::UnsignedGreaterThanEqual,
            LLVMIntPredicate::LLVMIntULT => Self::UnsignedLessThan,
            LLVMIntPredicate::LLVMIntULE => Self::UnsignedLessThanEqual,
            LLVMIntPredicate::LLVMIntSGT => Self::SignedGreaterThan,
            LLVMIntPredicate::LLVMIntSGE => Self::SignedGreaterThanEqual,
            LLVMIntPredicate::LLVMIntSLT => Self::SignedLessThan,
            LLVMIntPredicate::LLVMIntSLE => Self::SignedLessThanEqual,
        }
    }
}

impl Into<LLVMIntPredicate> for IntegerPredicate {
    fn into(self) -> LLVMIntPredicate {
        match self {
            Self::Equal => LLVMIntPredicate::LLVMIntEQ,
            Self::NotEqual => LLVMIntPredicate::LLVMIntNE,
            Self::UnsignedGreaterThan => LLVMIntPredicate::LLVMIntUGT,
            Self::UnsignedGreaterThanEqual => LLVMIntPredicate::LLVMIntUGE,
            Self::UnsignedLessThan => LLVMIntPredicate::LLVMIntULT,
            Self::UnsignedLessThanEqual => LLVMIntPredicate::LLVMIntULE,
            Self::SignedGreaterThan => LLVMIntPredicate::LLVMIntSGT,
            Self::SignedGreaterThanEqual => LLVMIntPredicate::LLVMIntSGE,
            Self::SignedLessThan => LLVMIntPredicate::LLVMIntSLT,
            Self::SignedLessThanEqual => LLVMIntPredicate::LLVMIntSLE,
        }
    }
}
