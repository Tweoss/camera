const { parentPort } = require('worker_threads');
const { undistort_data, init_panic_hook, detect_corners, ToleranceOptions, WeightageOptions, OverallOptions, DistortionOptions, DistortionOffsetOptions, Point } = require('../rust/pkg/img_tools.js');
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
        // we return the top 4 corners 

        // from original opencv code
        // -3.726853569351807338e-01 -6.904289461692791896e+00 -1.639538516417285733e-03 -1.753118375323940020e-02 5.536141760081946472e+01
        // from opencv_example with jpegs
        // -0.07497389 -0.60653263 -0.02834526  0.03355019  0.855165
        // from png's with opencv_examplet
        // -0.34943106  0.76334112  0.00735685  0.00543673 -2.18942544

        // fx: 1321.7091034200228
        //   fy: 1321.332818155101
        //   cx: 672.7447792056566
        //   cy: 319.9444084442252
        //   skew: 0.0
        // k1 -0.506650438282571
        // k2 0.4235342024136259
        // k3 -0.38569969192218445
        // t1: 0.0
        // t2: 0.0
        let distort = new DistortionOptions();
        distort.fx = 1321.7091034200228;
        distort.fy = 1321.332818155101;
        distort.cx = 672.7447792056566;
        distort.cy = 319.9444084442252;
        distort.skew = 0.0;
        distort.k1 = -0.506650438282571;
        distort.k2 = 0.4235342024136259;
        distort.k3 = -0.38569969192218445;
        distort.t1 = 0.0;
        distort.t2 = 0.0;

        let transform = new DistortionOffsetOptions();
        transform.x = -23.4;
        transform.y = 19.2375;
        transform.x_scale = 0.6;
        transform.y_scale = 0.3375;

        let buffer = undistort_data(message.pixels, message.width, message.height, distort, transform);

        parentPort.postMessage({ action: 'undistorted', data: buffer });
        // message.pixels = buffer.buffer;

        let opt = new ToleranceOptions();

        let weight = new WeightageOptions();
        weight.black_dist = 1.0;
        weight.avg = 1.0;
        weight.white_dist = 1.0;
        weight.intersect_dist = 2.0;
        weight.lock();

        let overall = new OverallOptions();
        overall.view_range = 7;
        overall.pre_corners = 100;
        overall.valid_proximity = 10.0;
        overall.post_corners = 30;

        let a = detect_corners(message.pixels, message.width, message.height, overall, opt, weight);
        let object = [];
        for (let index = 0; index < a.length; index++) {
            const element = a[index];
            object.push({ x: element.x, y: element.y });
            // console.log(element.x, element.y);
        }
        parentPort.postMessage({ action: 'located', data: object });

    }
});