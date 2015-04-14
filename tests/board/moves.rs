use argon::board::position::Position;
use argon::board::color::StoneColor;
use argon::board::moves::Move;

#[test]
fn position_ok() {
    let m = Move::Play(Position::new(3, 3), StoneColor::Black);
    assert!(m.position().unwrap() == Position::new(3, 3));
}

#[test]
fn position_fail() {
    let m = Move::Pass(StoneColor::White);
    assert!(m.position().is_none());
}
