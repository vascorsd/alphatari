fn main() {
    // rust Option
    let x1: Option<&str> = Some("ola");
    let x2: Option<&str> = None;

    let x1_ = x1.map(|n| n.len()*2);
    let x1__ = x1.map(|n| n.len()*5);

    println!("{:?}", x1_);
    println!("{:?}", x1__);

    println!("{:?}", x2);

    // my Opt
    // more scala styled.
    // ...
}

enum Opt<T> {
    None,
    Some(T)
}

impl<T> Opt<T> {
    fn is_empty(&self) -> bool {
        match self {
            &Opt::Some(_) => false,
            &Opt::None => true
        }
    }

    fn is_non_empty(&self) -> bool {
        !self.is_empty()
    }

    fn get(self) -> T {
        match self {
            Opt::Some(x) => x,
            Opt::None => panic!()
        }
    }

    fn get_or_else<F>(self, if_empty: F) -> T
                where F: FnOnce() -> T {
        match self {
            Opt::Some(x) => x,
            Opt::None => if_empty()
        }
    }

    fn fold<U, E, F>(self, if_empty: E, f: F) -> U
                where E: FnOnce() -> U, F: FnOnce(T) -> U {
        match self {
            Opt::Some(x) => f(x),
            Opt::None => if_empty()
        }
    }

    fn map<U, F>(self, f: F) -> Opt<U>
                where F: FnOnce(T) -> U {
        self.fold(|| Opt::None, |x| Opt::Some(f(x)))
    }

    fn flat_map<U, F>(self, f: F) -> Opt<U>
                where F: FnOnce(T) -> Opt<U> {
        self.fold(|| Opt::None, f)
    }
}