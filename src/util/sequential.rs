pub trait LowBound {
    fn lowest() -> Self;
}


pub trait Sequential {
    fn consequent(&self) -> Option<Self> where Self: Sized;
    fn sequence(&self) -> Sequence<Self> where Self: Clone {
        Sequence::new(self.clone())
    }
}


pub struct Sequence<T> {
    current: Option<T>,
    first_call: bool,
}


impl<T: Sequential> Sequence<T> {
    fn new(current: T) -> Self {
        Self { current: Some(current), first_call: true }
    }
}


impl<T: Sequential + Clone> Iterator for Sequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.first_call {
            self.first_call = false;
        } else {
            self.current = self.current.clone().and_then(|v| v.consequent());
        }
        self.current.clone()
    }
}

