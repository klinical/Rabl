const { app, BrowserWindow, Menu} = require('electron')

app.allowRendererProcessReuse = false

function createWindow () {
  const win = new BrowserWindow({
    width: 600,
    height: 800,
    webPreferences: {
      nodeIntegration: true,
      enableRemoteModule: true
    },
    frame: true, transparent: true, alwaysOnTop: true
  })
  var menu= null
  Menu.setApplicationMenu(menu)
  win.setResizable(false)

  win.loadFile('Views/login.html')
  win.openDevTools()
}

app.whenReady().then(createWindow)

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit()
  }
})

app.on('activate', () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow()
  }
})