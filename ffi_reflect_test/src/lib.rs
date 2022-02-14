use ffi_reflect::FfiReflect;

#[derive(FfiReflect)]
#[repr(transparent)]
pub struct Foo(f32);

#[derive(FfiReflect)]
#[repr(C)]
pub struct Bar {
    pub f1: i64,
    pub f2: i8
}

#[derive(FfiReflect)]
#[repr(u8)]
pub enum SomeEnum {
    A = 42,
    B = 17
}

#[derive(FfiReflect)]
#[repr(C)]
pub struct Baz {
    pub a: Foo,
    pub b: Bar,
    pub c: SomeEnum,
    pub d: [SomeEnum; 10],
    pub e: [Bar; 2],
    pub f: [f64; 7],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        println!("{:#?}", Baz::ffi_reflect())
    }
}
