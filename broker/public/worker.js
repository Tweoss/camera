// const cv = require("./opencv_js")
let classifier;


/**
 *  Here we will check from time to time if we can access the OpenCV
 *  functions. We will return in a callback if it has been resolved
 *  well (true) or if there has been a timeout (false).
 */
function waitForOpencv(callbackFn, waitTimeMs = 30000, stepTimeMs = 100) {
    if (cv.Mat && cv.CascadeClassifier) {
        // if (cv.Mat) {
        callbackFn(true)
    }

    let timeSpentMs = 0
    const interval = setInterval(() => {
        const limitReached = timeSpentMs > waitTimeMs
            // if (cv.Mat || limitReached) {
        if ((cv.Mat && cv.CascadeClassifier) || limitReached) {
            clearInterval(interval)
            return callbackFn(!limitReached)
        } else {
            timeSpentMs += stepTimeMs
        }
    }, stepTimeMs)
}

/**
 * This loads the classifier xml file.
 */
function createFileFromUrl(path, url, callback) {
    let request = new XMLHttpRequest();
    request.open('GET', url, true);
    request.responseType = 'arraybuffer';
    request.onload = function(ev) {
        if (request.readyState === 4) {
            if (request.status === 200) {
                let data = new Uint8Array(request.response);
                cv.FS_createDataFile('/', path, data, true, false, false);
                callback();
            } else {
                self.printError('Failed to load ' + url + ' status: ' + request.status);
            }
        }
    };
    request.send();
};

/**
 * This exists to capture all the events that are thrown out of the worker
 * into the worker. Without this, there would be no communication possible
 * with our project.
 */
onmessage = function(e) {
    switch (e.data.msg) {
        case 'load':
            {
                // Import Webassembly script
                self.importScripts('./opencv_public.js')
                // self.importScripts('./opencv_3_4_custom_O3.js')
                waitForOpencv(function(success) {
                    // console.log(cv.CascadeClassifier, cv.Mat)

                    if (success) {
                        classifier = new cv.CascadeClassifier();
                        let faceCascadeFile = './haarcascade_frontalface_default.xml';
                        createFileFromUrl(faceCascadeFile, faceCascadeFile, () => {
                            classifier.load(faceCascadeFile)
                        });
                        console.log("Loaded opencv")
                        postMessage({ msg: 'Loaded' })
                    } else throw new Error('Error on loading OpenCV')
                })
                break
            }
        case 'data':

            console.log("Processing data")
            let dst = new cv.Mat(),
                gray = new cv.Mat(),
                faces = new cv.RectVector(),
                array = new Uint8ClampedArray(e.data.data),
                imgData = new ImageData(array, e.data.width, e.data.height);
            // msize = new cv.Size(0, 0);
            dst = cv.matFromImageData(imgData);
            // dst.create(e.data.height, e.data.width, cv.CV_8UC4)
            cv.cvtColor(dst, gray, cv.COLOR_RGBA2GRAY, 0);
            // classifier.detectMultiScale(gray, faces, 1.1, 3, 0);
            let faces_object = [];
            try {
                classifier.detectMultiScale(gray, faces, 1.1, 3, 0);
                // postMessage({ msg: 'Gray', gray: gray.data8S })
                console.log("faces from cv length = ", faces.size())
                    // console.log("gray = ", gray.data, "faces from cv length = ", faces.size())
                for (let i = 0; i < faces.size(); ++i) {
                    let face = faces.get(i);
                    faces_object.push({ x: face.x, y: face.y, width: face.width, height: face.height })
                        // paint_context.beginPath();
                        // paint_context.rect(face.x, face.y, face.x + face.width, face.y + face.height);
                        // paint_context.stroke();
                }
            } catch (error) {
                console.log("classifier error: ", error)
            } finally {
                dst.delete();
                gray.delete();
                faces.delete();
            }
            postMessage({ msg: 'Processed', faces: faces_object })
            break
        default:
            break
    }
}


// // importScripts("jsonfn.min.js");
// // importScripts("opencv.js");
// // importScripts("https://docs.opencv.org/4.5.2/opencv.js");
// importScripts("opencv.js");



// cv['onRuntimeInitialized'] = () => {
//     console.log(cv.Mat())
// }

// const JSONfn_parse = function(str, date2obj) {

//     var iso8061 = date2obj ? /^(\d{4})-(\d{2})-(\d{2})T(\d{2}):(\d{2}):(\d{2}(?:\.\d*)?)Z$/ : false;

//     return JSON.parse(str, function(key, value) {
//         var prefix;

//         if (typeof value != 'string') {
//             return value;
//         }
//         if (value.length < 8) {
//             return value;
//         }

//         prefix = value.substring(0, 8);

//         if (iso8061 && value.match(iso8061)) {
//             return new Date(value);
//         }
//         if (prefix === 'function') {
//             return eval('(' + value + ')');
//         }
//         if (prefix === '_PxEgEr_') {
//             return eval(value.slice(8));
//         }
//         if (prefix === '_NuFrRa_') {
//             return eval(value.slice(8));
//         }

//         return value;
//     });
// };




// console.log("SUP")

// // const OPENCV_URL = 'opencv.js';
// // const loadOpenCv = function(onloadCallback) {
// //     let script = document.createElement('script');
// //     script.setAttribute('async', '');
// //     script.setAttribute('type', 'text/javascript');
// //     script.addEventListener('load', async() => {
// //         if (cv.getBuildInformation) {
// //             console.log(cv.getBuildInformation());
// //             onloadCallback();
// //         } else {
// //             // WASM
// //             if (cv instanceof Promise) {
// //                 cv = await cv;
// //                 console.log(cv.getBuildInformation());
// //                 onloadCallback();
// //             } else {
// //                     console.log(cv.getBuildInformation());
// //                     onloadCallback();
// //                 }
// //             }
// //         }
// //     });
// //     script.addEventListener('error', () => {
// //         self.printError('Failed to load ' + OPENCV_URL);
// //     });
// //     script.src = OPENCV_URL;
// //     let node = document.getElementsByTagName('script')[0];
// //     node.parentNode.insertBefore(script, node);
// // };

// // loadOpenCv(() => {

// // cv['onRuntimeInitialized'] = () => {



// let dst = null,
//     gray = null,
//     faces = null,
//     classifier = null,
//     cvtColor = null,
//     CV_8uC4 = null,
//     COLOR_RGBA2GRAY = null;
// let faceCascadeFile = './haarcascade_frontalface_default.xml'; // path to xml
// // use createFileFromUrl to "pre-build" the xml

// // onmessage = function(e) {


// // }
// // );


// function waitForOpencv(callbackFn, waitTimeMs = 300000, stepTimeMs = 1000) {
//     if (cv.Mat) callbackFn(true)
//     let timeSpentMs = 0
//     const interval = setInterval(() => {
//         const limitReached = timeSpentMs > waitTimeMs
//         if (cv.Mat || limitReached) {
//             clearInterval(interval)
//             return callbackFn(!limitReached)
//         } else {
//             console.log("waiting")
//             timeSpentMs += stepTimeMs
//         }
//     }, stepTimeMs)
// }

// /**
//  * This exists to capture all the events that are thrown out of the worker
//  * into the worker. Without this, there would be no communication possible
//  * with the project.
//  */
// onmessage = function(e) {
//     switch (e.data.msg) {
//         case 'load':
//             {
//                 // Import Webassembly script
//                 self.importScripts('./opencv.js')
//                 waitForOpencv(function(success) {
//                     if (success) {
//                         const createFileFromUrl = function(path, url, callback) {
//                             let request = new XMLHttpRequest();
//                             request.open('GET', url, true);
//                             request.responseType = 'arraybuffer';
//                             request.onload = function(ev) {
//                                 if (request.readyState === 4) {
//                                     if (request.status === 200) {
//                                         let data = new Uint8Array(request.response);
//                                         cv.FS_createDataFile('/', path, data, true, false, false);
//                                         callback();
//                                     } else {
//                                         self.printError('Failed to load ' + url + ' status: ' + request.status);
//                                     }
//                                 }
//                             };
//                             request.send();
//                         };
//                         createFileFromUrl(faceCascadeFile, faceCascadeFile, () => {
//                             console.log(classifier.load(faceCascadeFile)); // in the callback, load the cascade from file 
//                         });
//                         postMessage({ msg: 'Loaded' });

//                     } else throw new Error('Error on loading OpenCV')
//                 })
//                 break
//             }
//         case 'data':
//             {
//                 dst = new cv.Mat(e.data.height, e.data.width, cv.CV_8UC4),
//                 gray = new cv.Mat(),
//                 faces = new cv.RectVector();
//                 // if (e.data.msg === 'init') {
//                 // 	let d = JSONfn_parse(e.data.json);
//                 // 	[classifier, dst, gray, faces, cvtColor, CV_8uC4, COLOR_RGBA2GRAY] = [d.classifer, d.dst, d.gray, d.faces, d.cvtColor, d.cv_8uc4, d.color_rgba2gray];
//                 // 	console.log(d)
//                 // 	console.log(dst)

//                 dst.create(e.data.height, e.data.width, cv.CV_8uC4)
//                 dst.data.set(e.data.data);
//                 // src.copyTo(dst);
//                 cv.cvtColor(dst, gray, cv.COLOR_RGBA2GRAY, 0);
//                 // detect faces.
//                 classifier.detectMultiScale(gray, faces, 1.1, 3, 0);
//                 postMessage({ msg: 'Processed', faces: faces })
//                 // console.log('Message received from main script');
//                 // var workerResult = 'Result: ' + (e.data[0] * e.data[1]);
//                 // console.log('Posting message back to main script');
//                 // postMessage(workerResult);
//                 break
//             }

//         default:
//             break
//     }
// }