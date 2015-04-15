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

#[test]
fn color() {
    let m1 = Move::Pass(StoneColor::White);
    let m2 = Move::Resign(StoneColor::Black);
    let m3 = Move::Play(Position::new(1, 4), StoneColor::White);
    assert!(m1.color() == StoneColor::White);
    assert!(m2.color() == StoneColor::Black);
    assert!(m3.color() != StoneColor::Black);
}

#[test]
fn is_play() {
    let m1 = Move::Play(Position::new(3, 3), StoneColor::White);
    let m2 = Move::Pass(StoneColor::Black);

    assert!(m1.is_play());
    assert!(!m2.is_play());
}

#[test]
fn is_pass() {
    let m1 = Move::Play(Position::new(3, 3), StoneColor::White);
    let m2 = Move::Pass(StoneColor::Black);

    assert!(!m1.is_pass());
    assert!(m2.is_pass());
}

#[test]
fn is_resign() {
    let m1 = Move::Resign(StoneColor::White);
    let m2 = Move::Pass(StoneColor::Black);

    assert!(m1.is_resign());
    assert!(!m2.is_resign());
}
