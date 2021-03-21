
use text_io::read;


//TODO add a vector that keeps track of everywhere the deer has been.

// This is the coordinate structure.
#[derive(Debug)]
struct Coordinate {
    x:i32,
    y:i32
}

// This is the hunter structure
#[derive(Debug)]
struct Hunter {
    round           :u32,
    location        :Coordinate,
    distance_mod    :i32,
    base_stealth    :i32,
    base_attack     :i32,
    base_perception :i32,
    is_hiding       :bool,
    prey_id         :i32
}

impl Hunter {

    // This method checks hunter's perception against prey stealth
    fn check_surroundings(&mut self, prey_stealth:&i32) -> bool{
        // roll a 20 sided die and add the distance modifier
        self.base_perception = fastrand::i32(1..21) + self.distance_mod;
        println!("\nstealth:{}", *prey_stealth);
        println!("percep with roll and mod:{}\n", self.base_perception);
        // return if the the hunter sees the deer. 
        *prey_stealth <= self.base_perception
    }

    // This method checks hunter's attack against prey armor class
    fn attack(&mut self, prey_ac:&i32) -> bool {
        // roll a 20 sided die and add the distance modifier
        self.base_attack = fastrand::i32(1..21) + self.distance_mod;
        println!("\nattack:{}", self.base_attack);
        println!("defend:{}", *prey_ac);
        // return if the hunter sees the deer.
        self.base_attack > *prey_ac 
    }


    // This method increases hunter's stealth
    fn hide(&mut self) {
        if !self.is_hiding {
            self.is_hiding = true;
            self.base_stealth += 5
        } 
    }

    // This method decreases hunter's stealth
    fn make_noise(&mut self) {
        if self.is_hiding {
            self.is_hiding = false;
            self.base_stealth -= 5
        }
    }

    
    // This method calculates the distance between the hunter and something.
    fn calculate_distance(&self, prey_loc:&Coordinate) -> i32 {
        let mut a = self.location.x - prey_loc.x;
        if a < 0 {
            a *= -1
        }
        let mut b = self.location.y - prey_loc.y;
        if b < 0 {
            b *= -1
        }
        let c:f32 = ((a*a) + (b*b)) as f32;
        c.sqrt() as i32
    }

    // This method calculates the modifier to abilities based on distance between
    // the hunter and the prey
    fn calculate_modifier(&self, prey_loc:&Coordinate) -> i32{
        let loc_diff = self.calculate_distance(&prey_loc);
        if loc_diff <= 25 {
            return 20
        } else if loc_diff <= 50 {
            return 15
        } else if loc_diff <= 100 {
            return 10
        } else if loc_diff <= 150 {
            return 5 
        } else if loc_diff <= 200 {
            return 0
        } else if loc_diff <= 400 {
            return -5
        } else if loc_diff <= 600 {
            return -10
        } else if loc_diff <= 800 {
            return -15
        } else if loc_diff <= 1000 {
            return -20
        } else {
            return -25
        }
    }

    // This function makes the hunter do stuff over a period of time.
    fn run(&mut self) {
        // create a vector to keep track of the deer's path. This will need to 
        // be changed when the project is scaled bigger.
        let mut prey_path:Vec<Coordinate> = vec!();
        // print out the hunter's stats first
        println!("\n{:?}\n", self);
        // ask for values
        println!("Enter the Prey's id");
        let prey_id:i32 = read!(); // This will need to be mutable when the program is hooked up to the database.
        println!("Enter the Prey's x coordinate");
        let mut prey_loc_x = read!();
        println!("Enter the Prey's y coordinate");
        let mut prey_loc_y = read!();
        let mut prey_loc = Coordinate{x:prey_loc_x,y:prey_loc_y};
        prey_path.push(prey_loc);
        prey_loc = Coordinate{x:prey_loc_x,y:prey_loc_y};
        println!("Enter the Prey's stealth");
        let mut prey_stealth = read!();
        println!("Enter the Prey's Armor Class");
        let prey_ac = read!();
        let mut prey_alive = true;

        // Keep running until the prey is dead. 
        // This will have to change to accomidate multiple prey.
        while prey_alive {
            self.distance_mod = self.calculate_modifier(&prey_loc);
            println!("\ndistance_mod:{}\n", self.distance_mod);
            self.round += 1;
            if self.distance_mod >= -20 {
                if self.check_surroundings(&prey_stealth) {
                    println!("Hunter spotted prey with id {}", prey_id);
                    if self.attack(&prey_ac) {
                        println!("Hunter killed prey with id {}\n", prey_id);
                        prey_alive = false;
                    }
                    else {
                        println!("attack missed\n");
                    }
                }
                else {
                    println!("No prey was spotted.\n");
                    if fastrand::bool() {
                        self.hide();
                    }
                    else {
                        self.make_noise();
                    }
                }
            }
            if !prey_alive {
                break;
            }

            println!("{:?}\n", self);
            println!("Enter the Prey's x coordinate");
            prey_loc_x = read!();
            println!("Enter the Prey's y coordinate");
            prey_loc_y = read!();
            prey_loc = Coordinate{x:prey_loc_x,y:prey_loc_y};
            prey_path.push(prey_loc);
            prey_loc = Coordinate{x:prey_loc_x,y:prey_loc_y};
            println!("Enter the Prey's stealth");
            prey_stealth = read!();
        }
        println!("Prey's path to death is:");
        for location in prey_path{
            println!("{:?}", location);
        }
        
    }
}

// This is the main function that creates a hunter and calls the run function
fn main() {
    let mut hunter1 = Hunter {round:0,
                              location       :Coordinate{x:500,y:500},
                              distance_mod   :0,
                              base_stealth   :20,
                              base_attack    :20,
                              base_perception:20,
                              is_hiding      :false,
                              prey_id        :0};
    hunter1.run()
}
