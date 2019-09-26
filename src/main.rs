#[macro_use]
extern crate clap;
extern crate ctrlc;

use std::sync::mpsc::{channel, Receiver};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::{thread, str, time, process};

use clap::App;
use curl::easy::Easy;


fn main() {
    let conf = load_yaml!("commands.yml");
    let matches = App::from_yaml(conf).get_matches();
    let running = Arc::new(AtomicUsize::new(0)); 
    let r = running.clone();

    ctrlc::set_handler(move || {
        let prev = r.fetch_add(1, Ordering::SeqCst);
        if prev == 0 {
            println!("Exiting ...");
        } else {
            process::exit(0);
        }
    }).expect("Error setting ctrl-c handler");

    let interval = matches
        .value_of("milliseconds")
        .unwrap_or("1000")
        .parse::<u64>()
        .unwrap();

    if let Some(values) = matches.values_of("INPUT") {
        let url: Vec<String> = values
            .into_iter()
            .map(|v| { v.to_string() })
            .collect();
        fetch_url(&url[0], interval, running, 0); 
    }
}

fn process_resp(receiver: Receiver<String>, last_pos: usize) -> usize {
    let total_length = Arc::new(AtomicUsize::new(0)); 
    loop {
        match receiver.recv() {
            Ok(s) => {
                let length = s.chars().count();
                if length != 0 {
                    let _prev = total_length.fetch_add(length, Ordering::SeqCst);
                    let t = total_length.load(Ordering::SeqCst);
                    if t > last_pos {
                        print!("{}", s);
                        println!("{}, {}", t, last_pos); 
                    }
                } else {
                    break;
                }
            }
            Err(e) => {
                panic!("[ERROR]: {}", e);
            }
        }
    }
    total_length.load(Ordering::SeqCst)
}

fn fetch_url(url_str: &str, interval: u64, running: Arc<AtomicUsize>, mut last_pos: usize) {
    let (sender, rx) = channel::<String>();
    let duration = time::Duration::from_millis(interval);
    let mut easy = Easy::new();
    easy.url(url_str).unwrap();

    let count_tx = sender.clone();
    easy.write_function(move |buf| {
        let s = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        }.to_string();
        let _ = count_tx.send(s); 
        Ok(buf.len())
    }).unwrap();

    let req_tx = sender.clone();
    match easy.perform() {
        Ok(_) => {
            let _ = req_tx.send("".to_string());
        }
        Err(e) => {
            println!("[ERROR]: {}", e);
        }
    }

    last_pos = process_resp(rx, last_pos);
    thread::sleep(duration);
    println!("[ INFO]: End of Response, total length: {}", last_pos);

    if running.load(Ordering::SeqCst) <= 0 {
        fetch_url(url_str, interval, running, last_pos);
    }
}
