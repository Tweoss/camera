const Canvas = require('canvas');
const fs = require('fs');
const { JSDOM } = require('jsdom');
const { Worker } = require('worker_threads');
const worker = new Worker('./worker.js');

worker.once('message', (message) => {
    console.log(message);
});
worker.postMessage('message');


const base_url = 'cam1.local:5000/camera/',
    ws_url = 'ws://' + base_url,
    camera_name = 'cam1';


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
			}, 500);
				
		</script>
	</body>`, { runScripts: "dangerously", pretendToBeVisual: true });
    // capturing canvas to file
    let i = 10;
    setInterval(() => {
        console.log("making new file")
        canvasToFile(dom.window.document.getElementById("canvas"), './images/canvas' + i + '.png');
        i++;
    }, 10000);
});

// creates or overwrites the file at fileName with a png with contents of jsDOMCanvas
function canvasToFile(jsDOMCanvas, fileName) {
    let ctx = jsDOMCanvas.getContext("2d");
    let data = ctx.getImageData(0, 0, jsDOMCanvas.width, jsDOMCanvas.height);
    const out_canvas = Canvas.createCanvas(jsDOMCanvas.width, jsDOMCanvas.height);
    const out_ctx = out_canvas.getContext('2d')
    out_ctx.putImageData(data, 0, 0);
    let buffer = out_canvas.toBuffer('image/png');
    fs.writeFileSync(fileName, buffer);
}