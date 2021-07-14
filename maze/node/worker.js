const { parentPort } = require('worker_threads');
const { greet, args } = require('../rust/pkg/img_tools.js');
parentPort.on('message', (message) => {
    if (message.action === 'generate') {
        const arr = new Uint8Array(11);
        parentPort.postMessage(arr);
        parentPort.postMessage(greet(arr));
        parentPort.postMessage(arr);
    } else if (message.action === 'process') {
        console.log("worker", message.pixels);
        console.log(args(new Uint8Array(message.pixels)));
    }
});