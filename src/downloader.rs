use thread_control::make_pair;
use indicatif::*;
use reqwest::blocking;
use reqwest::IntoUrl;
use std::thread;
use std::io;
use std::fs::OpenOptions;

pub fn download<L: IntoUrl + Clone>(link: L, msg: String, prog: &ProgressBar, last: bool) {
    let mut req = blocking::get(link.clone()).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).read(true).open(&msg).unwrap();
    let len = req.content_length().unwrap();
    prog.set_length(len);
    prog.tick();
    let (f, c) = make_pair();
    let h = thread::spawn(move || {
        while f.alive() {
            io::copy(&mut req, &mut file).unwrap();
            break;
        }
    });
    // It's likely a byte or two has been written at this point.
    let meta = std::fs::metadata;
    while !c.is_done() {
        prog.set_position(meta(&msg).unwrap().len());
    }
    prog.set_position(prog.length().unwrap());
    if last {
        prog.finish();
    }
    h.join().unwrap();
}

pub fn qdownload<L: IntoUrl + Clone>(link: L, msg: String) {
    let mut req = blocking::get(link).unwrap();
    let mut file = OpenOptions::new().write(true).create(true).read(true).open(&msg).unwrap();
    io::copy(&mut req, &mut file).unwrap();
}