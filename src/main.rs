use std::env;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

fn ending(file_name: &String) -> String {
    let chars = file_name.chars().rev();
    let mut name = String::new();

    for c in chars {
        name.push(c);
        if c == '.' {
            break;
        }
    }

    name.chars().rev().collect::<String>()
}

// Example file name: Kingsman.The.Secret.Service.2014.UNCUT.1080p.BluRay.x265-RARBG.mp4
fn main() {
    let opt = Opt::from_args();

    let mut renamed_files: Vec<String> = Vec::new();
    let mut rename_counter: usize = 0;

    #[allow(unused_mut)]
    let mut entries = fs::read_dir(".").unwrap();

    for i in entries.map(|e| e.unwrap()).into_iter() {
        let current_name = i.file_name().into_string().unwrap();

        // If whitelist is enabled and the following iteration does not match the
        // whitelist, skip
        if let Some(whitelist) = &opt.whitelist {
            if !current_name.contains(whitelist) {
                continue;
            }
        }

        // If blacklist is enabled and the following iteration matches the
        // blacklist, skip
        if let Some(blacklist) = &opt.blacklist {
            if current_name.contains(blacklist) {
                continue;
            }
        }

        // If file does not match given resolution, skip
        if !current_name.contains(&opt.resolution) {
            continue;
        }

        let mut name_to_be = current_name.clone();
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);
        name_to_be = name_to_be
            .split_once(&opt.resolution)
            .unwrap()
            .0
            .to_string();
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);

        // Remove last character, which will be a dot
        // Check if there is any dots first, if not, the program have probably already ran
        if name_to_be.contains('.') {
            name_to_be.remove(&name_to_be.len() - 1);
        }
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);

        // Add ꞉ before the episode numbering
        if opt.season_before_res {
            // Counts
            let mut dots: isize = name_to_be.chars().filter(|x| x == &'.').count() as isize - 1;
            dots = if dots == 0 { std::isize::MAX } else { dots };
            let mut counter: usize = 0;

            let soon_name_to_be: String = name_to_be
                .chars()
                .map(|x| {
                    if x == '.' {
                        if counter as isize == dots {
                            return ':';
                        }
                        counter += 1;
                    }
                    x
                })
                .collect::<String>();

            name_to_be = soon_name_to_be.replace(":", "꞉ "); // This is not a regular colon
        }
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);
        name_to_be = name_to_be.replace(".", " ");
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);
        // Remove any trailing space that might be
        if name_to_be.ends_with(' ') {
            name_to_be.pop();
        }
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);

        // Set file ending if it is a file
        if let Ok(file_type) = i.file_type() {
            if file_type.is_file() {
                let end = ending(&current_name);
                name_to_be.extend(end.chars());
            }
        }
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);

        // Get working dir
        let here = match env::current_dir() {
            Ok(k) => k,
            Err(e) => {
                println!("Could not get current working directory {:?}", e);
                std::process::exit(1)
            }
        };
        let mut current = PathBuf::from(&here);
        let mut to = PathBuf::from(&here);

        current.push(&current_name);
        to.push(&name_to_be);

        if !opt.dry_mode {
            match fs::rename(&current, &to) {
                Ok(k) => k,
                Err(e) => {
                    println!("could not rename {}, \n{:?}", &current_name, &e)
                }
            };
        }
        println!("name_to_be: {}, current_name: {}",&name_to_be, &current_name);

        rename_counter += 1;
        renamed_files.push(name_to_be);
    }
    print_success(rename_counter, renamed_files, opt.dry_mode);
}

fn print_success(rename_counter: usize, renamed_files: Vec<String>, dry_mode: bool) {
    // Print amount of changed files for satisfaction
    if &rename_counter == &1 {
        println!(
            "{} {}",
            if dry_mode {
                "Would had renamed"
            } else {
                "renamed"
            },
            renamed_files[0]
        );
    } else if rename_counter == 0 {
        // print different depending on if it is debug or not
        println!(
            "{} rename any files",
            if dry_mode { "Wouldn't" } else { "Didn't" }
        );
    } else {
        // print different depending on if it is debug or not
        let debug_print: [&str; 2] = if dry_mode {
            ["Would had renamed", "is"]
        } else {
            ["Renamed", "was"]
        };

        println!("{} following files", &debug_print[0]);
        for i in renamed_files {
            println!("{}", &i);
        }
        println!("which {} {} files", &debug_print[1], &rename_counter);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "rename_here")]
struct Opt {
    // A flag, true if used in the command line. Note doc comment will
    // be used for the help message of the flag. The name of the
    // argument will be, by default, based on the name of the field.
    /// Whats the resolution in the file-name
    #[structopt(short, long)]
    resolution: String,

    /// Is season before_res
    #[structopt(short, long)]
    season_before_res: bool,

    /// Do not rename anything
    #[structopt(short, long)]
    dry_mode: bool,

    /// Whitelist entries
    #[structopt(short, long)]
    whitelist: Option<String>,

    /// Blacklist entries
    #[structopt(short, long)]
    blacklist: Option<String>,
}
