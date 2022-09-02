mod consts;
mod legend;
mod links;
mod downloader;
mod usb;
mod copy;

use indicatif::*;
use std::io::stdin;
use zip::ZipArchive;

fn main() {
    #[cfg(not(windows))] {
        panic!("wiiu_hb_installer is currently not supported on non-windows systems.");
    }
    let prog = ProgressBar::new(0);
    prog.set_style(ProgressStyle::default_bar()
    .progress_chars("#>-")
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
    .unwrap());
    print!("Welcome to the Wii U Homebrew installer! ");
    println!("Choose a homebrew enviroment to install by entering the number left of it.");
    legend::print_legend();
    legend::print_options();
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    // Stdin.read_line appends a line break to the String, so we have to get hacky...
    let mut num = buf.chars().collect::<Vec<_>>()
    .get(0).unwrap().to_string().parse::<u8>().unwrap();
    num -= 1;
    let links = links::getlinks();
    let link = consts::get_link_from_names(num, &links);
    let name = String::from(consts::NAMES[&num]);
    let msg = name.clone() + ".zip";
    downloader::download(link, msg.clone(), &prog, true);
    let file = std::fs::File::open(msg.clone()).unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    zip.extract(name.clone()).unwrap();
    drop(zip);
    std::fs::remove_file(msg).unwrap();
    let dict = usb::get_drives();
    println!("Choose a drive to install the files to by entering the number left of it.");
    for i in 0..dict.len() {
        let i = i as u8;
        let name = dict.get(&i).unwrap();
        println!("{}. {}", i, name);
    }
    buf.clear();
    stdin().read_line(&mut buf).unwrap();
    num = buf.chars().collect::<Vec<_>>()
    .get(0).unwrap().to_string().parse::<u8>().unwrap();
    let drive = std::path::Path::new(dict.get(&num).unwrap());
    usb::intall_to_drive(drive, name, &links);
    println!("Complete!");
}