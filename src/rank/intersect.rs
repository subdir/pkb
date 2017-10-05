use value::Value;
use sequential::Sequential;


pub trait Intersect<T> {
    fn intersects_with(&self, other: &T) -> bool;
    fn skip_intersecting(&self, skip: &T) -> Option<Self>
    where
        Self: Sized + Clone + Sequential
    {
        self
        .sequence()
        .skip_while(|v| v.intersects_with(skip))
        .next()
    }
}


pub trait IntersectOrd<T> {
    fn all_greater_than(&self, other: &T) -> bool;
    fn skip_to_greater_than(&self, skip: &T) -> Option<Self>
    where
        Self: Sized + Clone + Sequential
    {
        self
        .sequence()
        .skip_while(|v| !v.all_greater_than(skip))
        .next()
    }
}


impl Intersect<Value> for Value {
    fn intersects_with(&self, other: &Value) -> bool { self == other }
}


impl IntersectOrd<Value> for Value {
    fn all_greater_than(&self, other: &Value) -> bool { self > other }
}

