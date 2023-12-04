use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;


fn main() {
    if let Ok(file_lines) = read_lines("input.txt") {
        const SIZE: usize = 140;
        let mut grid: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];
        let mut lnumber = 0;
        let mut cnumber = 0;

        let mut stars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
        // read file into grid
        for line in file_lines {
            match line {
                Ok(line_string) => {
                    for character in line_string.chars() {
                        grid[lnumber][cnumber] = character;
                        cnumber += 1;
                    }
                },
                Err(_) => panic!("Bad line")
            }
            lnumber += 1;
            cnumber = 0;
        }
        let mut part_number_sum = 0;
        // Look for numbers in grid
        for l in 0..SIZE {
            let mut c = 0;
            let mut number:u32 = 0;
            loop {
                if c >= SIZE {
                    break;
                }
                let character = grid[l][c];
                //println!("{}", c);
                match character {
                    '0'..='9' => {
                        let digit = character.to_digit(10).expect("what?");
                        number = (10 * number) + digit;
                        //println!("{} => {} ", digit, number);
                        //println!("{}", number);
                        if c == SIZE - 1 || ! grid[l][c+1].is_digit(10) {
                            // We have our number, now look around it
                            let mut is_part_number = false;
                            let digits = number.to_string().chars().count() as usize;
                            let cstart: usize = c as usize - (digits - 1);
                            if cstart > 0 {
                                is_part_number |= grid[l][cstart - 1] != '.';
                                if grid[l][cstart - 1] == '*' {
                                    stars.entry((l, cstart - 1))
                                        .and_modify(|v| v.push(number))
                                        .or_insert(vec!(number));
                                }
                            }
                            if c < SIZE - 1 {
                                is_part_number |= grid[l][c + 1] != '.';
                                if grid[l][c + 1] == '*' {
                                    stars.entry((l, c+1))
                                        .and_modify(|v| v.push(number))
                                        .or_insert(vec!(number));
                                }
                            }
                            let left = if cstart > 0 { cstart - 1 } else { 0 };
                            let right = if c < SIZE-1 { c + 1 } else { c };
                            if l > 0 {
                                is_part_number |= grid[l-1][left..=right].iter().any(|c| *c != '.');
                                for g in left..=right {
                                    if grid[l-1][g] == '*' {
                                        stars.entry((l-1, g))
                                            .and_modify(|v| v.push(number))
                                            .or_insert(vec!(number));
                                    }
                                }
                            }
                            if l < SIZE - 1 {
                                is_part_number |= grid[l+1][left..=right].iter().any(|c| *c != '.');
                                for g in left..=right {
                                    if grid[l+1][g] == '*' {
                                        stars.entry((l+1, g))
                                            .and_modify(|v| v.push(number))
                                            .or_insert(vec!(number));
                                    }
                                }
                            }
                            if is_part_number {
                                part_number_sum += number;
                                //println!("Line {}. {} is a part number. new total is {}", l, number, part_number_sum);
                            }
                        }
                    },
                    _ => {
                        number = 0;
                    }
                }
                c += 1;
            }
        }
        let mut gear_ratio_sum = 0;
        for (_loc, nums) in stars {
            if nums.len() == 2 {
                let product: u32 = nums.iter().product();
                gear_ratio_sum += product;
                //println!("Star {:?}, {:?} =  {} -> {}", _loc, nums, product, gear_ratio_sum);
            }
        }
        println!("Part Number Total: {}, Gear Ratio Total: {}", part_number_sum, gear_ratio_sum);
    }

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
    use super::get_calibration_value;

    #[test]
    fn test_thing() {
    }
}
