import {greet, AudioProcessor} from "tune-master-wasm";
import {memory} from "tune-master-wasm/tune_master_wasm_bg.wasm";



const audioProcessor = AudioProcessor.new();

const audioCtx = new window.AudioContext();

let analyser, binCount, fftData, waveformData;

const canvas = document.getElementById("canvas");
canvas.height = window.innerHeight;
canvas.width = window.innerWidth;

const ctx = canvas.getContext("2d");

const startButton = document.getElementById("start-button");


startButton.addEventListener("click", e => {
    console.log("HIHIHI")
    audioProcessor.init();
    // audioProcessor.tick();
    let fftDataPtr = audioProcessor.get_fft_data()
    // console.log("fftLoc: ", fft_loc);
    let bufferSize = audioProcessor.get_buffer_size();
    const fft_data = new Uint8Array(memory.buffer, fftDataPtr, bufferSize);
    console.log(fft_data)
})

let p = new Path2D("50")



const setup = async () => {

    analyser = audioCtx.createAnalyser();
    analyser.fftSize = 	8192;
    binCount = analyser.frequencyBinCount
    fftData = new Uint8Array(binCount);
    waveformData = new Float32Array(binCount);

    const stream = await navigator.mediaDevices.getUserMedia({audio: true});
    const source = audioCtx.createMediaStreamSource(stream);
    source.connect(analyser);

    if (audioCtx.state === "running") {
        setInterval(updateData, 20);
    }
}