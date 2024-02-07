use std::{
    collections::HashSet,
    fmt::Debug,
    fs,
    io::{self, Write},
};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(next_line_help = true)]
struct Cli {
    in_path: String,
    out_path: Option<String>,
    #[arg(long)]
    blacklist: Option<String>,
    #[arg(long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    let in_file = fs::File::open(&args.in_path).unwrap();
    let out_buffer = process_zip(in_file, args.blacklist, args.verbose);

    let out_path = args.out_path.unwrap_or(args.in_path);
    let mut out_file = fs::File::create(out_path).unwrap();
    out_file.write_all(&out_buffer).unwrap();
}

fn process_zip(file: fs::File, blacklist: Option<String>, verbose: bool) -> Vec<u8> {
    let verbose_log = |msg: String| {
        if verbose {
            println!("{}", msg);
        }
    };

    let mut in_zip = zip::ZipArchive::new(file).unwrap();

    let mut out_buffer = Vec::new();
    let mut zip_writer = zip::ZipWriter::new(io::Cursor::new(&mut out_buffer));

    let zero_timestamp = zip::DateTime::default(); // 1980.01.01 00:00:00
    let mut seen_names = HashSet::new();

    if verbose {
        verbose_log(format!(
            "stripping top-level zip comment: {}",
            String::from_utf8(in_zip.comment().to_vec()).unwrap()
        ));
    }

    let blacklist = blacklist
        .map(|x| glob::Pattern::new(&x).unwrap())
        .unwrap_or_default();

    for i in 0..in_zip.len() {
        let mut zip_entry = in_zip.by_index(i).unwrap();

        verbose_log(format!(
            "{} (method: {})",
            zip_entry.name(),
            zip_entry.compression(),
        ));

        if blacklist.matches(zip_entry.name()) {
            verbose_log(format!(
                "Ignoring blacklisted zip entry: {}",
                zip_entry.name()
            ));
            continue;
        }

        // ignore duplicates (most common in bad .jar files)
        if !seen_names.insert(zip_entry.name().to_string()) {
            verbose_log(format!(
                "Ignoring duplicate zip entry: {}",
                zip_entry.name()
            ));
            continue;
        }

        let default_perms = if zip_entry.is_dir() { 0o755 } else { 0o644 };

        let options = zip::write::FileOptions::default()
            .compression_method(zip_entry.compression()) // keep compression method
            .unix_permissions(zip_entry.unix_mode().unwrap_or(default_perms)) // keep or fix permissions
            .last_modified_time(zero_timestamp); // reset timestamp

        zip_writer.start_file(zip_entry.name(), options).unwrap();
        io::copy(&mut zip_entry, &mut zip_writer).unwrap();
    }
    drop(zip_writer);
    return out_buffer;
}
