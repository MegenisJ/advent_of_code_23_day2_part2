use std::fs;
fn main() {
    let mut total = 0;
    for line in fs::read_to_string("input.txt").unwrap().lines(){

        let mut game_number_section = line.split(":");
        
        _=game_number_section.next();
        
        total += power_min_set_cubes(game_number_section.next().unwrap());
    }
    
    println!("{total}");
}

fn power_min_set_cubes(line : &str) -> i32 {
    let mut max_red = 0;
    let mut max_blue = 0;
    let mut max_green = 0;

    for game in line.split(";") {
        for colour in game.split(","){
    
            let mut sections = colour.split(" ");
            
            sections.next();
            
            let x = sections.next().unwrap();
            
            let quantity = x.parse::<i32>().unwrap();
         
            match sections.next().unwrap()
            {
                "red" => if quantity > max_red {max_red = quantity;} ,
                "green" =>  if quantity > max_green {max_green = quantity} ,
                "blue" =>  if quantity > max_blue {max_blue = quantity;},
                _ => (),
            }

        }
    }

    return max_red *  max_blue * max_green ;
}

