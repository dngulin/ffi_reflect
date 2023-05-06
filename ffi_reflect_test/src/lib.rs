use ffi_reflect::FfiReflect;

#[derive(Copy, Clone, FfiReflect)]
#[repr(transparent)]
pub struct Foo(f32);

#[derive(Copy, Clone, FfiReflect)]
#[repr(C)]
pub struct Bar {
    pub f1: i64,
    pub f2: i8
}

#[derive(Copy, Clone, FfiReflect)]
#[repr(u8)]
pub enum SomeEnum {
    A = 42,
    B = 17
}

#[derive(Copy, Clone, FfiReflect)]
#[repr(C)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone, FfiReflect)]
#[repr(C)]
pub struct Vec3 {
    pub a: i32,
    pub b: i32,
    pub c: i32,
}

#[derive(Copy, Clone, FfiReflect)]
#[repr(C)]
pub union VecUnion {
    pub two: Vec2,
    pub three: Vec3,
}

#[derive(Copy, Clone, FfiReflect)]
#[repr(C)]
pub struct Baz {
    pub a: Foo,
    pub b: Bar,
    pub c: SomeEnum,
    pub d: [SomeEnum; 10],
    pub e: [Bar; 2],
    pub f: [f64; 7],
    pub g: VecUnion,
    pub h: *const Bar,
    pub i: *mut bool,
    pub j: *const Baz,
    pub k: *const *mut i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use ffi_reflect::FfiType;

    #[test]
    fn smoke() {
        let reflect = Baz::ffi_reflect();
        println!("{:#?}", reflect);

        if let FfiType::Struct(s) = reflect {
            for field in s.fields {
                if let FfiType::Pointer(p) = field.field_type {
                    println!("The field `{}` actually points to:", field.field_name);
                    println!("{:#?}", (p.get_type)());
                }
            }
        }
    }
}
