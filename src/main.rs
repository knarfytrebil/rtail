#[macro_use]
extern crate clap;

use std::sync::mpsc::{channel, Sender, Receiver};
use std::{thread, str, time};

use clap::App;
use curl::easy::Easy;


fn main() {
    let conf = load_yaml!("commands.yml");
    let matches = App::from_yaml(conf).get_matches();

    let mut last_pos = 0;
    let interval = matches.value_of("milliseconds").unwrap_or("1000").parse::<u64>().unwrap();

    if let Some(values) = matches.values_of("INPUT") {
        let url: Vec<String> = values
            .into_iter()
            .map(|v| { v.to_string() })
            .collect();

        let (tx, rx) = channel::<usize>();

        loop {
            thread::spawn(move || { fetch_url(&url[0], tx.clone()); });
            last_pos = process_resp(rx.clone());
            let duration = time::Duration::from_millis(interval);
            thread::sleep(duration);
        }

        println!("[ INFO]: End of Data, total length: {}", last_pos);
    }
}

fn process_resp(receiver: Receiver<usize>) -> usize {
    let mut total_length = 0;
    loop {
        match receiver.recv() {
            Ok(length) => {
                if length != 0 {
                    total_length += length; 
                } else {
                    break;
                }
            }
            Err(e) => {
                panic!("[ERROR]: {}", e);
            }
        }
    }
    total_length
}

fn fetch_url(url_str: &str, sender: Sender<usize>) {
    let mut easy = Easy::new();
    easy.url(url_str).unwrap();

    let count_tx = sender.clone();
    easy.write_function(move |buf| {
        let s = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        // stdout().write_all(s).unwrap();
        println!("{}", s);
        let _ = count_tx.send(s.chars().count());
        Ok(buf.len())
    }).unwrap();

    let req_tx = sender.clone();
    match easy.perform() {
        Ok(_) => {
            let _ = req_tx.send(0);
        }
        Err(e) => {
            println!("[ERROR]: {}", e);
        }
    }
}
