const http = require("http");
const url = require("url");
const fs = require("fs");
const path = require("path");
const port = process.argv[2] || 8000;

const mimeType = {
  ".ico": "image/x-icon",
  ".html": "text/html",
  ".js": "text/javascript",
  ".json": "application/json",
  ".css": "text/css",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".wav": "audio/wav",
  ".mp3": "audio/mpeg",
  ".svg": "image/svg+xml",
  ".pdf": "application/pdf",
  ".doc": "application/msword",
  ".eot": "appliaction/vnd.ms-fontobject",
  ".ttf": "appliaction/font-sfnt",
  ".wasm": "appliaction/wasm"
};

http
  .createServer((req, res) => {
    console.log(`${req.method} ${req.url}`);
    let { pathname } = url.parse(req.url);
    pathname = path.join(__dirname, "../dist", pathname);
    fs.exists(pathname, function(exist) {
      if (!exist) {
        res.statusCode = 404;
        res.end(`File ${pathname} not found!`);
        return;
      }
      if (fs.statSync(pathname).isDirectory()) {
        pathname = path.join(pathname, "index.html");
      }
      fs.readFile(pathname, function(err, data) {
        if (err) {
          res.statusCode = 500;
          res.end(`Error getting the file: ${err}.`);
        } else {
          const { ext } = path.parse(pathname);
          const type = mimeType[ext] || "text/plain";
          res.setHeader("Content-Type", type);
          res.end(data);
        }
      });
    });
  })
  .listen(parseInt(port));
console.log(`Server listening on port ${port}`);
