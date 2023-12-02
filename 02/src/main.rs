use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct CubeSet {
    red: u32,
    green: u32,
    blue: u32
}

impl CubeSet {
    fn new() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    // Does the give cubeset "fit" in this one?
    fn fits(&self, set: &Self) -> bool {
        self.red >= set.red &&
        self.green >= set.green &&
        self.blue >= set.blue
    }

    // Enlarge this cubset to make the other fit
    fn make_room(&mut self, set: &Self) {
        self.red = max(self.red, set.red);
        self.green = max(self.green, set.green);
        self.blue = max(self.blue, set.blue);
    }

    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug, PartialEq)]
struct Game {
    id: u32,
    sets: Vec<CubeSet>
}

#[derive(Debug, PartialEq, Eq)]
struct GameParseError;

impl FromStr for Game {
    type Err = GameParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let (game_part, sets_part) = s.split_once(":").ok_or(GameParseError)?;

        // Extract Game ID, create Game object
        let (_, game_number) = game_part.split_once(" ").ok_or(GameParseError)?;
        let game_id: u32 = game_number.parse().ok().ok_or(GameParseError)?;
        let mut game = Game::new(game_id);

        let sets = sets_part
            .split(';')
            .map(str::trim);

        for set_string in sets {
            let mut set = CubeSet::new();
            let cube_counts = set_string
                .split(",")
                .map(str::trim)
                .map(|c| {c.split_once(" ")});

            for cc in cube_counts {
                let count: (&str, &str) = cc.ok_or(GameParseError)?;
                match count {
                    (c, "red")   => set.red   = c.parse().ok().ok_or(GameParseError)?,
                    (c, "blue")  => set.blue  = c.parse().ok().ok_or(GameParseError)?,
                    (c, "green") => set.green = c.parse().ok().ok_or(GameParseError)?,
                    _ => Err(GameParseError)?
                }
            }
            game.sets.push(set);
        }

        Ok(game)
        //let game_id = match game_part[5..].parse::<u32>() {
    }
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id: id,
            sets: Vec::new(),
        }
    }
}


fn main() -> Result<(), GameParseError>  {
    if let Ok(lines) = read_lines("input.txt") {
        let max_set = CubeSet {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut id_sum = 0;
        let mut power_sum = 0;
        for line in lines {
            if let Ok(line_str) = line {
                let game = Game::from_str(&line_str)?;
                let mut game_possible = true;
                let mut needed_cubes = CubeSet::new();
                for set in game.sets {
                    game_possible &= max_set.fits(&set);
                    needed_cubes.make_room(&set);
                }
                if game_possible {
                    id_sum += game.id;
                    //println!("Game {} is possible", game.id);
                } else {
                    //println!("Game {} is NOT possible", game.id);
                }
                power_sum += needed_cubes.power()
            }
        }
        println!("Possible game id sum: {}", id_sum);
        println!("Game power sum: {}", power_sum);
    }
    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parsing() {
        if let Ok(game) = Game::from_str("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green") {
            assert_eq!(game.id, 1);
            assert_eq!(game.sets.len(), 3);
        } else {
            assert!(false)
        }
    }

    #[test]
    fn test_power() {
        assert_eq!(CubeSet { red: 4, green: 2, blue: 6 }.power(), 48);
    }
}
