use proc_macro::TokenStream;
use proc_macro2::{Literal, Span};
use quote::{quote, quote_spanned, ToTokens};
use syn::*;
use syn::spanned::Spanned;


const PRIMITIVES : &[&str] = &["bool", "u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "f32", "f64"];

#[proc_macro_derive(FfiReflect)]
pub fn derive_ffi_reflect(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let type_expr = input.ident;
    let ffi_type_expr = get_ffi_type_expr(&input.data, &input.attrs, &type_expr);

    let expanded = quote! {
        impl #type_expr {
            pub const fn ffi_reflect() -> &'static ::ffi_reflect::FfiType<'static> {
                #ffi_type_expr
            }
        }
    };

    TokenStream::from(expanded)
}

fn get_ffi_type_expr(data: &Data, attrs: &[Attribute], type_expr: &Ident) -> proc_macro2::TokenStream {
    match data {
        Data::Struct(s) => {
            if let Some(repr) = get_repr_type(attrs) {
                match repr.as_str() {
                    "C" => return get_struct_type_expr(s, type_expr),
                    "transparent" => return get_transparent_type_expr(s),
                    _ => {}
                }
            }

            panic!("FfiReflect derive macro only works on structs with [repr(C)] or [repr(transparent)]")
        },
        Data::Enum(e) => {
            let enum_reprs = ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64"];
            if let Some(repr) = get_repr_type(attrs) {
                let repr_str = repr.as_str();
                if enum_reprs.contains(&repr_str) {
                    return get_enum_type_expr(e, type_expr, repr_str);
                }
            }

            panic!("FfiReflect derive macro only works on enums with [repr(int)]")
        },
        Data::Union(u) => {
            if let Some(repr) = get_repr_type(attrs) {
                if repr.as_str() == "C" {
                    return get_union_type_expr(u, type_expr);
                }
            }

            panic!("FfiReflect derive macro only works on unions with [repr(C)]");
        },
    }
}

fn get_repr_type(attributes: &[Attribute]) -> Option<String> {
    for attribute in attributes {
        if let Meta::List(list) = attribute.parse_meta().unwrap() {
            if let Some(attr) = list.path.get_ident() {
                if attr.to_string().as_str() == "repr" {
                    if let Some(NestedMeta::Meta(Meta::Path(path))) = list.nested.first() {
                        if let Some(arg) = path.get_ident() {
                            return Some(arg.to_string())
                        }
                    }
                }
            }
        }
    }
    None
}

fn get_struct_type_expr(s: &DataStruct, type_expr: &Ident) -> proc_macro2::TokenStream {
    let mut field_exprs = Vec::new();

    match &s.fields {
        Fields::Named(fields) => {
            for field in fields.named.iter() {
                let field_name_expr = Literal::string(&field.ident.as_ref().unwrap().to_string());
                let field_type_expr = get_inner_type_expr(&field.ty);

                field_exprs.push(quote_spanned!(field.span() => {
                    ::ffi_reflect::FfiStructField {
                        field_name: #field_name_expr,
                        field_type: #field_type_expr
                    }
                }));
            }

        }
        Fields::Unnamed(fields) => {
            for (index, field) in fields.unnamed.iter().enumerate() {
                let field_name_expr = Literal::string(&format!("item_{}", index));
                let field_type_expr = get_inner_type_expr(&field.ty);

                field_exprs.push(quote_spanned!(field.span() => {
                    ::ffi_reflect::FfiStructField {
                        field_name: #field_name_expr,
                        field_type: #field_type_expr
                    }
                }));
            }

        },
        Fields::Unit => panic!("Unit structs can not derive FfiReflect"),
    };

    let name_expr = Literal::string(&type_expr.to_string());

    quote_spanned!(s.fields.span() => {
        const TYPE_SIZE : usize = ::core::mem::size_of::<#type_expr>();
        const TYPE_ALIGN : usize = ::core::mem::align_of::<#type_expr>();
        const TYPE_INFO : ::ffi_reflect::FfiType<'static> = ::ffi_reflect::FfiType::Struct(::ffi_reflect::FfiStruct{
            name: #name_expr,
            size: TYPE_SIZE,
            align: TYPE_ALIGN,
            fields: &[
                #(#field_exprs),*
            ],
        });
        &TYPE_INFO
    })
}

fn get_union_type_expr(u: &DataUnion, type_expr: &Ident) -> proc_macro2::TokenStream {
    let mut field_exprs = Vec::new();

    for field in u.fields.named.iter() {
        let field_name_expr = Literal::string(&field.ident.as_ref().unwrap().to_string());
        let field_type_expr = get_inner_type_expr(&field.ty);

        field_exprs.push(quote_spanned!(field.span() => {
                    ::ffi_reflect::FfiStructField {
                        field_name: #field_name_expr,
                        field_type: #field_type_expr
                    }
                }));
    }

    let name_expr = Literal::string(&type_expr.to_string());

    quote_spanned!(u.fields.span() => {
        const TYPE_SIZE : usize = ::core::mem::size_of::<#type_expr>();
        const TYPE_ALIGN : usize = ::core::mem::align_of::<#type_expr>();
        const TYPE_INFO : ::ffi_reflect::FfiType<'static> = ::ffi_reflect::FfiType::Union(::ffi_reflect::FfiStruct{
            name: #name_expr,
            size: TYPE_SIZE,
            align: TYPE_ALIGN,
            fields: &[
                #(#field_exprs),*
            ],
        });
        &TYPE_INFO
    })
}

fn get_inner_type_expr(t: &Type) -> proc_macro2::TokenStream {
    match t {
        Type::Array(a) => {
            let item_type = a.elem.as_ref();
            let array_name_expr = Literal::string(&get_array_type_name(item_type, &a.len));

            let item_type_expr = get_inner_type_expr(item_type);
            let item_count_expr = &a.len;

            return quote_spanned!(t.span() => {
                &::ffi_reflect::FfiType::Array(::ffi_reflect::FfiArray{
                    name: #array_name_expr,
                    item_type: #item_type_expr,
                    item_count: #item_count_expr,
                })
            })
        }
        Type::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                let last_seg_string = seg.ident.to_string();
                let last_seg_str = last_seg_string.as_str();

                return if PRIMITIVES.contains(&last_seg_str) {
                    get_primitive_type_expr(t.span(), last_seg_str)
                } else {
                    quote_spanned!(t.span() => #t::ffi_reflect())
                }
            }
        }
        _ => {}
    }
    panic!("Failed to impl type info")
}

fn get_primitive_type_expr(span: Span, type_name: &str) -> proc_macro2::TokenStream {
    match type_name {
        "bool" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::BOOL)),
        "u8" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::U8)),
        "u16" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::U16)),
        "u32" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::U32)),
        "u64" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::U64)),
        "i8" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::I8)),
        "i16" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::I16)),
        "i32" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::I32)),
        "i64" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::I64)),
        "f32" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::F32)),
        "f64" => quote_spanned!(span => &::ffi_reflect::FfiType::Primitive(::ffi_reflect::FfiPrimitive::F64)),
        _ => unreachable!()
    }
}

fn get_array_type_name(t: &Type, len_expr: &Expr) -> String {
    if let Expr::Lit(lit) = len_expr {
        if let Lit::Int(int) = &lit.lit {
            let len : usize = int.base10_parse().unwrap();
            let type_name = get_type_name(t);
            return format!("ArrayOf{}{}", len, type_name);
        }
    }
    panic!("Failed to get array length")
}

fn get_type_name(t: &Type) -> String {
    match t {
        Type::Array(a) => return get_array_type_name(a.elem.as_ref(), &a.len),
        Type::Path(p) => {
            if let Some(seg) = p.path.segments.last() {
                return seg.ident.to_string()
            }
        },
        _ => {}
    }
    panic!("Failed to get type name")
}

fn get_transparent_type_expr(s: &DataStruct) -> proc_macro2::TokenStream {
    if s.fields.len() != 1 {
        unreachable!()
    }

    let field = match &s.fields {
        Fields::Named(fields) => fields.named.first().unwrap(),
        Fields::Unnamed(fields) => fields.unnamed.first().unwrap(),
        _ => unreachable!()
    };

    let type_seg = match &field.ty {
        Type::Path(p) => p.path.segments.first().unwrap(),
        _ => unreachable!()
    };

    let type_string = type_seg.ident.to_string();
    let type_str = type_string.as_str();

    if !PRIMITIVES.contains(&type_str) {
        panic!("[repr(transparent)] only is supported over primitives")
    }

    get_primitive_type_expr(type_seg.span(), type_str)
}

fn get_enum_type_expr(e: &DataEnum, type_expr: &Ident, repr: &str) -> proc_macro2::TokenStream {
    let mut item_exprs = Vec::new();

    for variant in e.variants.iter() {
        let variant_name_expr = Literal::string(&variant.ident.to_string());
        let (_, variant_value_expr) = &variant.discriminant.as_ref()
            .expect("All enum values should be assigned for deriving FfiReflect");

        let value_literal = Literal::string(&variant_value_expr.into_token_stream().to_string());

        item_exprs.push(quote_spanned!(variant.span() => {
            ::ffi_reflect::FfiEnumItem {
                name: #variant_name_expr,
                value: #value_literal
            }
        }));
    }

    let underlying_type_expr = get_underlying_type_expr(e.enum_token.span, repr);

    let name_expr = Literal::string(&type_expr.to_string());
    quote_spanned!(e.variants.span() => {
        const TYPE_INFO : ::ffi_reflect::FfiType<'static> = ::ffi_reflect::FfiType::Enum(::ffi_reflect::FfiEnum{
            name: #name_expr,
            underlying_type: #underlying_type_expr,
            values: &[
                #(#item_exprs),*
            ]
        });
        &TYPE_INFO
    })
}

fn get_underlying_type_expr(span: Span, repr: &str) -> proc_macro2::TokenStream {
    match repr {
        "u8" => quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::U8),
        "u16"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::U16),
        "u32"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::U32),
        "u64"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::U64),
        "i8"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::I8),
        "i16"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::I16),
        "i32"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::I32),
        "i64"=> quote_spanned!(span => ::ffi_reflect::FfiEnumUnderlyingType::I64),
        _ => unreachable!(),
    }
}
