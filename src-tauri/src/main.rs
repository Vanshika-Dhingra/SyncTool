// use std::process::Command;
// use image_compression::dir_compression;
// use std::path::Path;
// use std::fs;    

// // Extracts a JPEG preview image from a raw image file using exiftool.

// // exiftool -b -JpgFromRaw -w jpg -ext dng DIR
// // exiftool -previewimage -w _preview.jpg -ext cr2 -b DIR
// // exiftool -a -b -W %d%f_%t%-c.%s -preview:all dir

// // #[tauri::command]
// // fn extract_jpg_preview(raw_dir_file_path: String,output_folder:String) {
// //     // Run the exiftool command to extract the JPEG preview.
// //     println!("extract_jpg_preview called with path: {}", raw_dir_file_path);
// //     //let mut dir = output_folder.to_owned();
// //     //dir = dir + "%f_%t%-c.%s";
// //     println!("extract_jpg_preview called with out path: {}", output_folder);

// //     Command::new("exiftool")
// //         .arg("-o")
// //         .arg(&output_folder)
// //         .arg("-a")
// //         .arg("-b")
// //         .arg("-W")
// //         .arg("-previewimage")
// //         .arg(&raw_dir_file_path)
// //         .output()
// //         .expect("Failed to execute exiftool");
// // }


// #[tauri::command]
// fn extract_jpg_preview(raw_dir_file_path: &str, output_folder: &str) {
//     // Get a list of all files in the input folder
//     let files = fs::read_dir(raw_dir_file_path)
//         .expect("Failed to read input folder")
//         .map(|res| res.map(|e| e.path()))
//         .collect::<Result<Vec<_>, std::io::Error>>()
//         .expect("Failed to get files in input folder");

//     // Process each file in the folder
//     for raw_file_path in files {
//         // Skip non-JPG files
//         // if raw_file_path.extension().unwrap_or_default() != "jpg" {
//         //     continue;
//         // }

//         let preview_filename = format!(
//             "{}_preview.jpg",
//             raw_file_path.file_stem().unwrap().to_str().unwrap()
//         );
//         let output_path = std::path::Path::new(output_folder).join(&preview_filename);
//         println!(
//             "Extracting preview for file {} to {}",
//             raw_file_path.display(),
//             output_path.display()
//         );

//     Command::new("exiftool")
//         .arg("-o")
//         .arg(&output_path)
//         .arg("-a")
//         .arg("-b")
//         .arg("-W")
//         .arg("-previewimage")
//         .arg(&raw_file_path)
//         .output()
//         .expect("Failed to execute exiftool");

//         // Command::new("exiftool")
//         //     .arg("-b")
//         //     .arg("-previewimage")
//         //     .arg(&raw_file_path)
//         //     .arg("-o")
//         //     .arg(&output_path)
//         //     .output()
//         //     .expect("Failed to execute exiftool");
//     }
//   }





// fn main() {
//   tauri::Builder::default()
//     // This is where you pass in your commands
//     .invoke_handler(tauri::generate_handler![extract_jpg_preview])
//     .run(tauri::generate_context!())
//     .expect("failed to run app");
//      //Extract a JPEG preview image from a raw file.
//     //  let raw_dir_file_path=String::from("/Users/vanshikadhingra/dass/src-tauri/images");
  
//     //  extract_jpg_preview(raw_dir_file_path);

//     // // Compress all previews extracted into image2 folder
//     // // Here goes the path where the previews are stored.
//     // let dir_string = Path::new(r"./src/image2");
//     // // Here goes the path where the compressed jpegs are stored.
//     // let dir_string2 = Path::new(r"./src/image2output");
//     // dir_compression(dir_string, dir_string2);

//     // println!("Preview image extracted and saved to 'preview.jpg'.");
    
// }
use std::process::Command;
use image_compression::dir_compression;
use std::path::Path;
#[tauri::command]
 fn extract_jpg_preview(raw_dir_file_path: &str,output_folder: &str) {
    let mut diir =output_folder.to_owned();
    diir = diir + "%f_%t%-c.%s";

    Command::new("exiftool")
        
        .arg("-a")
        .arg("-b")
        .arg("-W")
        .arg(diir)
       
        .arg("-previewimage")
        .arg(raw_dir_file_path)
        .output()
        .map_err(|e| format!("Failed to execute exiftool: {}", e)).unwrap();
 }

fn main() {
  tauri::Builder::default()
      .invoke_handler(tauri::generate_handler![extract_jpg_preview])
      .run(tauri::generate_context!())
      .expect("failed to run app");
}