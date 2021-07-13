const { parentPort } = require('worker_threads');
const { greet } = require('../rust/pkg/img_tools.js');
parentPort.once('message', (message) => {
    parentPort.postMessage(greet());
});