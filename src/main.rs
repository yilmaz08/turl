use std::{
    io::{Read, Write},
    net::TcpStream
};
use clap::Parser;

mod input_functions;
mod file_functions;

#[derive(Parser, Debug, Clone)]
#[command(version, about)]
struct Args {
    #[arg(help = "Target Address")]
    addr: String,
    
    // Request Content
    #[arg(help = "Request Content (overwrites --content-file)", short, long)]
    content: Option<String>,
    #[arg(help = "Request Content File", long)]
    content_file: Option<String>,

    #[arg(help = "HTTP Method (a http request is formed and content data is used as http body)", long)]
    http: Option<String>,

    // Output
    #[arg(help = "Output File", short, long)]
    output: Option<String>,

    // Boolean Flags
    #[arg(help = "Force", short, long)]
    force: bool,
    #[arg(help = "Debug", short, long)]
    debug: bool
}

// Static Variables
static HTTP_METHODS: [&str; 9] = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS", "CONNECT", "TRACE"];

// Printing
fn print_response(response: Vec<u8>, lossy: bool) {
    if lossy {
        println!("{}", String::from_utf8_lossy(&response));
        return;
    }
    match String::from_utf8(response) {
        Ok(s) => println!("{}", s),
        Err(_e) => println!("Response is not valid UTF-8. Use --force to print it anyway or save it to a file.")
    }
}
fn print_connection_error(e: std::io::Error) {
    if e.kind() == std::io::ErrorKind::ConnectionRefused {
        println!("Error: Connection refused");
    } else if e.kind() == std::io::ErrorKind::TimedOut {
        println!("Error: Connection timed out");
    } else {
        println!("Error: {}", e);
    }
}

fn normalize_text(content: String) -> String {
    // Applies escape sequences
    // TODO: Improve to support all possible escape sequences
    return content.replace("\\r", "\r")
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
        .replace("\\'", "'");
}

fn get_content(args: Args) -> String {
    // Content is provided as argument
    if args.content.is_some() {
        return normalize_text(args.content.clone().expect("Request content couldn't be normalized"));
    }
    
    // Get content from file
    if args.content_file.is_some() {
        return file_functions::read_from_file(args.content_file.unwrap()).expect("Failed to read request content file");
    }

    // Multi-line input
    println!("Enter Request Content (Ctrl+D to finish):");
    let input = input_functions::get_multiline_input();
    return input;
}

fn save_response_to_file(response: Vec<u8>, file_path: String, force: bool) -> std::io::Result<()> {
    if !std::path::Path::new(&file_path).exists() {
        file_functions::write_to_file(response, file_path.clone(), true)?;
        println!("Request content created at {}", file_path);
        return Ok(());
    }

    if force {
        let _ = file_functions::write_to_file(response, file_path.clone(), true);
        println!("Request content created at {}", file_path);
        return Ok(());
    }

    let inp = input_functions::get_singleline_input("File already exists! (O)verride/(A)ppend/(C)ancel".to_string());

    match inp.trim().to_uppercase().as_str() {
        "O" => {
            let _ = file_functions::write_to_file(response, file_path.clone(), true);
            println!("Request content saved to {}", file_path);
        }
        "A" => { 
            let _ = file_functions::write_to_file(response, file_path.clone(), false);
            println!("Request content appended into {}", file_path);
        } 
        _ => { println!("Cancelled"); }
    }
    return Ok(());
}

fn parse_http_path(addr: String) -> String {
    // TODO: Create a parser
    return "/".to_string(); // Temporary solution
}

fn http_method_validity_check(method: String, force: bool) -> bool {
    if force { return true; } // Force flag is set, ignore check
    return HTTP_METHODS.contains(&method.to_uppercase().as_str());
}

fn main() -> std::io::Result<()> {
    // Parse Arguments
    let args = Args::parse();
    
    // Get Request
    let mut request = get_content(args.clone());

    // HTTP Mode
    if args.http.is_some() {
        if !http_method_validity_check(args.http.clone().unwrap(), args.force) {
            println!("Error: Invalid HTTP Method, use a valid HTTP method or use --force to ignore this check");
            std::process::exit(1);
        }

        let request_line = format!("{} {} {}\r\n", args.http.unwrap().to_uppercase(), parse_http_path(args.addr.clone()), "HTTP/1.1".to_string());
        let mut headers = String::new();

        headers.push_str(&format!("Host: {}\r\n", args.addr));
        headers.push_str(&format!("Content-Length: {}\r\n", request.len()));

        request = format!("{}{}\r\n{}", request_line, headers, request);
    }

    // Connect
    let mut stream = match TcpStream::connect(args.addr) {
        Ok(stream) => stream,
        Err(e) => {
            print_connection_error(e);
            std::process::exit(1);
        }
    };

    if args.debug {
        println!("--- REQUEST ---\n{}", request);
    }

    // Request and shutdown write
    stream.write_all(request.as_bytes())?;
    stream.shutdown(std::net::Shutdown::Write)?;

    // Read response
    let mut response = Vec::<u8>::new();
    stream.read_to_end(&mut response)?;

    // Print response
    if !args.output.is_some() {
        if args.debug {
            println!("--- RESPONSE ---");
        }
        print_response(response, args.force);
        return Ok(());
    }

    // Save response to file
    return save_response_to_file(response, args.output.unwrap(), args.force);
}