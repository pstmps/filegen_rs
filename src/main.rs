#[macro_use]
extern crate log;

use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use clap::Parser;
use csv::Writer;
use rand::Rng;
use rand::distributions::Alphanumeric;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// root path for file generation
    #[arg(short, long)]
    root_path: String,

    /// Number of files to generate
    #[arg(short, long)]
    number_of_files: u64,

    /// size for random files in bytes 
    #[arg(short, long, default_value_t = 10)]
    filesize: u64,

    /// Number of subfolders generated
    #[arg(short, long, default_value_t = 1)]
    subfolders: u64,

    /// name of csv output file
    #[arg(short, long, default_value = "output.csv")]
    outfile: String,

    /// purge random files after generation default is false
    #[arg(short, long, default_value_t = false)]
    purge: bool,
}

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    std::iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(length)
        .collect()
}

fn create_random_file(folder_name: &str, random_string_length: usize, filesize: usize, wtr: &Arc<Mutex<Writer<File>>>) -> Result<(), Error> {
    let file_name = format!("{}/{}", folder_name, generate_random_string(random_string_length));
    // create file
    let mut file = match std::fs::File::create(&file_name) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    // write random bytes to file
    let mut bytes = vec![0u8; filesize];
    rand::thread_rng().fill(&mut bytes[..]);
    file.write_all(&bytes)?;

    // check if mutex is poisoned
    let mut wtr = match wtr.lock() {
        Ok(guard) => guard,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Mutex lock poisoned")),
    };
    // write to csv
    if let Err(e) = wtr.write_record([&file_name, &filesize.to_string()]) {
        return Err(e.into());
    }

    wtr.flush()?;

    Ok(())
}

fn purge_directory(path: &String) -> std::io::Result<()> {
    match fs::read_dir(path) {
        Ok(entries) => {
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        if path.is_dir() {
                            if let Err(e) = fs::remove_dir_all(&path) {
                                error!("Failed to remove directory {}: {}", path.display(), e);
                            }
                        }
                    },
                    Err(e) => error!("Failed to read directory entry: {}", e),
                }
            }
        },
        Err(e) => error!("Failed to read directory {}: {}", path, e),
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize logger
    env_logger::init();

    let args = Args::parse();
    info!("Args: {:?}", args);

    // csv writer
    let wtr = Writer::from_path(args.outfile)?;
    let wtr = Arc::new(Mutex::new(wtr));

    // counters for files and directories
    let file_counter = Arc::new(AtomicUsize::new(0));
    let directory_counter = Arc::new(AtomicUsize::new(0));

    // clone variable for threading
    let directory_counter = Arc::clone(&directory_counter);

    // loop via rayon thread pool
    (0..args.subfolders).into_par_iter().for_each(|_i| {
        let folder_name = format!("{}/{}", args.root_path, generate_random_string(16));
        
        match std::fs::create_dir_all(&folder_name) {
            Ok(_) => {directory_counter.fetch_add(1, Ordering::SeqCst);},
            Err(e) => error!("Failed to create directory: {}", e),
        }
   
        // clone variables for threading
        let wtr = Arc::clone(&wtr);
        let file_counter = Arc::clone(&file_counter);

        (0..args.number_of_files).into_par_iter().for_each(|_j| {
            match create_random_file(&folder_name, 32, args.filesize as usize, &wtr) {
                Ok(_) => {file_counter.fetch_add(1, Ordering::SeqCst);},
                Err(e) => error!("Failed to create file: {}", e),
            }
        });
    });


    let final_directory_count = directory_counter.load(Ordering::SeqCst);
    info!("Created {} subdirectories", final_directory_count);
    if final_directory_count != args.subfolders.try_into().unwrap() {
        warn!("Warning: Created directory count does not match the expected number: {}", args.subfolders);
    }

    let final_count = file_counter.load(Ordering::SeqCst);
    let expected_count = (args.number_of_files * args.subfolders).try_into()?;

    info!("Created {} files", final_count);
    if final_count != expected_count {
        warn!("Warning: Created file count does not match the expected number: {}", expected_count);
    }

    if args.purge {
        purge_directory(&args.root_path)?;
    }
    Ok(())
}
