use crate::build;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Instant,
};

pub fn start_dev_server(mut path: String) {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    println!("Server listening on port 6969");

    if path.is_empty() {
        let current_dir = std::env::current_dir();

        match current_dir {
            Ok(dir) => {
                path = dir.to_string_lossy().into_owned();
            }
            Err(e) => {
                println!("Error getting current directory: {:?}", e);
            }
        }
    }

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        build_project(&"test".to_string());
        handle_connection(stream, path.clone());
    }
}

fn handle_connection(mut stream: TcpStream, path: String) {
    let buf_reader = BufReader::new(&mut stream);
    let current_dir = std::env::current_dir();
    let entry_path = match current_dir {
        Ok(dir) => dir.to_string_lossy().into_owned(),
        Err(e) => {
            println!("Error getting current directory: {:?}", e);
            "/".to_string()
        }
    };

    // println!("{}", format!("{}/{}/dist/any file should be here", entry_path, path));
    let mut contents =
        fs::read(format!("{}/{}/dist/404.html", entry_path, path)).unwrap();
    let mut length = contents.len();
    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let mut content_type = "text/html";

    let request_line = buf_reader.lines().next().unwrap();
    match request_line {
        Ok(request) => {

            // HANDLE REQUESTS
            if request == "GET / HTTP/1.1" {
                contents = fs::read(format!("{}/dist/index.html", path)).unwrap();
                length = contents.len();
                status_line = "HTTP/1.1 200 OK";
                println!("Sending Home page");
            } else if request.starts_with("GET /") {
                // Get requested path
                let file_path = request.split_whitespace().collect::<Vec<&str>>()[1];

                // Set the Content-Type based on the file extension
                if file_path.ends_with(".js") {
                    content_type = "application/javascript";
                } else if file_path.ends_with(".wasm") {
                    content_type = "application/wasm";
                } else if file_path.ends_with(".css") {
                    content_type = "text/css";
                }

                println!("Requested path: {}", file_path);
                let file_requested = if file_path.ends_with(".wasm") {
                        fs::read(format!("{}/dist{}", path, file_path))
                    } else {
                        fs::read_to_string(format!("{}/dist{}", path, file_path)).map(|c| c.into_bytes())
                    };

                match file_requested {
                    Ok(c) => {
                        // Make sure the path does not try to access any directories outside of /dist
                        if !file_path.contains("..") {
                            contents = c;
                            length = contents.len();
                            status_line = "HTTP/1.1 200 OK";
                            println!("Sending requested file");
                        }
                    }
                    Err(_) => {
                        println!("File not found");
                    }
                }
            }
        }
        _ => {
            println!("Error reading request line");
        }
    }

    let string_response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        status_line,
        length,
        content_type,
    );

    let response = &[string_response.as_bytes(), &contents].concat();

    match stream.write_all(response) {
        Ok(_) => {
            println!("Response sent");
        }
        Err(e) => {
            println!("Error sending response: {:?}", e);
        }
    };
}

fn build_project(build_path: &String) {
    println!("Building project...");
    let start = Instant::now();
    match build::build(build_path.to_string()) {
        Ok(_) => {
            let duration = start.elapsed();
            println!("Project built in: {:?}", duration);
        }
        Err(e) => {
            println!("Error building project: {:?}", e);
            return;
        }
    }
}