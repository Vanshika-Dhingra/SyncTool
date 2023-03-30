// use std::process::Command;
// use image_compression::dir_compression;
// use std::path::Path;
// #[tauri::command]
//  fn extract_jpg_preview(raw_dir_file_path: &str,output_folder: &str) {
//     let mut diir =output_folder.to_owned();
//     diir = diir + "%f_%t%-c.%s";

//     Command::new("exiftool")

//         .arg("-a")
//         .arg("-b")
//         .arg("-W")
//         .arg(diir)

//         .arg("-previewimage")
//         .arg(raw_dir_file_path)
//         .output()
//         .map_err(|e| format!("Failed to execute exiftool: {}", e)).unwrap();
//  }

//  #[tauri::command]
//   fn compress_jpeg(source_dir: &str, dest_dir: &str)
// {
//     let origin = PathBuf::from(source_dir);   // original directory path
//     let dest = PathBuf::from(dest_dir);       // destination directory path
//     let thread_count = 16;                       // number of threads
//     let (tx, tr) = mpsc::channel();             // Sender and Receiver. for more info, check mpsc and message passing.

//     let mut comp = FolderCompressor::new(origin, dest);
//     comp.set_cal_func(|_width, _height, _file_size| {return Factor::new(70.0, 1.0)});
//     comp.set_thread_count(thread_count);
//     comp.set_sender(tx);

//     match comp.compress(){
//         Ok(_) => {},
//         Err(e) => println!("Cannot compress the folder!: {}", e),
//     }
// }

// fn main() {
//   tauri::Builder::default()
//       .invoke_handler(tauri::generate_handler![extract_jpg_preview])
//       .run(tauri::generate_context!())
//       .expect("failed to run app");
// }
use std::process::Command;
use std::path::PathBuf;
use std::sync::mpsc;
use image_compressor::{FolderCompressor, Factor};

#[tauri::command]
fn extract_jpg_preview(raw_dir_file_path: &str, output_folder: &str) {
    let mut diir = output_folder.to_owned();
    diir = diir + "%f_%t%-c.%s";

    Command::new("exiftool")
        .arg("-a")
        .arg("-b")
        .arg("-W")
        .arg(diir)
        .arg("-previewimage")
        .arg(raw_dir_file_path)
        .output()
        .map_err(|e| format!("Failed to execute exiftool: {}", e))
        .unwrap();
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
        .invoke_handler(tauri::generate_handler![extract_jpg_preview, compress_jpeg]) // Add compress_jpeg here
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
