#![no_std]

pub use ffi_reflect_derive::FfiReflect;

/// An enum representing supported FFI types
#[derive(Clone, Copy, Debug)]
pub enum FfiType<'x> {
    Primitive(FfiPrimitive),
    Enum(FfiEnum<'x>),
    Struct(FfiStruct<'x>),
    Union(FfiStruct<'x>),
    Array(FfiArray<'x>),
    Pointer(FfiPointer<'x>),
}

/// An enum representing supported primitive types
#[derive(Clone, Copy, Debug)]
pub enum FfiPrimitive {
    BOOL,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
}

/// A struct representing a C-compatible enum
#[derive(Clone, Copy, Debug)]
pub struct FfiEnum<'x> {
    pub name: &'x str,
    pub underlying_type: FfiEnumUnderlyingType,
    pub values: &'x [FfiEnumItem<'x>],
}

/// An enum representing the underlying type of a C-compatible enum
#[derive(Clone, Copy, Debug)]
pub enum FfiEnumUnderlyingType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

/// A struct representing a numeric enum memeber
#[derive(Clone, Copy, Debug)]
pub struct FfiEnumItem<'x> {
    pub name: &'x str,
    pub value: &'x str,
}

/// A struct representing a C-compatible struct
#[derive(Clone, Copy, Debug)]
pub struct FfiStruct<'x> {
    pub name: &'x str,
    pub size: usize,
    pub align: usize,
    pub fields: &'x [FfiStructField<'x>]
}

/// A struct representing a struct field
#[derive(Clone, Copy, Debug)]
pub struct FfiStructField<'x> {
    pub field_name: &'x str,
    pub field_type: &'x FfiType<'x>
}

/// A struct representing a C-compatible array
#[derive(Clone, Copy, Debug)]
pub struct FfiArray<'x> {
    pub name: &'x str,
    pub item_type: &'x FfiType<'x>,
    pub item_count: usize
}

/// A struct representing a C-compatible pointer type
#[derive(Clone, Copy, Debug)]
pub struct FfiPointer<'x> {
    pub get_type: fn() -> &'x FfiType<'x>,
    pub is_const: bool,
}