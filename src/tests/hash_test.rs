use crate::util::hash;

#[test]
fn hash_matches() {
    let h1 = hash::hasher(&"Hello");
    let h2 = hash::hasher(&"Hello");

    assert_eq!(h1, h2);
}

#[test]
fn all_unique() {
    let h1 = hash::hasher(&"Hello1");
    let h2 = hash::hasher(&"Hello2");
    let h3 = hash::hasher(&"Hello3");

    assert_ne!(h1, h2);
    assert_ne!(h2, h3);
}