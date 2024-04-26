use filterable_enum::FilterableEnum;

#[derive(Debug, PartialEq, FilterableEnum)]
enum MyEnum {
    A(i32),
    B(String),
    C(f64),
}

#[test]
fn test_filter_found() {
    let my_enum: FilterableMyEnum = MyEnum::A(19).into();
    let my_enum_b: FilterableMyEnum = MyEnum::B("B".to_string()).into();
    let filter = MyEnumKind::A | MyEnumKind::B;
    assert_eq!(my_enum.filter_and_take(filter), Some(MyEnum::A(19)));
    assert_eq!(my_enum_b.filterable_id(), MyEnumKind::B);
    assert_eq!(
        my_enum_b.filter_ref(filter),
        Some(&MyEnum::B("B".to_string()))
    );
}

#[test]
fn test_filter_not_found() {
    let my_enum: FilterableMyEnum = MyEnum::A(19).into();
    let my_enum_b: FilterableMyEnum = MyEnum::B("B".to_string()).into();
    let filter = MyEnumKind::A;
    assert_eq!(my_enum.filter_and_take(filter), Some(MyEnum::A(19)));
    assert_eq!(my_enum_b.filter_ref(filter), None);
}

#[test]
fn test_filter_not_found_2() {
    let my_enum: FilterableMyEnum = MyEnum::A(19).into();
    let my_enum_b: FilterableMyEnum = MyEnum::B("B".to_string()).into();
    let filter = MyEnumKind::C;
    assert_eq!(my_enum.filter_and_take(filter), None);
    assert_eq!(my_enum_b.filter_ref(filter), None);
}

#[test]
fn test_filter_id() {
    let my_enum: FilterableMyEnum = MyEnum::A(19).into();
    let my_enum_b: FilterableMyEnum = MyEnum::B("B".to_string()).into();
    let my_enum_c: FilterableMyEnum = MyEnum::C(3.14).into();
    assert_eq!(my_enum.filterable_id(), MyEnumKind::A);
    assert_eq!(my_enum_b.filterable_id(), MyEnumKind::B);
    assert_eq!(my_enum_c.filterable_id(), MyEnumKind::C);
}
