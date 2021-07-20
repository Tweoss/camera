const Canvas = require('canvas');
const fs = require('fs');
const { JSDOM } = require('jsdom');
const { Worker } = require('worker_threads');
const worker = new Worker('./worker.js');


const base_url = 'vachuska.com/camera/?id=Menlo&t=25b46b54-b22a-4c62-aac9-741802c8c169',
    ws_url = 'wss://' + base_url,
    camera_name = 'Menlo';
// const base_url = 'cam1.local:5000/camera/',
//     ws_url = 'ws://' + base_url,
//     camera_name = 'cam1';

// read live player in as a string to avoid url, document scope issues
fs.readFile(__dirname + "/lib/http-live-player.js", "utf8", function(err, data) {
    if (err) {
        return console.log(err);
    }

    const dom = new JSDOM(` <body>
	
		<button type="button" onclick="wsavc.playStream()">Start Video</button>
		<button type="button" onclick="wsavc.stopStream()">Stop Video</button>
		<button type="button" onclick="wsavc.disconnect()">Disconnect</button>
		<br/>
		
		
		<!-- provide WSAvcPlayer -->
		<script type="text/javascript">${data};</script>
		<script type="text/javascript">
			var canvas = document.createElement("canvas");
			canvas.id = "canvas";
			document.body.appendChild(canvas);
			
			// Create h264 player
			var uri = "${ws_url}"
			var wsavc = new WSAvcPlayer(canvas, "2d", 1, 35);
			wsavc.connect(uri, onstatus);
			
			
			//expose instance for button callbacks
			window.wsavc = wsavc;
			
			function onstatus(d) {
				// if (d.data && !d.data.err) {
					// 	if (d.data.name === 'Pan' || d.data.name === 'Tilt') {
					// 		if (d.data.name === 'Pan') pan = d.data.pos;
					// 		if (d.data.name === 'Tilt') tilt = d.data.pos;
					// 		px = map(pan, panMin, panMax, 0, 305);
					// 		py = map(-tilt, tiltMin, tiltMax, 0, 75);
					// 		$('#position').css('left', px).css('top', py);
					// 		showUIOverlay();
					// 	} else if (d.data.name === 'StartTrack') {
					// 		verifiedStartTracking()
					// 	}
				// }
			}
			setTimeout(() => {
				wsavc.playStream('${camera_name}');
			}, 1000);
				
		</script>
	</body>`, { runScripts: "dangerously", pretendToBeVisual: true });
    const canvas = dom.window.document.getElementById('canvas');
    // capturing canvas to file
    let i = 10;
    setTimeout(() => {
        console.log("making new file")
        i++;
        processCanvas(canvas, worker);
        canvasToFile(canvas, './images/canvas.png');
    }, 5000);
    worker.on('message', (message) => {
        let imgData = new Canvas.ImageData(Uint8ClampedArray.from(message), canvas.width, canvas.height);
        imgDataToFile(imgData, './images/canvas_wasm.png');
    });
});

// creates or overwrites the file at fileName with a png with contents of jsDOMCanvas
function canvasToFile(jsDOMCanvas, fileName) {
    let ctx = jsDOMCanvas.getContext("2d");
    let data = ctx.getImageData(0, 0, jsDOMCanvas.width, jsDOMCanvas.height);
    imgDataToFile(data, fileName);
}

// auxiliary function to place imgData in a png file
function imgDataToFile(imgData, fileName) {
    const out_canvas = Canvas.createCanvas(imgData.width, imgData.height);
    const out_ctx = out_canvas.getContext('2d')
    out_ctx.putImageData(imgData, 0, 0);
    let buffer = out_canvas.toBuffer('image/png');
    fs.writeFileSync(fileName, buffer);
}

// process canvas with worker
function processCanvas(canvas, worker) {
    let imgData = canvas.getContext('2d').getImageData(0, 0, canvas.width, canvas.height);
    const out_canvas = Canvas.createCanvas(imgData.width, imgData.height);
    const out_ctx = out_canvas.getContext('2d')
    out_ctx.putImageData(imgData, 0, 0);
    const pixels = out_ctx.getImageData(0, 0, canvas.width, canvas.height);
    worker.postMessage({
        action: 'process',
        pixels: pixels.data.buffer,
        width: pixels.width,
        height: pixels.height,
    }, [pixels.data.buffer]);
}