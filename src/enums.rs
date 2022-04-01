enum Movement {
    // varitants
    Up,
    Down,
    Left,
    Right
}
fn movement_avatar(movement: Movement) {
    // perform action depending on info
    match movement {
        Movement::Up => println!("Avater is moving Up"),
        Movement::Down => println!("Avater is moving Down"),
        Movement::Left => println!("Avater is moving Left"),
        Movement::Right => println!("Avater is moving Right"),
    }
}

pub fn run() {
    let avater1 = Movement::Up;
    let avater2 = Movement::Down;
    let avater3 = Movement::Left;
    let avater4 = Movement::Right;
    movement_avatar(avater1);
    movement_avatar(avater2);
    movement_avatar(avater3);
    movement_avatar(avater4);
}