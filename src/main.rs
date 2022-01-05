use std::env;
use std::fs;
use std::fs::ReadDir;
use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let opt = Opt::from_args();
    let resolution = opt.resolution;
    let season_before_res = opt.season_before_res;
    let dry_mode = opt.dry_mode;
    //let resolution = "1080p".to_string();
    //let season_before_res = true;
    //let dry_mode = false;

    let mut renamed_files: Vec<String> = Vec::new();
    let mut rename_counter: usize = 0;

    #[allow(unused_mut)]
    let mut entries = fs::read_dir(".").unwrap();

    for i in entries.map(|e| e.unwrap()).into_iter() {
        let current_name = i.file_name().into_string().unwrap();

        // If file does not match given resolution, keep on going
        if !current_name.contains(&resolution) {
            //println!("skip: {}", &current_name);
            continue;
        }

        println!("original filename: '{}'", &current_name);
        let mut name_to_be = current_name.clone();
        name_to_be = name_to_be.split_once(&resolution).unwrap().0.to_string();

        // Remove last character, which will be a dot
        let name_to_be: String = name_to_be.chars().take(&name_to_be.len() - 1).collect();

        let dots = name_to_be.chars().filter(|x| x == &'.').count() - 1;
        println!("dots: {}", &dots);

        let mut counter: usize = 0;
        let mut name_to_be: String = name_to_be
            .chars()
            .map(|x| {
                if x == '.' {
                    if counter == dots {
                        return ':';
                    }
                    counter += 1;
                }
                x
            })
            .collect::<String>();

        // If season is before resolution
        if season_before_res {
            name_to_be = name_to_be.replace(":", "꞉ "); // This is not a regular colon
        } else {
            name_to_be = name_to_be.replace(":", "꞉"); // This is not a regular colon
        }
        name_to_be = name_to_be.replace(".", " ");

        println!("'{}'", &name_to_be);

        let here = env::current_dir().unwrap();
        let mut current = PathBuf::from(&here);
        let mut to = PathBuf::from(&here);
        current.push(&current_name);
        to.push(&name_to_be.replace(" ", " "));

        if !dry_mode {
            match fs::rename(&current, &to) {
                Ok(k) => k,
                Err(e) => {
                    println!("could not rename {}, \n{:?}", &current_name, &e)
                }
            };
        }

        rename_counter += 1;
        renamed_files.push(name_to_be);
    }

    // Print amount of changed files for satisfaction
    if &rename_counter == &1 {
        println!("renamed {}", renamed_files[0]);
    } else {
        println!("Renamed following files");
        for i in renamed_files {
            println!("{}", &i);
        }
        println!("which was {} files", &rename_counter);
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "episodeat")]
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
}
