#[test]
fn iter() {
    let v = vec![1, 2, 3];
    let mut i = v.iter();
    assert_eq!(i.next(), Some(&1));
    assert_eq!(i.next(), Some(&2));
    assert_eq!(i.next(), Some(&3));
    assert_eq!(i.next(), None);
}
