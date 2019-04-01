extern crate bumpalo;
use bumpalo::{collections::Vec, Bump};

#[test]
fn push_a_bunch_of_items() {
    let b = Bump::new();
    let mut v = Vec::new_in(&b);
    for x in 0..10_000 {
        v.push(x);
    }
}

#[test]
fn collect_in_a_bump_arena() {
    use bumpalo::collections::vec::{IteratorBump, Vec};
    let b = Bump::new();
    (0..10_000).collect_in::<Vec<_>>(&b);
}
