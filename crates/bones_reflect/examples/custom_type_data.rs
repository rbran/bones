use bones_reflect::prelude::*;

/// This is a custom type data.
///
/// While it must implement [`HasSchema`] it is fine to just make it opaque.
///
/// In this case we want to store the name of the type in our custom type data.
#[derive(HasSchema, Clone, Default)]
#[schema(opaque)]
struct TypeName(String);

/// In order to make [`TypeName`] derivable, we must implement [`FromType`] for it.
impl<T> FromType<T> for TypeName {
    fn from_type() -> Self {
        Self(std::any::type_name::<T>().to_string())
    }
}

/// We must also implement [`TypeData`] and associate a unique ID to our type data.
impl TypeData for TypeName {
    const TYPE_DATA_ID: ulid::Ulid = Ulid(2042822440874910572532850263467841496);
}

/// Finally we can derive our type data on other types that implement [`HasSchema`] by using the
/// `#[type_datas()]` attribute with one or more type datas to derive.
#[derive(HasSchema, Debug, Default, Clone)]
#[type_datas(TypeName)]
#[repr(C)]
struct MyStruct {
    x: f32,
    y: f32,
}

fn main() {
    let s = MyStruct::schema();
    let tn = s.type_data.get::<TypeName>().unwrap();
    assert_eq!(tn.0, "custom_type_data::MyStruct")
}