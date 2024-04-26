pub trait FilterableEnum<Enum> {
    type Id;
    type Filter: EnumFilter<Id = Self::Id>;
    fn filterable_id(&self) -> Self::Id;
    fn filter_ref(&self, filter: impl Into<Self::Filter>) -> Option<&Enum>;
    fn filter_and_take(self, filter: impl Into<Self::Filter>) -> Option<Enum>;
}

pub trait EnumFilter {
    type Id;

    fn contains(&self, id: Self::Id) -> bool;
}

#[cfg(test)]
mod test {
    use enumflags2::BitFlags;

    use super::*;

    #[derive(Debug, PartialEq)]
    enum MyEnum {
        A(i32),
        B(String),
        C(f64),
    }

    #[enumflags2::bitflags]
    #[repr(u32)]
    #[derive(Debug, PartialEq, Clone, Copy)]
    enum MyEnumKind {
        A,
        B,
        C,
    }

    #[derive(Debug)]
    struct FilterableMyEnum {
        inner: MyEnum,
        id: MyEnumKind,
    }

    impl EnumFilter for BitFlags<MyEnumKind> {
        type Id = MyEnumKind;

        fn contains(&self, id: Self::Id) -> bool {
            self.intersects(id)
        }
    }

    impl FilterableEnum<MyEnum> for FilterableMyEnum {
        type Id = MyEnumKind;
        type Filter = BitFlags<MyEnumKind>;

        fn filterable_id(&self) -> Self::Id {
            self.id
        }

        fn filter_and_take(self, filter: impl Into<Self::Filter>) -> Option<MyEnum> {
            if filter.into().contains(self.id) {
                Some(self.inner)
            } else {
                None
            }
        }

        fn filter_ref(&self, filter: impl Into<Self::Filter>) -> Option<&MyEnum> {
            if filter.into().contains(self.id) {
                Some(&self.inner)
            } else {
                None
            }
        }
    }

    impl From<MyEnum> for FilterableMyEnum {
        fn from(inner: MyEnum) -> Self {
            let id = match inner {
                MyEnum::A(_) => MyEnumKind::A,
                MyEnum::B(_) => MyEnumKind::B,
                MyEnum::C(_) => MyEnumKind::C,
            };
            FilterableMyEnum { inner, id }
        }
    }

    #[test]
    fn test_filterable_enum() {
        let my_enum: FilterableMyEnum = MyEnum::A(19).into();
        let my_enum_b: FilterableMyEnum = MyEnum::B("B".to_string()).into();
        let filter = MyEnumKind::A | MyEnumKind::B;
        let filter_c = BitFlags::from_flag(MyEnumKind::C);
        assert_eq!(my_enum.filterable_id(), MyEnumKind::A);
        assert_eq!(my_enum.filter_and_take(filter), Some(MyEnum::A(19)));
        assert_eq!(my_enum_b.filterable_id(), MyEnumKind::B);
        assert_eq!(
            my_enum_b.filter_ref(filter),
            Some(&MyEnum::B("B".to_string()))
        );
        assert_eq!(my_enum_b.filter_and_take(filter_c), None);
    }
}
