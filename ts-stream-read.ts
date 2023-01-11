const rs = require('.');
const fs = require('fs');

let file = fs.createReadStream('file.txt');
let output = new rs.JsSink();

output.on('newListener', function(event, func) {
    console.log('--- newListener  ', event, func);
})

function openned() {
    // This just pipes the read stream to the response object (which goes to the client)
    file.pipe(output);
  }

// This will wait until we know the readable stream is actually valid before piping
file.on('open', openned);