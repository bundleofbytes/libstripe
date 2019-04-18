use crate::resources::common::object::Object;

#[derive(Debug, Serialize, Deserialize)]
pub struct List<T> {
    pub data: Vec<T>,
    pub object: Object,
    pub has_more: bool,
    pub total_count: Option<i64>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Deleted {
    pub deleted: bool,
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Period {
    start: i64,
    end: i64,
}

#[derive(Serialize, Debug)]
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

#[derive(Serialize, Debug)]
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

//pub fn to_snakecase(camel: &str) -> String {
//    let mut i = 0;
//    let mut snake = String::new();
//    let mut chars = camel.chars().peekable();
//    while let Some(ch) = chars.next() {
//        if ch.is_uppercase() {
//            if i > 0 && !chars.peek().unwrap_or(&'A').is_uppercase() {
//                snake.push('_');
//            }
//            snake.push(ch.to_lowercase().next().unwrap_or(ch));
//        } else {
//            snake.push(ch);
//        }
//        i += 1;
//    }
//
//    snake
//}
