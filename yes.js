var http = require('http'),
    fileSystem = require('fs'),
    path = require('path')
    util = require('util');

http.createServer(function(request, response) {
    var filePath = './nate is bad.mp3';
    var stat = fileSystem.statSync(filePath);

    response.writeHead(200, {
        'Content-Type': 'audio/mpeg',
        'Content-Length': stat.size
    });

    var readStream = fileSystem.createReadStream(filePath);
    // We replaced all the event handlers with a simple call to util.pump()
    readStream.pipe(response);
})
.listen(2000);