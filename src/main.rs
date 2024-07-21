use std::{
    io::{self, Read, Write},
    net::TcpStream
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(help = "Target IP/Domain", short, long, default_value = "127.0.0.1")]
    target: String,
    #[arg(help = "Target Port", short, long, default_value = "80")]
    port: u16,
    #[arg(help = "Content", short, long)]
    content: String,
    #[arg(help = "Lossy UTF-8 Conversion", long)]
    lossy: bool,
    #[arg(help = "Force", short, long)]
    force: bool,

    // TODO: Implement Output
    #[arg(help = "Output File", short, long)]
    output: Option<String>,
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
        Err(_e) => println!("Content is not valid UTF-8. Use --lossy to print it anyways.")
    }
}

fn main() -> std::io::Result<()> {
    // Parse Arguments
    let args = Args::parse();
    // Normalize Content (Replace escape sequences)
    let normalized_content = normalize_text(args.content.clone());
    
    // Connect
    let mut stream = match TcpStream::connect(format!("{}:{}", args.target, args.port)) {
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
        print_content(content, args.lossy);
        return Ok(());
    }

    // TODO: Save content to file
    println!("Save to file - TODO");

    return Ok(());
}