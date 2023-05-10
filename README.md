# ffi_reflect

The `ffi_reflect` is a compile time reflection library, that provides the `FfiReflect` derive macro.
The macro itself generates a fucntion `pub const fn ffi_reflect() -> FfiType<'static>`
that can be useful for bindings generation.

You can derive the `FfiReflect` only on structs that are marked with `#[repr(C)]` or `#[repr(transparent)]`,
on enums that are marked with `#[repr($INTEGER_TYPE)]` and unions that are marked with `#[repr(C)]`.

You can use the `ffi_reflect_csharp` to generate C#-types with the exact same memory layout.