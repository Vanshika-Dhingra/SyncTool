// // When using the Tauri global script (if not using the npm package)
// // Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.invoke
const path = window.__TAURI__.path
const dialog = window.__TAURI__.dialog

// // Invoke the command
// invoke('extract_jpg_preview', { rawDirFilePath: '/Users/vanshikadhingra/dass/src-tauri/images' })

let inputFolderPath = '';
let outputFolderPath = '';
// const folderInput = document.getElementById("folder");
// folderInput.addEventListener("change", (event) => {
//   const folderPath = event.target.value;
//   // const folderrr=folderPath;
//   // inputFolderPath = path.dirname(folderrr);
//   console.log("Folder path:",folderPath);
//   });






document.getElementById('folder').addEventListener('click', () => {
  //invoke('extract_jpg_preview', { rawDirFilePath: '/Users/vanshikadhingra/OneDrive - International Institute of Information Technology/8Mar2023_dass/src-tauri/images' })
  dialog
    .open({
      directory: true, // Set directory to true to allow selecting a folder instead of a file
    })
    .then((result) => {
      // result contains an array of paths that the user has selected
      inputFolderPath=result
    })
    .catch((err) => {
      console.error(err);
    });
});

document.getElementById('outFolder').addEventListener('click', () => {
  //invoke('extract_jpg_preview', { rawDirFilePath: '/Users/vanshikadhingra/OneDrive - International Institute of Information Technology/8Mar2023_dass/src-tauri/images' })
  dialog
    .open({
      directory: true, // Set directory to true to allow selecting a folder instead of a file
    })
    .then((result) => {
      // result contains an array of paths that the user has selected
      outputFolderPath=result+'/'
    })
    .catch((err) => {
      console.error(err);
    });
});

document.getElementById('preview').addEventListener('click', () => {
      invoke('extract_jpg_preview', { rawDirFilePath: inputFolderPath,outputFolder: outputFolderPath }, (res) => {
        console.log(res)
      })
});
