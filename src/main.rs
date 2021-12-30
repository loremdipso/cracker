use structopt::StructOpt;
use itertools::Itertools;
use multimap::MultiMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Unscramble the letters on a bike-style lock and see if it matches any common dictionary words
#[derive(StructOpt, Debug)]
struct Cli {
    /// The path to the file to read
    #[structopt(long = "dict", parse(from_os_str))]
    dict: std::path::PathBuf,

    /// The tumblers
    tumblers: Vec<String>,
}

fn main() {
	let args = Cli::from_args();
	let words = get_words_from_dict(args.dict);

	// we've built our dictionary, now let's iterate through all combinations of
	// our tumblers
	for combination in iterate(&args.tumblers) {
		let key = get_key(&combination);
		if let Some(results) = words.get_vec(&key) {
			println!("Combination: {}", combination);
			for result in results {
				println!("\tMatch: {}", result);
			}
		}
	}
}

fn iterate(tumblers: &Vec<String>) -> impl std::iter::Iterator<Item = String> + '_ {
    let mut tumbler_indexes: Vec<usize> = vec![0];
	let mut finished = false;

	while tumbler_indexes.len() < tumblers.len() {
		tumbler_indexes.push(0);
	}

    return std::iter::from_fn(move || {
		if tumblers.len() == 0 || tumbler_indexes[0] > tumblers[0].len() || finished {
			return None;
		}

		let mut s = String::new();
		for (i, index) in tumbler_indexes.iter().enumerate() {
			s.push(tumblers[i].chars().nth(*index).unwrap());
		}

		let mut i = tumbler_indexes.len() - 1;
		loop {
			tumbler_indexes[i] += 1;
			if tumbler_indexes[i] >= tumblers[i].len() {
				if i == 0 {
					finished = true;
					break;
				}
				tumbler_indexes[i] = 0;
			} else {
				break;
			}

			if i == 0 {
				break;
			}
			i -= 1;
		}

		return Some(s);
        // if num < n {
        //     result = Some(num);
        //     num += 1
        // } else {
        //     result = None
        // }
        // result
    });
}

fn get_words_from_dict<P>(filename: P) -> MultiMap<String, String>
where P: AsRef<Path> {
	let mut words = MultiMap::new();
	if let Ok(lines) = read_lines(filename) {
		for line in lines {
			if let Ok(line) = line {
				let line = line.trim();
				if line.len() > 0 {
					words.insert(get_key(line), line.to_string());
				}
			}
		}
	}
	return words;
}

fn get_key(s: &str) -> String {
	let s = &s.to_lowercase();
	let s = s.chars().sorted().rev().collect::<String>();
	return s.to_string();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
