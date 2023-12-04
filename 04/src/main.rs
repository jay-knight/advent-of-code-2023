use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    winning_numbers: HashSet<u32>,
    my_numbers: HashSet<u32>,
}

impl Card {
    fn new() -> Self {
        Self {
            winning_numbers: HashSet::new(),
            my_numbers: HashSet::new(),
        }
    }

    fn add_winning(&mut self, n: u32) {
        self.winning_numbers.insert(n);
    }

    fn add_my(&mut self, n: u32) {
        self.my_numbers.insert(n);
    }

    fn winning_numbers(&mut self) -> Vec<u32> {
        self.winning_numbers
            .intersection(&self.my_numbers)
            .map(|c| *c)
            .collect()
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        //let mut cards: Vec<Card> = Vec::new();
        const NUMCARDS:usize = 187;
        let mut score_sum = 0;
        let mut card_counts: [u32; NUMCARDS] = [1; NUMCARDS];
        let base: u32 = 2;
        let mut c = 0;
        for line in lines {
            if let Ok(line_str) = line {
                let mut card = Card::new();
                for i in 0..10 {
                    let s = 10+(3*i);
                    let n = &line_str[s..=s+1];
                    card.add_winning(n.trim().parse::<u32>().expect("Bad num"));
                }
                for i in 0..25 {
                    let s = 42+(3*i);
                    let n = &line_str[s..=s+1];
                    card.add_my(n.trim().parse::<u32>().expect("Bad num"));
                }

                // Part 1
                let winners = card.winning_numbers();
                if winners.len() > 0 {
                    score_sum += base.pow(winners.len() as u32 - 1)
                }

                // Part 2
                // however many of this card I have...
                for _ in 0..card_counts[c] {
                    // Add following cards based on numbers of winners
                    for j in 1..=winners.len() {
                        card_counts[c + j] += 1;
                    }
                }
                println!("{}: {}: {:?}, New Score: {}", c, winners.len(), winners, score_sum);
                println!("{:?}", card_counts.iter().sum::<u32>());
                c += 1;
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// // Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
