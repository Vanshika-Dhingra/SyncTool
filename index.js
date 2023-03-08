// // When using the Tauri global script (if not using the npm package)
// // Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.invoke
const path = window.__TAURI__.path

// // Invoke the command
// invoke('extract_jpg_preview', { rawDirFilePath: '/Users/vanshikadhingra/dass/src-tauri/images' })

// const finalfolderpath='';
// const folderInput = document.getElementById("folder");
// folderInput.addEventListener("change", (event) => {
//   const folderPath = event.target.value;
//   const folderrr=folderPath;
//   finalfolderpath = path.dirname(folderrr);
//   console.log("Folder path:", finalfolderpath);
// });




  
  document.getElementById('preview').addEventListener('click', () => {
invoke('extract_jpg_preview', { rawDirFilePath: '/Users/vanshikadhingra/OneDrive - International Institute of Information Technology/8Mar2023_dass/src-tauri/images' })
 
  });

