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

let classifier;
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
                // console.log("gray = ", gray.data, "faces from cv length = ", faces.size())
                for (let i = 0; i < faces.size(); ++i) {
                    let face = faces.get(i);
                    faces_object.push({ x: face.x, y: face.y, width: face.width, height: face.height })
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