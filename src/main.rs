enum Direction {
    Up,
    Down,
    Left,
    Right,
}
fn main() {
    let player_direction: Direction = Direction::Up;
    match player_direction {
        Direction::Up => println!("north"),
        Direction::Down => println!("south"),
        Direction::Left => println!("west"),
        Direction::Right => println!("east"),
    }
}
