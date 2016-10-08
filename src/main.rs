extern crate alphatari;

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


