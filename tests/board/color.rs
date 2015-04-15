use argon::board::color::StoneColor;

#[test]
fn from_string() {
    let w1 = StoneColor::from_string("w");
    let w2 = StoneColor::from_string("x");
    let b1 = StoneColor::from_string("b");
    let b2 = StoneColor::from_string("B");

    assert!(w1 == Some(StoneColor::White));
    assert!(w2 == None);
    assert!(b1 == Some(StoneColor::Black));
    assert!(b2 == Some(StoneColor::Black));
}

#[test]
fn opposite_color() {
    let w = StoneColor::White;
    let b = StoneColor::Black;

    assert!(w.opposite_color().opposite_color() == w);
    assert!(w.opposite_color() == b);
    assert!(b.opposite_color() == w);
}
