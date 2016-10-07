mod alphatari {
    #[derive(Clone, Copy, Debug)]
    pub enum Opt<T> {
        None,
        Some(T)
    }

    impl<T> Opt<T> {
        pub fn is_empty(&self) -> bool {
            match self {
                &Opt::Some(_) => false,
                &Opt::None => true
            }
        }

        pub fn non_empty(&self) -> bool {
            !self.is_empty()
        }

        pub fn fold<U, E, F>(self, if_empty: E, f: F) -> U
            where E: FnOnce() -> U, F: FnOnce(T) -> U {
            match self {
                Opt::Some(x) => f(x),
                Opt::None => if_empty()
            }
        }

        pub fn get_or_else<F>(self, if_empty: F) -> T
            where F: FnOnce() -> T {
            self.fold(if_empty, free::identity)
        }

        pub fn map<U, F>(self, f: F) -> Opt<U>
            where F: FnOnce(T) -> U {
            self.fold(|| Opt::None, |x| Opt::Some(f(x)))
        }

        pub fn flat_map<U, F>(self, f: F) -> Opt<U>
            where F: FnOnce(T) -> Opt<U> {
            self.fold(|| Opt::None, f)
        }
    }

    mod free {
        pub fn identity<T>(x: T) -> T { x }
    }
}

use alphatari::Opt;

fn main() {
    // rust Option
    let x1: Opt<&str> = Opt::Some("ola");
    let x2: Opt<&str> = Opt::None;

    let x1_ = x1.map(|n| n.len()*2);
    let x1__ = x1.map(|n| n.len()*5);

    println!("{:?}", x1_);
    println!("{:?}", x1__);

    println!("{:?}", x2);

    let y = x1.map(|x| println!("{:?}", x) );


    // my Opt
    // more scala styled.
    // ...

}


