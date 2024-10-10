#![warn(clippy::all, clippy::nursery)]

pub use enumflags2;

pub use filterable_enum_derive::FilterableEnum;

pub trait FilterableEnum<Enum> {
    type Id;
    type Filter: EnumFilter<Self::Id>;
    fn filterable_id(&self) -> Self::Id;
    fn filter_ref(&self, filter: impl Into<Self::Filter>) -> Option<&Enum>;
    fn filter_and_take(self, filter: impl Into<Self::Filter>) -> Option<Enum>;
}

pub trait EnumFilter<Id> {
    fn contains(&self, id: Id) -> bool;
}
