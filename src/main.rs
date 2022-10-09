#![allow(dead_code)]

enum Day {
    Monday, Tuesday, Wednesday, Thursday, Friday, 
    Saturday, Sunday
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true
            
        }
    }
    
}

fn main() {
    let d = Day::Tuesday;

    println!("Is d a weekday? {}", d.is_weekday());
}


// enum Direction{
//     Up,
//     Down,
//     Left,
//     Right
// }
// fn main() {
//     let player_direction:Direction = Direction::Up;

//     match player_direction {
//         Direction::Up => println!("We are heading up!"),
//         Direction::Down => println!("We are heading down!"),
//         Direction::Left => println!("We are heading left!"),
//         Direction::Right => println!("We are heading right!"),
        
//     }
// }
