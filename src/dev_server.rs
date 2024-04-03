use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, path::Path, time::{Duration, Instant}, thread
};
use notify::{RecursiveMode, Watcher};
use notify_debouncer_mini::{new_debouncer_opt, Config};
use crate::build;

pub fn start_dev_server(path: String) {
    let listener = TcpListener::bind("127.0.0.1:6969").unwrap();
    
    // watch_directory(Path::new("./src"), &path);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, path.clone());
    }

}

fn handle_connection(mut stream: TcpStream, path: String) {
    let buf_reader = BufReader::new(&mut stream);
    let mut contents = fs::read_to_string(format!("{}/dist/404.html", path)).unwrap();
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
                contents = fs::read_to_string(format!("{}/dist{}", path, file_path)).unwrap();

                // Make sure the path does not try to access any directories outside of /dist
                if !file_path.contains("..") {
                    length = contents.len();
                    status_line = "HTTP/1.1 200 OK";
                    println!("Sending page");
                }
            }
            if request.starts_with("GET /css") {
                // Get requested css path
                let file_path = request.split_whitespace().collect::<Vec<&str>>()[1];

                if !file_path.contains("..") {
                    contents = fs::read_to_string(format!("{}/dist{}", path, file_path)).unwrap();
                    length = contents.len();
                    status_line = "HTTP/1.1 200 OK";
                }
            }
        }
        _=> {
            println!("Error reading request line");
        }
    }

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

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
    let debouncer_config = Config::default()
        .with_notify_config(backend_config);
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
            },
            Err(error) => println!("Error {error:?}"),
        }
    }
}

