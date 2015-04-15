use argon::board::intersection::Intersection;
use argon::board::color::StoneColor;

#[test]
fn from_stone_color() {
    assert!(Intersection::from_stone_color(StoneColor::White) == Intersection::Stone(StoneColor::White));
}
