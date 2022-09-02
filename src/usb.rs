use std::path::Path;
use std::collections::HashMap;
use std::fs;
use std::process::Command;

use zip::ZipArchive;
use crate::links::Links;
use crate::downloader::qdownload;
use crate::copy::copy_dir_all;

pub fn get_drives() -> HashMap<u8, String> {
    let snum = 65u32;
    let lnum = 91u32;
    let mut chars = Vec::new();
    for i in snum..lnum {
        let char = char::from_u32(i).unwrap();
        chars.push(char);
    }
    let strings = chars.iter().map(|x| x.to_string())
    .map(|x| x + ":").filter(|x| Path::new(x).exists()).collect::<Vec<_>>();
    let mut dict = HashMap::new();
    for i in 0..strings.len() {
        dict.insert(i as u8, strings[i].clone());
    }
    dict
}

/// NOTE: For some reason indicatif doesn't work in this func.
pub fn intall_to_drive(path: &Path, name: String, links: &Links) {
    let mut curdir = std::env::current_dir().unwrap();
    curdir.push(&name);
    for dir in fs::read_dir(&curdir).unwrap() {
        let dir = dir.unwrap().path();
        let mut rpath = path.to_path_buf();
        rpath.push("\\");
        rpath.push(dir.file_name().unwrap());
        let rpath = format!("{}", rpath.display());
        if dir.is_dir() {
            copy_dir_all(dir, rpath).unwrap();
        } else {
            let args = format!("{} {}", dir.display(), rpath);
            Command::new("cp").args(args.split(" ")).spawn().unwrap().wait().unwrap();
        }
    }
    std::env::set_current_dir(path).unwrap();
    for i in 0..links.programs.len() {
        let dir = std::env::current_dir().unwrap();
        let link = links.programs.get(i).unwrap();
        let rid = link.rfind("/").unwrap_or_default();
        let msg = String::from(&link[rid+1..]);
        qdownload(link, msg.clone());
        let file = fs::File::open(&msg).unwrap();
        let mut arch = ZipArchive::new(file).unwrap();
        arch.extract(dir).unwrap();
        drop(arch);
        std::fs::remove_file(msg).unwrap();
    }
    for file in std::fs::read_dir(path).unwrap() {
        let file = file.unwrap().path();
        if file.is_file() {
            std::fs::remove_file(file).unwrap();
        }
    }
    std::env::set_current_dir(std::env::current_exe().unwrap().parent().unwrap()).unwrap();
    std::fs::remove_dir_all(curdir).unwrap();
}