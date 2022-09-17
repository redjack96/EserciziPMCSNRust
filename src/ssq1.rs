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
    last_departure: f64, // Es. 1.2.2
}

impl Adder {
    fn throughput(&self, n: i64) -> f64 {
        n as f64 / self.last_departure
    }
    fn avg_interarrival(&self, index: i64) -> f64 {
        self.interarrival / index as f64
    }
    fn avg_service(&self, index: i64) -> f64 {
        self.service / index as f64
    }
    fn avg_delay(&self, index: i64) -> f64 {
        self.delay / index as f64
    }
    fn avg_wait(&self, index: i64) -> f64 {
        self.wait / index as f64
    }
    // Es. 1.2.2
    fn avg_population(&self, n: i64) -> f64 {
        self.avg_wait(n) * self.throughput(n)
    }
    fn avg_queue_population(&self, n: i64) -> f64 {
        self.avg_delay(n) * self.throughput(n)
    }
    fn avg_service_population(&self, n: i64) -> f64 {
        self.avg_service(n) * self.throughput(n)
    }
    fn traffic_intensity(&self, n: i64) -> f64 {
        self.avg_service(n) / self.avg_interarrival(n)
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
    doubles.first().cloned().map(|d| d*1.2)
}

fn get_service(doubles: &Vec<f64>) -> Option<f64> {
    // doubles.last().cloned()
    Some(7.124851)
}

// Esegui con: cargo run --bin ssq1
fn main() {
    let mut index = 0i64;
    let mut arrival = START;
    let mut delay;
    let mut service;
    let mut wait;
    let mut departure = START;

    let mut adder = Adder::default();

    // Es. 1.2.3
    let mut max_delay = 0.0;
    let mut job_delayed : i64 = 0;

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
            job_delayed += 1;
            if delay > max_delay {
                max_delay = delay;
            }
        } else {
            delay = 0.0;
        }

        service = get_service(&vec).unwrap_or(0.0);
        wait = delay + service;
        departure = arrival + wait;
        adder.last_departure = departure; // c_i
        adder.delay += delay;
        adder.wait += wait;
        adder.service += service;
        index += 1;
    }
    adder.interarrival = arrival - START;

    println!("Per {} jobs", index);
    println!("\tTempo di interarrivo medio (r) = {}", adder.avg_interarrival(index));
    println!("\tTempo di servizio medio    (s) = {}", adder.avg_service(index));
    println!("\tTempo di attesa medio      (w) = {}", adder.avg_delay(index));
    println!("\tTempo di risposta medio    (d) = {}", adder.avg_wait(index));
    // Esercizio 1.2.2 pag 25. Aggiungi l, q, x
    println!("\tPopolazione media del nodo    (l) = {}", adder.avg_population(index));
    println!("\tPopolazione media della coda  (q) = {}", adder.avg_queue_population(index));
    println!("\tPopolazione media in servizio (x) = {} (utilizzazione)", adder.avg_service_population(index));
    println!("\tThroughput                    (X) = {}", adder.throughput(index));
    println!("\tIntensità di traffico       (s/r) = {}", adder.traffic_intensity(index));
    println!("\tIstante di completamento    (c_n) = {}", adder.last_departure);
    // Esercizio 1.2.3. Max d, # job al tempo t, percentuale di job che attendono.
    println!("\tMassima attesa in coda    (max d) = {}", max_delay);
    println!("\tProporzione di job che attendono  = {} %", job_delayed as f64 / index as f64 * 100.0);
}
