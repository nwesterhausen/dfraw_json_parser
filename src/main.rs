use regex::Regex;
use serde_json::to_string;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod creature;

enum RawObjectKind {
    Creature,
    None,
}

fn main() {
    let re = Regex::new(r"(\[(?P<key>[^\[:]+):?(?P<value>[^\]\[]*)])").unwrap();

    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        println!("Usage: {} raw_file(s)", args[0]);
        return;
    }

    let n_args = args.len();
    for x in 1..n_args {
        let input_path = ::std::env::args().nth(x).unwrap();
        let file = File::open(&input_path).unwrap();
        let reader = BufReader::new(file);

        let mut creatures = 0;
        let mut raw_filename = String::new();
        let mut current_object = RawObjectKind::None;
        let mut creature_temp = creature::Creature::new("None", "None");

        for (index, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            if index == 0 {
                raw_filename = String::from(&line);
                continue;
            }
            for cap in re.captures_iter(&line) {
                // println!("Key: {} Value: {}", &cap[2], &cap[3])
                if &cap[2] == "CREATURE" {
                    // We are starting a creature object capture
                    creatures += 1;
                    match current_object {
                        RawObjectKind::Creature => {
                            // If we already *were* capturing a creature, export it.
                            // Reset the temp values !!Todo
                            //println!("{:#?}", creature_temp);
                            println!("{},", to_string(&creature_temp).unwrap());
                        }
                        RawObjectKind::None => (),
                    }
                    current_object = RawObjectKind::Creature;
                    creature_temp = creature::Creature::new(&raw_filename, &cap[3]);
                    continue;
                }
                if &cap[2] == "NAME" {
                    creature_temp.name = String::from(&cap[3]);
                    continue;
                }
                if &cap[2] == "EGG_SIZE" {
                    creature_temp.lays_eggs = true;
                    creature_temp.egg_size = cap[3].parse().expect("EGG_SIZE should be an integer");
                    continue;
                }
                if &cap[2] == "CLUTCH_SIZE" {
                    creature_temp.lays_eggs = true;
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    creature_temp.clutch_size[0] = split[0]
                        .parse()
                        .expect("CLUTCH_SIZE min should be an integer");
                    creature_temp.clutch_size[1] = split[1]
                        .parse()
                        .expect("CLUTCH_SIZE max should be an integer");
                    continue;
                }
                if &cap[2] == "DESCRIPTION" {
                    creature_temp.description = String::from(&cap[3]);
                    continue;
                }
                if &cap[2] == "MAXAGE" {
                    let split = cap[3].split(":").collect::<Vec<&str>>();
                    creature_temp.max_age[0] =
                        split[0].parse().expect("MAXAGE min should be an integer");
                    creature_temp.max_age[1] =
                        split[1].parse().expect("MAXAGE max should be an integer");
                    continue;
                }
            }
        }
        match current_object {
            RawObjectKind::Creature => {
                // If we already *were* capturing a creature, export it.
                // println!("Finished capturing creature, now finished");
                // Reset the temp values !!Todo
                //println!("{:#?}", creature_temp);
                println!("{}", to_string(&creature_temp).unwrap());
            }
            RawObjectKind::None => (),
        }
        // println!("{} creatures defined in {}", creatures, &raw_filename);
    }
}
