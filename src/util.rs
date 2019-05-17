use crate::resources::common::object::Object;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct List<T> {
    pub data: Vec<T>,
    pub object: Object,
    pub has_more: bool,
    pub total_count: Option<i64>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged, rename_all="lowercase")]
pub enum Expandable<T> {
    Object(T),
    Id(String)
}

macro_rules! match_object {
    ($self:ident) => (
        match $self {
            Expandable::Object(obj) => Some(obj),
            Expandable::Id(..) => None,
        }
    )
}

macro_rules! match_id {
    ($self:ident) => (
        match $self {
            Expandable::Object(..) => None,
            Expandable::Id(id) => Some(id),
        }
    )
}

impl<T> Expandable<T> {
    pub fn as_object(&self) -> Option<&T> {
        match_object!(self)
    }

    pub fn as_id(&self) -> Option<&str> {
        match_id!(self)
    }

    pub fn into_object(self) -> Option<T> {
        match_object!(self)
    }

    pub fn into_id(self) -> Option<String> {
        match_id!(self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Deleted {
    pub object: Object,
    pub deleted: bool,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Period {
    start: i64,
    end: i64,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct RangeBounds {
    pub gt: Option<i64>,
    pub gte: Option<i64>,
    pub lt: Option<i64>,
    pub lte: Option<i64>,
}

impl Default for RangeBounds {
    fn default() -> Self {
        RangeBounds {
            gt: None,
            gte: None,
            lt: None,
            lte: None,
        }
    }
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum RangeQuery {
    Exact(i64),
    Bounds(RangeBounds),
}

impl RangeQuery {
    /// Filter results to exactly match a given value
    pub fn eq(value: i64) -> RangeQuery {
        RangeQuery::Exact(value)
    }

    /// Filter results to be after a given value
    pub fn gt(value: i64) -> RangeQuery {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be after or equal to a given value
    pub fn gte(value: i64) -> RangeQuery {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before to a given value
    pub fn lt(value: i64) -> RangeQuery {
        let mut bounds = RangeBounds::default();
        bounds.gt = Some(value);
        RangeQuery::Bounds(bounds)
    }

    /// Filter results to be before or equal to a given value
    pub fn lte(value: i64) -> RangeQuery {
        let mut bounds = RangeBounds::default();
        bounds.gte = Some(value);
        RangeQuery::Bounds(bounds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Foo {
        bar: Box<Expandable<Bar>>,
    }

    struct Bar {
        foo: Box<Expandable<Foo>>,
    }

    #[test]
    fn expand_chaining() {
        let foo = Foo { bar: Box::new(Expandable::Id("bar".into())) };
        let bar = Bar { foo: Box::new(Expandable::Object(foo)) };
        let foo = Foo { bar: Box::new(Expandable::Object(bar)) };

        let res = foo
            .bar.into_object().unwrap()
            .foo.into_object().unwrap()
            .bar.into_id().unwrap();
        assert_eq!(res, format!("bar"));
    }
}
