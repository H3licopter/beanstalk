use crate::{build, settings};
use colour::{blue_ln, dark_cyan_ln, green_ln_bold, grey_ln, print_bold, red_ln};
use settings::get_default_config;
use std::error::Error;
use std::path::PathBuf;
use std::time::SystemTime;
use std::{
    fs::{self, metadata},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    time::Instant,
};

pub fn start_dev_server(mut path: String) -> Result<(), Box<dyn Error>> {
    let url = "127.0.0.1:6969";
    let listener = TcpListener::bind(url).unwrap();
    print_bold!("Dev Server created on: ");
    green_ln_bold!("http://{}", url.replace("127.0.0.1", "localhost"));

    let current_dir = std::env::current_dir()?;
    path = format!("{}/{}", current_dir.to_string_lossy().into_owned(), path);
    build_project(&path, false);

    let mut modified = SystemTime::UNIX_EPOCH;
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

    // println!("{}", format!("{}/{}/dev/any file should be here", entry_path, path));
    let mut contents = fs::read(format!("{}/dev/404.html", path)).unwrap();
    let mut status_line = "HTTP/1.1 404 NOT FOUND";
    let mut content_type = "text/html";

    let request_line = buf_reader.lines().next().unwrap();
    match request_line {
        Ok(request) => {
            // HANDLE REQUESTS
            if request == "GET / HTTP/1.1" {
                match get_home_page_path(&path, false) {
                    Ok(p) => {
                        contents = fs::read(p).unwrap();
                        status_line = "HTTP/1.1 200 OK";
                    }
                    Err(e) => {
                        red_ln!("Error reading home page: {:?}", e);
                    }
                };
            } else if request.starts_with("HEAD /check") {
                // the check request has the page url as a query parameter after the /check
                let request_path = request.split("?page=").nth(1);

                let parsed_url = match request_path {
                    Some(p) => {
                        let page_path = p.split_whitespace().collect::<Vec<&str>>()[0];
                        if page_path == "/" {
                            get_home_page_path(&path, true)
                        } else {
                            let src_path = PathBuf::from(format!(
                                "{}/{}{}",
                                path,
                                get_default_config().src,
                                page_path
                            ))
                            .with_extension("bs");
                            Ok(src_path)
                        }
                    }
                    None => get_home_page_path(&path, true),
                };

                match &parsed_url {
                    // Get the metadata of the file
                    Ok(parsed_url) => {
                        let global_file_path = PathBuf::from(format!(
                            "{}/{}/{}",
                            path,
                            get_default_config().src,
                            settings::GLOBAL_PAGE_KEYWORD
                        ))
                        .with_extension("bs");
                        let global_file_modified = if fs::metadata(&global_file_path).is_ok() {
                            has_been_modified(&global_file_path, last_modified)
                        } else {
                            false
                        };

                        if has_been_modified(parsed_url, last_modified) || global_file_modified {
                            blue_ln!("Changes detected for {:?}", parsed_url);
                            build_project(&path, false);
                            status_line = "HTTP/1.1 205 Reset Content";
                        } else {
                            status_line = "HTTP/1.1 200 OK";
                        }
                    }
                    // Throw an error if the url is invalid
                    Err(e) => {
                        red_ln!("Error parsing url: {:?}", e);
                        status_line = "HTTP/1.1 404 NOT FOUND";
                    }
                }
            } else if request.starts_with("GET /") {
                // Get requested path
                let file_path = request.split_whitespace().collect::<Vec<&str>>()[1];

                // Set the Content-Type based on the file extension
                let file_requested = if file_path.ends_with(".js") {
                    content_type = "application/javascript";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else if file_path.ends_with(".wasm") {
                    content_type = "application/wasm";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else if file_path.ends_with(".css") {
                    content_type = "text/css";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else if file_path.ends_with(".png") {
                    content_type = "image/png";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else if file_path.ends_with(".jpg") {
                    content_type = "image/jpeg";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else if file_path.ends_with(".ico") {
                    content_type = "image/ico";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else if file_path.ends_with(".webmanifest") {
                    content_type = "application/manifest+json";
                    fs::read(format!("{}/dev{}", path, file_path))
                } else {
                    let page_path = format!("{}/dev{}.html", path, file_path);

                    fs::read_to_string(page_path).map(|c| c.into_bytes())
                };

                match file_requested {
                    Ok(c) => {
                        // Make sure the path does not try to access any directories outside of /dev
                        if !file_path.contains("..") {
                            contents = c;
                            status_line = "HTTP/1.1 200 OK";
                        }
                    }
                    Err(_) => {
                        red_ln!("File not found: {}", file_path);
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
        status_line,
        contents.len(),
        content_type,
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
            grey_ln!("------------------------------------");
            print!("\nProject built in: ");
            green_ln_bold!("{:?}", duration);
        }
        Err(e) => {
            red_ln!("Error building project: {:?}", e);
            return;
        }
    }
}

fn has_been_modified(path: &PathBuf, modified: &mut std::time::SystemTime) -> bool {
    // Check if it's a file or directory
    let path_metadata = match fs::metadata(path) {
        Ok(m) => m,
        Err(_) => {
            red_ln!(
                "Error reading directory (probably doesn't exist): {:?}",
                path
            );
            return false;
        }
    };

    if path_metadata.is_dir() {
        let entries = match fs::read_dir(path) {
            Ok(all) => all,
            Err(_) => {
                red_ln!("Error reading directory: {:?}", path);
                return false;
            }
        };

        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(_) => {
                    red_ln!("Error reading entry");
                    return false;
                }
            };

            let meta = match metadata(entry.path()) {
                Ok(m) => m,
                Err(_) => {
                    red_ln!("Error reading file modified metadata");
                    return false;
                }
            };

            let modified_time = match meta.modified() {
                Ok(t) => t,
                Err(_) => {
                    red_ln!("Error reading file modified time in it's metadata");
                    *modified
                }
            };

            if modified_time > *modified {
                *modified = modified_time;
                return true;
            }
        }
    }

    if path_metadata.is_file() {
        match path_metadata.modified() {
            Ok(t) => {
                if t > *modified {
                    *modified = t;
                    return true;
                }
            }
            Err(_) => {
                red_ln!("Error reading file modified time in it's metadata");
                return false;
            }
        }
    }

    false
}

fn get_home_page_path(path: &String, src: bool) -> Result<PathBuf, Box<dyn Error>> {
    let root_src_path = if src {
        PathBuf::from(&path).join(get_default_config().src)
    } else {
        PathBuf::from(&path).join(get_default_config().dev_folder)
    };

    let src_files = match fs::read_dir(root_src_path) {
        Ok(m) => m,
        Err(e) => {
            red_ln!("Error reading root src directory metadata");
            return Err(e.into());
        }
    };

    // Look for first file that starts with '#page' in the src directory
    let mut first_page = None;
    for entry in src_files {
        first_page = match entry {
            Ok(e) => {
                let page = e.path();
                if src {
                    if page
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .starts_with(settings::COMP_PAGE_KEYWORD)
                    {
                        Some(page)
                    } else {
                        continue;
                    }
                } else {
                    if page
                        .file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .starts_with(settings::INDEX_PAGE_KEYWORD)
                    {
                        Some(page)
                    } else {
                        continue;
                    }
                }
            }
            Err(e) => {
                red_ln!("Error reading src directory");
                return Err(e.into());
            }
        };
    }

    match first_page {
        Some(index_page_path) => Ok(index_page_path),
        None => {
            red_ln!(
                "No page found in {} directory: {:?}",
                if src { "src" } else { "dev" },
                first_page
            );
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "No page found in src directory",
            )));
        }
    }
}
