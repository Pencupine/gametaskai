const { app, BrowserWindow, ipcMain } = require('electron');
const path = require('path');
const url = require('url');

if (process.env.ELECTRON_ENV == 'dev') {
  require("electron-reload")(__dirname, __dirname, {
    electron: require(`${__dirname}/../node_modules/electron`)
  });
}

function createWindow() {
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false,
      devTools: true
    },
  });

  mainWindow.loadURL(
    url.format({
      pathname: path.join(__dirname, 'public/index.html'),
      protocol: 'file:',
      slashes: true
    })
  );

  // mainWindow.loadFile(path.join(__dirname, '/index.html'));

  if (process.env.NODE_ENV === 'development') {
    mainWindow.webContents.openDevTools();
  }

//   ipcMain.on('fetch-3d-object-data', (event) => {
//     const objectData = retrieveObjectDataFromRustBackend();

//     event.reply('3d-object-data', objectData);
//   });
}

// function retrieveObjectDataFromRustBackend() {
// }

// Event listener for the app's ready event
app.on('ready', createWindow);
