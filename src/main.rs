use std::{
    io::{self, Read, Write},
    net::TcpStream
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(help = "Target IP/Domain")]
    target: String,
    #[arg(help = "Content", short, long)]
    content: String,
    #[arg(help = "Output File", short, long)]
    output: Option<String>,
    #[arg(help = "Force", short, long)]
    force: bool,
}

fn normalize_text(content: String) -> String {
    return content.replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
        .replace("\\'", "'");
}

fn print_content(content: Vec<u8>, lossy: bool) {
    if lossy {
        println!("{}", String::from_utf8_lossy(&content));
        return;
    }
    match String::from_utf8(content) {
        Ok(s) => println!("{}", s),
        Err(_e) => println!("Content is not valid UTF-8. Use --force to print it anyway or save it to a file.")
    }
}

fn get_user_input(prompt: String) -> String {
    print!("{}: ", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return input;
}

fn write_to_file(content: Vec<u8>, file_path: String, create: bool) -> std::io::Result<()> {
    if create {
        let mut file = std::fs::File::create(file_path)?;
        return file.write_all(&content);
    } else {
        let mut file = std::fs::OpenOptions::new().append(true).open(file_path)?;
        return file.write_all(&content);
    }
}

fn main() -> std::io::Result<()> {
    // Parse Arguments
    let args = Args::parse();
    // Normalize Content (Replace escape sequences)
    let normalized_content = normalize_text(args.content.clone());
    
    // Connect
    let mut stream = match TcpStream::connect(args.target) {
        Ok(stream) => stream,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::ConnectionRefused {
                println!("Error: Connection refused");
                return Ok(());
            } else if e.kind() == std::io::ErrorKind::TimedOut {
                println!("Error: Connection timed out");
                return Ok(());
            } else {
                return Err(e);
            }
        }
    };

    // Request and shutdown write
    stream.write_all(normalized_content.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    // Read response
    let mut content = Vec::<u8>::new();
    stream.read_to_end(&mut content)?;

    // Print content
    if !args.output.is_some() {
        print_content(content, args.force);
        return Ok(());
    }

    // Save content to file
    let file_path = args.output.unwrap();
    if std::path::Path::new(&file_path).exists() {
        if args.force {
            let _ = write_to_file(content, file_path, true);
            return Ok(());
        }
        let inp = get_user_input("File already exists! (O)verride/(A)ppend/(C)ancel".to_string());

        match inp.trim().to_uppercase().as_str() {
            "O" => { let _ = write_to_file(content, file_path, true); }
            "A" => { let _ = write_to_file(content, file_path, false); } 
            _ => { println!("Cancelled"); }
        }
        
    } else { write_to_file(content, file_path, true)?; } // Create

    return Ok(());
}