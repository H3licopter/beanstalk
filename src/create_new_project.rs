use std::{fs, env};
use fs_extra::dir::{copy, CopyOptions};

pub fn create_project(user_project_path: String) -> Result<(), fs_extra::error::Error> {
  // Get the current directory
  let current_dir = env::current_dir()?;

  // Create the full path to the user specified path
  let full_path = current_dir.join(user_project_path);

  // Create user specified path
  fs::create_dir_all(&full_path)?;

  let options = CopyOptions::new(); // Default options

  // Copy project directory from /output folder to user specified path
  copy("src/html_project_template", &full_path, &options)?;

  println!("Project created at: {:?}", &full_path);

  Ok(())
}