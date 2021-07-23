const { parentPort } = require('worker_threads');
const { undistort_data, init_panic_hook, detect_corners, ToleranceOptions, WeightageOptions, corner_map, condense_corners } = require('../rust/pkg/img_tools.js');
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


        // parentPort.postMessage({ action: 'undistorted', data: undistort_data(message.pixels, message.width, message.height, -0.05, 0.0, 0.0) });

        let opt = new ToleranceOptions();
        // opt.black_dist = 100;
        // opt.white_dist = 100;
        // opt.avg = 120;
        // opt.intersect_dist = 3;

        let weight = new WeightageOptions();
        weight.black_dist = 1.0;
        weight.avg = 1.0;
        weight.white_dist = 1.0;
        weight.intersect_dist = 2.0;
        weight.lock();

        parentPort.postMessage({ action: 'undistorted', data: corner_map(message.pixels, message.width, message.height, 7, opt, weight) });
        console.log("wasm finished");

        opt = new ToleranceOptions();
        // opt.black_dist = 100;
        // opt.white_dist = 100;
        // opt.avg = 120;
        // opt.intersect_dist = 80;
        // opt.center_dist = 10;

        weight = new WeightageOptions();
        weight.black_dist = 1.0;
        weight.avg = 1.0;
        weight.white_dist = 1.0;
        weight.intersect_dist = 2.0;
        weight.lock();


        let a = detect_corners(message.pixels, message.width, message.height, 7, opt, weight, 30);
        let object = [];
        for (let index = 0; index < a.length; index++) {
            const element = a[index];
            // console.log(element, element.x, element.y);
            object.push({ x: element.x, y: element.y });
        }
        let b = condense_corners(object, 5.0, 4.0);
        object = [];
        for (let index = 0; index < b.length; index++) {
            const element = b[index];
            object.push({ x: element.x, y: element.y });
        }
        // console.log(object.length);
        parentPort.postMessage({ action: 'located', data: object });

    }
});