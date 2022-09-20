#![no_std]

pub use ffi_reflect_derive::FfiReflect;

#[derive(Clone, Copy, Debug)]
pub enum FfiType<'x> {
    Primitive(FfiPrimitive),
    Enum(FfiEnum<'x>),
    Struct(FfiStruct<'x>),
    Union(FfiStruct<'x>),
    Array(FfiArray<'x>),
    Pointer(FfiPointer<'x>),
}

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

#[derive(Clone, Copy, Debug)]
pub struct FfiEnum<'x> {
    pub name: &'x str,
    pub underlying_type: FfiEnumUnderlyingType,
    pub values: &'x [FfiEnumItem<'x>],
}

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

#[derive(Clone, Copy, Debug)]
pub struct FfiEnumItem<'x> {
    pub name: &'x str,
    pub value: &'x str,
}

#[derive(Clone, Copy, Debug)]
pub struct FfiStruct<'x> {
    pub name: &'x str,
    pub size: usize,
    pub align: usize,
    pub fields: &'x [FfiStructField<'x>]
}

#[derive(Clone, Copy, Debug)]
pub struct FfiStructField<'x> {
    pub field_name: &'x str,
    pub field_type: &'x FfiType<'x>
}

#[derive(Clone, Copy, Debug)]
pub struct FfiArray<'x> {
    pub name: &'x str,
    pub item_type: &'x FfiType<'x>,
    pub item_count: usize
}

#[derive(Clone, Copy, Debug)]
pub struct FfiPointer<'x> {
    pub get_type: fn() -> &'x FfiType<'x>,
    pub is_const: bool,
}