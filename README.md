# ffi_reflect

The `ffi_reflect` is a compile time reflection library, that provides the `FfiReflect` derive macro.
The macro itself generates a fucntion `pub const fn ffi_reflect() -> FfiType<'static>`
that can be useful for bindings generation.

Note that you can derive the `FfiReflect` only on structs that marked with `#[repr(C)]` or `#[repr(transparent)]`,
on enums that marked with `#[repr($INTEGER_TYPE)]` and unions marked with `#[repr(C)]`.