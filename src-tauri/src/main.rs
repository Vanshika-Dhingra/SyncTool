use tauri::{
    api::process::{Command, CommandEvent},
    Manager,
  };
  
use std::path::PathBuf;
use std::sync::mpsc;
use image_compressor::{FolderCompressor, Factor};

#[tauri::command]
async fn extract_jpg_preview(raw_dir_file_path: &str, output_folder: &str) -> Result<String, String> {
    let mut diir = output_folder.to_owned();
    diir = diir + "%f_%t%-c.%s";
  let (mut rx, mut child) = Command::new_sidecar("exiftool")
    .expect("failed to create `my-sidecar` binary command")
    .args(["-a","-b","-W",&diir,"-previewimage",raw_dir_file_path])
    .spawn()
    .expect("Failed to spawn sidecar");

  let mut output = String::new();
  while let Some(event) = rx.recv().await {
    if let CommandEvent::Stdout(line) = event {
      output.push_str(&line);
    }
  }
  Ok(output)
}

#[tauri::command]
fn compress_jpeg(raw_dir_file_path: &str, output_folder: &str) {
    let origin = PathBuf::from(raw_dir_file_path); // original directory path
    let dest = PathBuf::from(output_folder); // destination directory path
    let thread_count = 16; // number of threads
    let (tx, tr) = mpsc::channel(); // Sender and Receiver. for more info, check mpsc and message passing.

    let mut comp = FolderCompressor::new(origin, dest);
    comp.set_cal_func(|_width, _height, _file_size| return Factor::new(70.0, 1.0));
    comp.set_thread_count(thread_count);
    comp.set_sender(tx);

    match comp.compress() {
        Ok(_) => {}
        Err(e) => println!("Cannot compress the folder!: {}", e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![compress_jpeg,extract_jpg_preview]) // Add compress_jpeg here
        .run(tauri::generate_context!())
        .expect("failed to run app");
}