use std::fs::File;
use std::io::{BufRead, BufReader};

type DepthMeasurement = isize;

fn input() -> Vec<DepthMeasurement> {
    let file = File::open("input.txt").unwrap();

    let mut out: Vec<DepthMeasurement> = Vec::new();

    for line in BufReader::new(file).lines() {
        out.push(line.unwrap().parse().unwrap())
    }

    out
}

fn print_change(reading: isize, last: isize) {
    if reading != last {
        println!("{}: change of {}", reading, reading - last);
    } else {
        println!("{}: no difference", reading);
    }
}

fn main() {
    let readings = input();

    let mut increases = 0;
    let mut decreases = 0;
    let mut last_reading: Option<isize> = None;

    for reading in readings {
        if let Some(last) = last_reading {
            if reading > last {
                print_change(reading, last);
                increases = increases + 1;
            } else if reading < last {
                print_change(reading, last);
                decreases = decreases + 1;
            } else {
                print_change(reading, last);
            }
        } else {
            println!("no last value");
        }

        last_reading = Some(reading);
    }

    println!("inceases: {}", increases);
    println!("decreases: {}", decreases);
}
