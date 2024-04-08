use crate::build;
use std::{path::{Path, PathBuf}, time::{Duration, Instant}};
use notify_debouncer_mini::{new_debouncer_opt, Config, DebouncedEvent};
use notify::{RecursiveMode, Watcher};
use tokio::fs;
use tokio::sync::mpsc;
use tokio::task;

#[tokio::main]
pub async fn launch_server(path: String) {
    let path = format!("{}/test_output/dist", path);

    // Create a channel to communicate changes
    let (sender, mut reciever) = mpsc::channel(1);

    // Spawn a task to watch for file changes
    task::spawn(watch_files(path.clone(), sender, reciever));

    // Define a filter to serve static files
    let static_files = warp::fs::dir(path.clone());

    // Launch the server
    warp::serve(static_files).run(([127, 0, 0, 1], 6969)).await;
}

async fn watch_files(path: String, sender: mpsc::Sender<()>, mut reciever: mpsc::Receiver<()>) {
    // Watch for changes in the directory
    let dist_dir = format!("{}/test_output/dist", path);
    let src_dir = format!("{}/test_output/src", path);
    
    // setup debouncer
    // notify backend configuration
    let backend_config = notify::Config::default().with_poll_interval(Duration::from_secs(2));
    // debouncer configuration
    let debouncer_config = Config::default().with_notify_config(backend_config);
    // select backend via fish operator, here PollWatcher backend
    let mut debouncer = new_debouncer_opt::<_, 
        notify::PollWatcher>(
        debouncer_config, 
        move |result: Result<Vec<DebouncedEvent>, notify::Error>| {
            if let Ok(events) = result {
                for event in events {
                    println!("File changed: {:?}", event);
                    let _ = sender.send(());
                }
            } else if let Err(e) = result {
                println!("Watch error: {:?}", e);
            }
        }
    ).unwrap();

    debouncer
        .watcher()
        .watch(Path::new(&src_dir), RecursiveMode::Recursive)
        .unwrap();

    loop {
        // Wait for a file change event
        let _ = reciever.recv();

        // Print a message indicating a change
        println!("File changed, restarting server...");

        // Restart the server
        let path = dist_dir.clone();
        task::spawn(async move {
            // Gracefully shutdown the server
            tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");

            // Serve static files again
            warp::serve(warp::fs::dir(path.clone()))
                .run(([127, 0, 0, 1], 6969))
                .await;
        });
    }
}

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

/*

fn handle_connection(mut stream: TcpStream, path: String) {
    let buf_reader = BufReader::new(&mut stream);


    // 
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

 */

