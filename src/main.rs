use std::{collections::HashSet, env, fs, io, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 || args.len() >= 4 {
        println!("Usage: stripzip-rs <in_path> [out_path]");
        process::exit(1);
    }
    let in_path = &args[1];
    let out_path = &args[args.len() - 1]; // same as in_path if omitted

    let in_zip_data = fs::read(in_path).unwrap();
    let cursor = io::Cursor::new(in_zip_data);
    let mut in_zip = zip::ZipArchive::new(cursor).unwrap();

    let out_file = fs::File::create(out_path).unwrap();
    let mut out_zip = zip::ZipWriter::new(out_file);

    let zero_timestamp = zip::DateTime::default(); // 1980.01.01 00:00:00
    let mut seen_names = HashSet::new();

    for i in 0..in_zip.len()  {
        let mut zip_entry = in_zip.by_index(i).unwrap();

        // ignore duplicates (most common in bad .jar files)
        if !seen_names.insert(zip_entry.name().to_string()) {
            println!("Ignoring duplicate zip entry: {}", zip_entry.name());
            continue;
        }

        let default_perms = if zip_entry.is_dir() { 0o755 } else { 0o644 };

        let options = zip::write::FileOptions::default()
            .compression_method(zip_entry.compression()) // keep compression method
            .unix_permissions(zip_entry.unix_mode().unwrap_or(default_perms)) // keep or fix permissions
            .last_modified_time(zero_timestamp); // reset timestamp

        out_zip.start_file(zip_entry.name(), options).unwrap();
        io::copy(&mut zip_entry, &mut out_zip).unwrap();
    }
}
