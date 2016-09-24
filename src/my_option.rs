//enum Opt<T> {
//    None,
//    Some(T)
//}
//
//impl<T> Opt<T> {
//    fn is_empty(&self) -> bool {
//        match self {
//            &Opt::Some(_) => false,
//            &Opt::None => true
//        }
//    }
//
//    fn non_empty(&self) -> bool {
//        !self.is_empty()
//    }
//
//    fn fold<U, E, F>(self, if_empty: E, f: F) -> U
//        where E: FnOnce() -> U, F: FnOnce(T) -> U {
//        match self {
//            Opt::Some(x) => f(x),
//            Opt::None => if_empty()
//        }
//    }
//
//    fn get(self) -> T {
//        self.fold(|| panic!(), identity)
//    }
//
//    fn get_or_else<F>(self, if_empty: F) -> T
//        where F: FnOnce() -> T {
//        self.fold(if_empty, identity)
//    }
//
//    fn map<U, F>(self, f: F) -> Opt<U>
//        where F: FnOnce(T) -> U {
//        self.fold(|| Opt::None, |x| Opt::Some(f(x)))
//    }
//
//    fn flat_map<U, F>(self, f: F) -> Opt<U>
//        where F: FnOnce(T) -> Opt<U> {
//        self.fold(|| Opt::None, f)
//    }
//}
//
//fn identity<T>(x: T) -> T { x }