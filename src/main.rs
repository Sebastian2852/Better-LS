use std::{ fs::{ self, DirEntry }, path::{ Path, PathBuf } };
use chrono::{ DateTime, Utc };
use clap::Parser;
use serde::Serialize;
use strum::Display;
use owo_colors::{ OwoColorize };
use tabled::{ settings::{ object::{ Columns, Rows }, Color, Style }, Table, Tabled };

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Replacment for ls command",
    long_about = "An awesome replacment for the ls command!"
)]
struct Cli {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

#[derive(Debug, Display, Serialize)]
enum FileType {
    File,
    Dir,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled{rename="Name"}]
    name: String,
    #[tabled{rename="Size (Bytes)"}]
    len_bytes: u64,
    #[tabled{rename="Last Modified"}]
    last_modified: String,
    #[tabled{rename="Type"}]
    file_type: FileType,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.path.unwrap_or(PathBuf::from("."));

    if let Ok(does_exists) = fs::exists(&path) {
        if does_exists {
            if cli.json {
                let get_files = get_files(&path);
                println!("{}", serde_json::to_string(&get_files).unwrap_or("Cannot parse JSON".to_string()))
            } else {
                print_table(path);
            }
        } else { println!("{}", "Path does not exist".red()) }
    } else {
        println!("{}", "Error reading directory (missing permissions?)".yellow())
    }
}

fn print_table(path: PathBuf) {
    let get_files = get_files(&path);
    let mut table = Table::new(get_files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_BRIGHT_CYAN);
    table.modify(Columns::one(1), Color::FG_BRIGHT_MAGENTA);
    table.modify(Columns::one(2), Color::FG_BRIGHT_YELLOW);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut data = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for entry in read_dir {
            if let Ok(file) = entry {
                data.push(map_data(file));
            }
        }
    }

    return data;
}

fn map_data(file: DirEntry) -> FileEntry {
    if let Ok(meta) = fs::metadata(&file.path()) {
        return FileEntry {
            name: file.file_name().into_string().unwrap_or("UNKOWN".red().to_string().into()),
            len_bytes: meta.len(),
            last_modified: if let Ok(mmod) = meta.modified() {
                let date: DateTime<Utc> = mmod.into();
                format!("{}", date.format("%b %e %Y (%a)"))
            } else {
                String::default()
            },
            file_type: if meta.is_dir() {
                FileType::Dir
            } else {
                FileType::File
            },
        };
    } else {
        return FileEntry {
            name: "UNKOWN".to_string(),
            len_bytes: 0,
            last_modified: "".to_string(),
            file_type: FileType::File,
        };
    }
}
