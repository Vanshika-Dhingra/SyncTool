// // When using the Tauri global script (if not using the npm package)
// // Be sure to set `build.withGlobalTauri` in `tauri.conf.json` to true
const invoke = window.__TAURI__.invoke
const path = window.__TAURI__.path
const dialog = window.__TAURI__.dialog
const Command=window.__TAURI__.shell.Command
// import * as walkdir from 'walkdir';
// import * as chrono from 'chrono-node';
// import { EventEmitter } from 'events';
// import events from 'events';

// const emitter = new EventEmitter();

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
      //outputFolderPath=result+'/'
      outputFolderPath = `${result}/output%f_%t%-c.%s`;
    })
    .catch((err) => {
      console.error(err);
    });
});

document.getElementById('compress').addEventListener('click', () => {
  invoke('compress_jpeg', { rawDirFilePath: inputFolderPath,outputFolder: outputFolderPath })
    .then((res) => {
      console.log(res)
    })
    .catch((err) => {
      console.error(err)
    })
});

// document.getElementById('preview').addEventListener('click', async () => {
//   console.log(outputFolderPath);
//   const command = Command.sidecar('./exiftool_folder/blib/script/exiftool');
//   const args = ["-a","-b","-W","/Users/vanshikadhingra/output%f","-previewimage","/Users/vanshikadhingra/images/"]
//   const { stdout, stderr } = await command.execute(args, { stdio: ["pipe", "pipe", "pipe"] });
//   const outputElement = document.getElementById('output');
//   outputElement.textContent = `stdout: ${stdout}\nstderr: ${stderr}`;
//   console.log(`stdout: ${stdout}\nstderr: ${stderr}`);
// });
document.getElementById('preview').addEventListener('click', async () => {  
  console.log(outputFolderPath);
  const command = Command.sidecar('./exiftool_folder/blib/script/exiftool',["-a","-b","-W",outputFolderPath,"-previewimage",inputFolderPath]);
  const outputElement = command.execute()
  console.log(outputElement)
  // outputElement.textContent = `stdout: ${stdout}\nstderr: ${stderr}`;
  // console.log(`stdout: ${stdout}\nstderr: ${stderr}`);
});

