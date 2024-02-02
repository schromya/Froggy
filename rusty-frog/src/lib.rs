use std::{io::{self}, thread, time::Duration};
use rand::Rng;

pub struct Frog {
    _name: String,
    _weight: f64,
    _size: f64,
    _is_superfrog: bool,
    _is_chilling: bool
}

impl Frog {
    pub fn new() -> Frog { // Like a constructor, returns an instance of the Frog struct
        Frog{
            _name: "Rusty Frog".to_string(),
            _weight: 1.0,
            _size: 2.0,
            _is_superfrog: false,
            _is_chilling: true
        }
    }

    pub fn new_with_name(name: String) -> Frog { // Returns a different instance (with name param)
        Frog {
             _name: name,
             _weight: 1.0,
             _size: 2.0,
             _is_superfrog: false,
             _is_chilling: true
        }
    }

    pub fn new_with_name_and_weight(name: String, weight: f64) -> Frog { // Returns another instance
        Frog {
            _name: name,
            _weight: weight,
            _size: 2.0,
            _is_superfrog: false,
            _is_chilling: true
        }
    }

    pub fn is_chilling(&self) -> bool { // Public method (accessor of _is_chilling)
        self._is_chilling
    }    

    pub fn frog_life(&mut self) {

        println!("\n{} is chilling, what do you want to do?", self._name.trim());
        println!("| 0. Quit | 1. Poke | 2. Prod | 3. Put on scale | 4. Feed | Choice: ");
        let mut input: String = String::new();
        let mut choice: i32;
        io::stdin()
            .read_line(&mut input)
            .expect("Froggy didn't understand!");
        choice = input.trim().parse().unwrap();
        while choice < 0 || choice > 4 {
            println!("Invalid input. Try again");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("Froggy didn't understand!");
            choice = input.trim().parse().unwrap();
        }
        match choice {
            0 => self._is_chilling = false,
            1 => self.ribbit(),
            2 => self.hop(),
            3 => self.test_weight(),
            4 => self.munchies(),
            _ => println!("Something else"),
        }
        
    }

    pub fn hop(&self) {
        println!("{} hopped {} times!", self._name.trim(), self.hop_num());
    }

    fn hop_num(&self) -> u32 {
        let rnd: i32 = rand::thread_rng().gen_range(2..=11);
        return rnd as u32;
    }

    fn ribbit(&self) {
        println!("{} goes {}", self._name.trim(), self.ribbit_sound());
    }

    fn ribbit_sound(&self) -> String {
        let rnd: i32 = rand::thread_rng().gen_range(0..=3);
        match rnd {
            0 => "Ribbit!".to_string(),
            1 => "RibBit ribBit".to_string(),
            2 => "ribBIT!".to_string(),
            3 => "RiBbIt".to_string(),
            _ => "<awkward silence>".to_string(),
        }
    }

    pub fn test_weight(&self) {
        print!("Testing your froggy's weight");
        for _ in 0..3 {
            // print!(". ");
            io::Write::flush(&mut io::stdout()).expect("Failed to flush buffer");
            thread::sleep(Duration::from_millis(750));
            print!(". ");
        }
        io::Write::flush(&mut io::stdout()).expect("Failed to flush buffer");
        thread::sleep(Duration::from_millis(750));
        print!("\n");
        println!("Ding Ding Ding: {}", self.calculate_weight());
    }

    fn calculate_weight(&self) -> String {
        if self._weight < 0.0 { return "Need more frog, lest ghost.".to_string(); }
        else if self._weight < 2.0 { return "Froggy needs more bugs and munchies.".to_string(); }
        else if self._weight < 4.0 { return "Froggy is big boy.".to_string(); }
        else { return "What does frog eat? Humans?".to_string(); }
    }

    fn munchies(&mut self) {
        let mut input: String = String::new();
        let mut choice: u32;
        println!("{} is hungry. What should frog eat?", self._name);
        print!("| 1. Flies | 2. Slippery Fiskies | 3. Humans | 4. Strange green stuff | Choice: ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush buffer");
        io::stdin()
            .read_line(&mut input)
            .expect("That's not a food!");
        choice = input.trim().parse().unwrap();
        while choice < 1 || choice > 4 {
            println!("Invalid input. Try again");
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("That's not a food!");
            choice = input.trim().parse().unwrap();
        }
        match choice {
            1 => {
                println!("* Froggy consumed lots of flies. Looking fatter now");
                self._weight += 2.0;
            }
            2 => {
                println!("* Froggy downed some slippery fiskies");
                self._weight += 3.0;
            }
            3 => {
                println!("* Froggy chomped humans. Ouch");
                self._weight += 5.0;
            }
            4 => {
                println!("* Uh oh");
                self._weight = 10_000.0;
                self._is_superfrog = true;
            }
            _ => {
                println!("* Wildcard food!");
            }
        }

        println!(">> {} had a case of the munchies, but that's all better now!\n>> {}",
            self._name.trim(), self.ribbit_sound());

    }


}