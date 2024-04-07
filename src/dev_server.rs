use crate::build;
use notify::{RecursiveMode, Watcher};
use notify_debouncer_mini::{new_debouncer_opt, Config};
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
    time::{Duration, Instant},
};

pub fn start_dev_server(mut path: String) {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();

    // watch_directory(Path::new("./src"), &path);

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
        fs::read_to_string(format!("{}/{}/dist/404.html", entry_path, path)).unwrap();
    let mut length = contents.len();
    let mut status_line = "HTTP/1.1 404 NOT FOUND";

    let request_line = buf_reader.lines().next().unwrap();
    match request_line {
        Ok(request) => {
            // Build updated HTML on each request
            // DEFINITELY DON'T DO FOR PRODUCTION
            build_project(&path);

            // HANDLE REQUESTS
            if request == "GET / HTTP/1.1" {
                contents = fs::read_to_string(format!("{}/dist/index.html", path)).unwrap();
                length = contents.len();
                status_line = "HTTP/1.1 200 OK";
                println!("Sending Home page");
            } else if request.starts_with("GET /") {
                // Get requested path
                let file_path = request.split_whitespace().collect::<Vec<&str>>()[1];
                println!("Requested path: {}", file_path);

                if file_path.ends_with(".png") || file_path.ends_with(".jpg") || file_path.ends_with(".gif") {
                    match fs::read(format!("{}/dist{}", path, file_path)) {
                        Ok(c) => {
                            let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{c}");
                            stream.write_all(response.as_bytes()).unwrap();
                            return;
                        }
                        Err(_) => {
                            println!("Image not found");
                        }
                    }
                }

                let file_requested = fs::read_to_string(format!("{}/dist{}", path, file_path));
                match file_requested {
                    Ok(c) => {
                        // Make sure the path does not try to access any directories outside of /dist
                        if file_path.contains("..") {
                            println!("Invalid path");
                            return;
                        }

                        contents = c;
                        length = contents.len();
                        status_line = "HTTP/1.1 200 OK";
                        println!("Sending requested file");
                        
                    }
                    Err(_) => {
                        println!("File path not found");
                        fs::read_to_string(format!("{}/{}/dist/404.html", entry_path, path)).unwrap();
                    }
                }
            }
        }
        _ => {
            println!("Error reading request line");
        }
    }

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
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

// CURRENTLY JUST BLOCKS THE THREAD, NEED TO GET WORKING
fn watch_directory(dev_dir: &Path, build_path: &String) {
    println!("Watching {:?} for changes...", dev_dir);

    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();
    // notify backend configuration
    let backend_config = notify::Config::default().with_poll_interval(Duration::from_millis(100));
    // debouncer configuration
    let debouncer_config = Config::default().with_notify_config(backend_config);
    // select backend via fish operator, here PollWatcher backend
    let mut debouncer = new_debouncer_opt::<_, notify::PollWatcher>(debouncer_config, tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(dev_dir), RecursiveMode::Recursive)
        .unwrap();
    // print all events, non returning
    for result in rx {
        match result {
            Ok(event) => {
                println!("Event {event:?}");
                build_project(build_path);
            }
            Err(error) => println!("Error {error:?}"),
        }
    }
}
