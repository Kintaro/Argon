use argon::board::position::Position;

#[test]
fn generate_positions() {
    let p = Position::generate_positions(2);
    let v = vec!(
        Position::new(1, 1),
        Position::new(1, 2),
        Position::new(2, 1),
        Position::new(2, 2));
    assert!(p == v);
}
