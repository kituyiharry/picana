macro_rules! dart_c_bool {
    $($x:ident < i32) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kBool,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x }
        };
    }}
}

macro_rules! dart_c_int32 {
    $($x:ident < i32) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kInt32,
            value: _Dart_CObject__bindgen_ty_1 { as_int32: $x }
        };
    }}
}

macro_rules! dart_c_int64 {
    $($x:ident < i64) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kInt64,
            value: _Dart_CObject__bindgen_ty_1 { as_int32: $x }
        };
    }}
}

macro_rules! dart_c_double {
    $($x:ident < f64 ) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kDouble,
            value: _Dart_CObject__bindgen_ty_1 { as_double: $x }
        };
    }}
}

macro_rules! dart_c_string {
    $($x:ident) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kString,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x }
        };
    }}
}

macro_rules! dart_c_array {
    $($x:ident) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kArray,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x }
        };
    }}
}

macro_rules! dart_c_typed_data {
    $($x:ident) => {{
        Dart_CObject{
            type_: Dart_CObject_Type::Dart_CObject_kTypedData,
            value: _Dart_CObject__bindgen_ty_1 { as_bool: $x }
        };
    }}
}
