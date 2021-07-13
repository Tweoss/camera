const { parentPort } = require('worker_threads');
const { greet } = require('../rust/pkg/img_tools.js');
parentPort.once('message', (message) => {
    const arr = new Uint8Array(11);
    parentPort.postMessage(arr);
    parentPort.postMessage(greet(arr));
    parentPort.postMessage(arr);
});