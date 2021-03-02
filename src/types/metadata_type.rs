use llvm_sys::core::{LLVMConstReal, LLVMConstRealOfStringAndSize, LLVMConstArray};
use llvm_sys::execution_engine::LLVMCreateGenericValueOfFloat;
use llvm_sys::prelude::{LLVMTypeRef, LLVMValueRef};

use crate::AddressSpace;
use crate::context::ContextRef;
use crate::types::traits::AsTypeRef;
use crate::types::{Type, PointerType, FunctionType, BasicTypeEnum, ArrayType, VectorType};
use crate::values::{AsValueRef, ArrayValue, FloatValue, GenericValue, IntValue};

/// A `MetadataType` is the type of a metadata.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MetadataType<'ctx> {
    metadata_type: Type<'ctx>,
}

impl<'ctx> MetadataType<'ctx> {
    pub(crate) fn new(metadata_type: LLVMTypeRef) -> Self {
        assert!(!metadata_type.is_null());

        MetadataType {
            metadata_type: Type::new(metadata_type),
        }
    }

    /// Creates a `FunctionType` with this `MetadataType` for its return type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let md_type = context.metadata_type();
    /// let fn_type = md_type.fn_type(&[], false);
    /// ```
    pub fn fn_type(self, param_types: &[BasicTypeEnum<'ctx>], is_var_args: bool) -> FunctionType<'ctx> {
        self.metadata_type.fn_type(param_types, is_var_args)
    }

    /// Creates an `ArrayType` with this `MetadataType` for its element type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let md_type = context.metadata_type();
    /// let md_array_type = md_type.array_type(3);
    ///
    /// assert_eq!(md_array_type.len(), 3);
    /// assert_eq!(md_array_type.get_element_type().into_metadata_type(), md_type);
    /// ```
    pub fn array_type(self, size: u32) -> ArrayType<'ctx> {
        self.metadata_type.array_type(size)
    }

    /// Creates a `VectorType` with this `MetadataType` for its element type.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let md_type = context.metadata_type();
    /// let md_vector_type = md_type.vec_type(3);
    ///
    /// assert_eq!(md_vector_type.get_size(), 3);
    /// assert_eq!(md_vector_type.get_element_type().into_metadata_type(), md_type);
    /// ```
    pub fn vec_type(self, size: u32) -> VectorType<'ctx> {
        self.metadata_type.vec_type(size)
    }

    /// Creates a constant zero value of this `MetadataType`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    /// use inkwell::values::AnyValue;
    ///
    /// let context = Context::create();
    /// let md_type = context.metadata_type();
    /// let md_zero = md_type.const_zero();
    ///
    /// assert_eq!(md_zero.print_to_string().to_string(), "float 0.000000e+00");
    /// ```
    pub fn const_zero(self) -> FloatValue<'ctx> {
        FloatValue::new(self.metadata_type.const_zero())
    }

    /// Gets a reference to the `Context` this `MetadataType` was created in.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use inkwell::context::Context;
    ///
    /// let context = Context::create();
    /// let md_type = context.metadata_type();
    ///
    /// assert_eq!(*md_type.get_context(), context);
    /// ```
    pub fn get_context(self) -> ContextRef<'ctx> {
        self.metadata_type.get_context()
    }
}

impl AsTypeRef for MetadataType<'_> {
    fn as_type_ref(&self) -> LLVMTypeRef {
        self.metadata_type.ty
    }
}
