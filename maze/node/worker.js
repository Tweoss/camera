const { parentPort } = require('worker_threads');
const { undistort_data, init_panic_hook } = require('../rust/pkg/img_tools.js');
init_panic_hook();
parentPort.on('message', (message) => {
    if (message.action === 'generate') {
        const arr = new Uint8Array(11);
        parentPort.postMessage(arr);
        parentPort.postMessage(greet(arr));
        parentPort.postMessage(arr);
    } else if (message.action === 'process') {
        // message.pixels is an arraybuffer containing RGBA data and owned by the worker (transferred)
        // we create another Uint8Array and transform message.pixels into it
        // for testing visualization, we create another Uint8Array with the same size as the transformed pixels and calculate the strength of corners
        // we keep a list of the strongest corners while calculating
        // we return the top 4 corners 

        parentPort.postMessage(undistort_data(message.pixels, message.width, message.height, -0.05, 0.0, 0.0));
    }
});