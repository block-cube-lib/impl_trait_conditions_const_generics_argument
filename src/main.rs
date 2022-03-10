#![feature(generic_const_exprs)]

struct ArrayWrapper<const N: usize>([i32; N]);

trait Foo {
    fn foo(&self) {}
}

trait FooCondition<const C: bool> {}

impl<const N: usize> FooCondition<{ N < 5 }> for ArrayWrapper<N> {}

impl<const N: usize> Foo for ArrayWrapper<N> where ArrayWrapper<N>: FooCondition<true> {}

fn main() {
    ArrayWrapper([]).foo(); // OK
    ArrayWrapper([0]).foo(); // OK
    ArrayWrapper([0, 1]).foo(); // OK
    ArrayWrapper([0, 1, 2]).foo(); // OK
    ArrayWrapper([0, 1, 2, 3]).foo(); // OK
    // ArrayWrapper([0, 1, 2, 3, 4]).foo(); // Error
}
