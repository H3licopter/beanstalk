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
        thread::spawn(|| {
            handle_connection(stream);
        });
    }

}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let contents = fs::read_to_string("../html_project_template/dist/index.html".to_string()).unwrap();
    let status_line = "HTTP/1.1 200 OK";
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

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
                build_and_send_html(build_path);
            },
            Err(error) => println!("Error {error:?}"),
        }
    }
}

fn build_and_send_html(build_path: &String) {
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

