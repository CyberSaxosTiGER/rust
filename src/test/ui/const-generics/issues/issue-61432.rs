// run-pass

#![feature(const_generics)]
//~^ WARN the feature `const_generics` is incomplete

fn promote<const N: i32>() {
    // works:
    //
    // let n = N;
    // &n;

    &N;
}

fn main() {
    promote::<0>();
}
