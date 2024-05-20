use crate::build;
use colour::{blue_ln, dark_cyan, dark_cyan_ln, dark_yellow_ln, green_ln_bold, print_bold, red_ln};
use std::error::Error;
use std::time::SystemTime;

use std::{
    fs::{self, metadata},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Instant,
};

pub fn start_dev_server(mut path: String) -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    println!("Server listening on port 6969");

    let current_dir = std::env::current_dir()?;
    path = format!("{}/{}", current_dir.to_string_lossy().into_owned(), path);

    build_project(&path, true);

    let mut modified = get_last_modified(&format!("{}/src", &path));
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, path.clone(), &mut modified);
    }

    Ok(())
}

fn handle_connection(
    mut stream: TcpStream,
    path: String,
    last_modified: &mut std::time::SystemTime,
) {
    let buf_reader = BufReader::new(&mut stream);

    // println!("{}", format!("{}/{}/dist/any file should be here", entry_path, path));
    let mut contents = fs::read(format!("{}/dist/404.html", path)).unwrap();
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
                dark_cyan_ln!("Sending Home page");
            } else if request.starts_with("HEAD /check") {
                // check if anything has changed in the src folder since the last check,
                // send a response to the client indicating there has been a change

                // Get the metadata of the file
                let check_modified = get_last_modified(&format!("{}/src", &path));

                if *last_modified < check_modified {
                    blue_ln!("Changes detected in src folder");
                    build_project(&path, false);
                    *last_modified = check_modified;
                    status_line = "HTTP/1.1 205 Reset Content";
                } else {
                    status_line = "HTTP/1.1 200 OK";
                }
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
                } else if file_path.ends_with(".png") {
                    content_type = "image/png";
                } else if file_path.ends_with(".jpg") {
                    content_type = "image/jpeg";
                } else if file_path.ends_with(".ico") {
                    content_type = "image/ico";
                }

                dark_cyan!("Requested path: ");
                dark_yellow_ln!("{}", file_path);

                let file_requested = if file_path.ends_with(".wasm")
                    || file_path.ends_with(".png")
                    || file_path.ends_with(".jpg")
                    || file_path.ends_with(".ico")
                {
                    fs::read(format!("{}/dist{}", path, file_path))
                } else {
                    fs::read_to_string(format!("{}/dist{}", path, file_path))
                        .map(|c| c.into_bytes())
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
                        red_ln!("File not found");
                    }
                }
            }
        }
        _ => {
            red_ln!("Error reading request line");
        }
    }

    let string_response = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        status_line, length, content_type,
    );

    let response = &[string_response.as_bytes(), &contents].concat();

    match stream.write_all(response) {
        Ok(_) => {}
        Err(e) => {
            red_ln!("Error sending response: {:?}", e);
        }
    };
}

fn build_project(build_path: &String, release: bool) {
    dark_cyan_ln!("Building project...");
    let start = Instant::now();
    match build::build(build_path.to_string(), release) {
        Ok(_) => {
            let duration = start.elapsed();
            print_bold!("Project built in: ");
            green_ln_bold!("{:?}", duration);
        }
        Err(e) => {
            red_ln!("Error building project: {:?}", e);
            return;
        }
    }
}

fn get_last_modified(path: &String) -> SystemTime {
    let mut latest_mod_time = SystemTime::UNIX_EPOCH;
    let entries = match fs::read_dir(path) {
        Ok(all) => all,
        Err(_) => {
            red_ln!("Error reading directory");
            return latest_mod_time;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                red_ln!("Error reading entry");
                return latest_mod_time;
            }
        };

        let meta = match metadata(entry.path()) {
            Ok(m) => m,
            Err(_) => {
                red_ln!("Error reading file modified metadata");
                return latest_mod_time;
            }
        };

        let modified_time = match meta.modified() {
            Ok(t) => t,
            Err(_) => {
                red_ln!("Error reading file modified time in it's metadata");
                return latest_mod_time;
            }
        };

        if modified_time > latest_mod_time {
            latest_mod_time = modified_time;
        }
    }

    latest_mod_time
}
