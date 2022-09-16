/**
 Servente M/M/1/FIFO Trace-driven
 codice ripreso da ssq1.c
 **/
use std::fs;
use std::path::Path;

const START: f64 = 0.0;
const FILENAME: &str = "ssq1.dat";

#[derive(Default)]
struct Adder {
    service: f64,
    delay: f64,
    wait: f64,
    interarrival: f64,
}

impl Adder {
    fn avg_interarrival(&self, index: i64) -> f64 {
        self.interarrival / index as f64
    }
    pub fn avg_service(&self, index: i64) -> f64 {
        self.service / index as f64
    }
    pub fn avg_delay(&self, index: i64) -> f64 {
        self.delay / index as f64
    }
    pub fn avg_wait(&self, index: i64) -> f64 {
        self.wait / index as f64
    }
}


fn parse_doubles(line: &str) -> Vec<f64> {
    line.trim()
        .split_whitespace()
        .map(|n| {
            n.parse::<f64>().unwrap()
        })
        .collect::<Vec<f64>>()
}

fn get_arrival(doubles: &Vec<f64>) -> Option<f64> {
    doubles.first().cloned()
}

fn get_service(doubles: &Vec<f64>) -> Option<f64> {
    doubles.last().cloned()
}

// Esegui con: cargo run --bin ssq1
fn main() {
    let mut index = 0i64;
    let mut arrival = START;
    let mut delay ;
    let mut service;
    let mut wait ;
    let mut departure = START;

    let mut adder = Adder::default();


    println!("Hello, world!");
    // TRACE DRIVEN_ prende i dati da un file
    let file = fs::read_to_string(Path::new(FILENAME))
        .expect("Impossibile leggere il file");

    for (_, line) in file.lines().enumerate() {
        let vec = parse_doubles(&line);
        arrival = get_arrival(&vec).unwrap_or(0.0);
        // Non c'è bisogno di una coda
        if arrival < departure {
            delay = departure - arrival;
        } else {
            delay = 0.0;
        }

        service = get_service(&vec).unwrap_or(0.0);
        wait = delay + service;
        departure = arrival + wait;

        adder.delay += delay;
        adder.wait += wait;
        adder.service += service;
        index += 1;
    }
    adder.interarrival = arrival - START;

    println!("Per {} jobs", index);
    println!("\tTempo di interarrivo medio = {}", adder.avg_interarrival(index));
    println!("\tTempo di servizio medio    = {}", adder.avg_service(index));
    println!("\tTempo di attesa medio      = {}", adder.avg_delay(index));
    println!("\tTempo di risposta medio    = {}", adder.avg_wait(index));
}
