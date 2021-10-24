const rust = import("./pkg");

const canvas = document.getElementById("arDisplay");

rust.then(m => {
    let constraints = { video: { ideal: "environment" } };

    let mediaDevices = navigator.mediaDevices;

    if (mediaDevices === undefined) {
        alert("no media devices");
    } else {
        mediaDevices.getUserMedia(constraints).then(d => {
            console.log("found user media", d);
        }).catch(error => {
            console.log("failure to find user media", error);
        })
    }


    let client = m.start();

    const FPS_THROTTLE = 1000.0 / 60.0;
    const initialTime = Date.now();
    let lastDrawTime = -1;

    function render() {
        window.requestAnimationFrame(render);
        const currTime = Date.now();

        if (currTime >= lastDrawTime + FPS_THROTTLE) {
            lastDrawTime = currTime;

            let elapsedTime = currTime - initialTime;
            client.update(elapsedTime, window.innerWidth, window.innerHeight)
            client.render();
        }
    }
    setGlSize(window.innerWidth, window.innerHeight);
    render()

}).catch(console.error);


window.onresize = () => {
    setGlSize(window.innerWidth, window.innerHeight);
}

const setGlSize = (width, height) => {
    canvas.width = width;
    canvas.clientWidth = width;
    canvas.style.width = width;

    canvas.height = height;
    canvas.clientHeight = height;
    canvas.style.height = height;
}
