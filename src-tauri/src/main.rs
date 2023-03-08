use std::process::Command;
use image_compression::dir_compression;
use std::path::Path;

// Extracts a JPEG preview image from a raw image file using exiftool.

// exiftool -b -JpgFromRaw -w jpg -ext dng DIR
// exiftool -previewimage -w _preview.jpg -ext cr2 -b DIR
// exiftool -a -b -W %d%f_%t%-c.%s -preview:all dir

#[tauri::command]
fn extract_jpg_preview(raw_dir_file_path: String) {
    // Run the exiftool command to extract the JPEG preview.
    println!("extract_jpg_preview called with path: {}", raw_dir_file_path);
    let mut dir = "/Users/vanshikadhingra/OneDrive - International Institute of Information Technology/8Mar2023_dass/src-tauri/src/image2".to_owned();
    dir = dir + "%f_%t%-c.%s";

    Command::new("exiftool")
        .arg("-o")
        .arg(&dir)
        .arg("-a")
        .arg("-b")
        .arg("-W")
        .arg("-previewimage")
        .arg(&raw_dir_file_path)
        .output()
        .expect("Failed to execute exiftool");
}

fn main() {
  tauri::Builder::default()
    // This is where you pass in your commands
    .invoke_handler(tauri::generate_handler![extract_jpg_preview])
    .run(tauri::generate_context!())
    .expect("failed to run app");
     //Extract a JPEG preview image from a raw file.
    //  let raw_dir_file_path=String::from("/Users/vanshikadhingra/dass/src-tauri/images");
  
    //  extract_jpg_preview(raw_dir_file_path);

    // // Compress all previews extracted into image2 folder
    // // Here goes the path where the previews are stored.
    // let dir_string = Path::new(r"./src/image2");
    // // Here goes the path where the compressed jpegs are stored.
    // let dir_string2 = Path::new(r"./src/image2output");
    // dir_compression(dir_string, dir_string2);

    // println!("Preview image extracted and saved to 'preview.jpg'.");
    
}
