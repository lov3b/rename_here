use std::env;
use std::fs;
use std::io;

fn main() {
    println!("Hello, world!");
    let here = env::current_dir().unwrap();

    let files = fs::read_dir(&here).unwrap();

    let mut entries = fs::read_dir(".")
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()
        .unwrap();

    // Entries here
    for i in entries.iter() {
        println!("{:?}", &i);
    }

    for i in entries.iter() {
        println!("{:?}", &i);

        let tmp = i.to_str().unwrap().replace("./", "").replace(".", " ");
        let resolution = if tmp.contains("1080p") {
            "1080p"
        } else if tmp.contains("720p") {
            "720p"
        } else if tmp.contains("2160p") {
            "2160p"
        } else if tmp.contains("2700p") {
            "2700p"
        } else if tmp.contains(".20") {
            ".20"
        } else if tmp.contains(".19") {
            ".19"
        } else {
            println!("Could not find resulotion: {}", &tmp);
            std::process::exit(0);
        };
    }
}
