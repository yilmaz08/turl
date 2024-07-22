use std::io::Write;

pub fn write_to_file(content: Vec<u8>, file_path: String, create: bool) -> std::io::Result<()> {
    if create {
        let mut file = std::fs::File::create(file_path)?;
        return file.write_all(&content);
    } else {
        let mut file = std::fs::OpenOptions::new().append(true).open(file_path)?;
        return file.write_all(&content);
    }
}

pub fn read_from_file(file_path: String) -> std::io::Result<String> {
    if !std::path::Path::new(&file_path).exists() {
        println!("Error: Request content file not found");
        std::process::exit(1);
    }
    let content = std::fs::read_to_string(file_path)?;
    return Ok(content);
}