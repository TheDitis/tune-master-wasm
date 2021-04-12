mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{
    AudioContext,
    OscillatorNode,
    OscillatorType,
    Navigator,
    AnalyserNode
};

const BUFFER_SIZE: usize = 256;
const BIN_COUNT: usize = BUFFER_SIZE / 2;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, tune-master-wasm!");
}



#[wasm_bindgen]
pub struct AudioProcessor {
    ctx: AudioContext,
    analyser: AnalyserNode,
    buffer_size: usize,
    fft_data: [u8; BIN_COUNT],
    wav_data: Vec<u32>,
}


#[wasm_bindgen]
impl AudioProcessor {
    pub fn new() -> AudioProcessor {
        // let BUFFER_SIZE = 8192;
        let ctx = AudioContext::new().unwrap();
        let analyser = ctx.create_analyser().unwrap();
        analyser.set_fft_size(BUFFER_SIZE as u32);
        AudioProcessor {
            ctx,
            analyser,
            buffer_size: BUFFER_SIZE,
            fft_data: [0; BIN_COUNT],
            wav_data: vec![],
        }
    }

    pub fn init(&self) {
        // utils::set_panic_hook();
        // panic!("message!")
        let ctx = &self.ctx;
        let analyser = &self.analyser;

        let osc1 = ctx.create_oscillator().unwrap();
        let gain = ctx.create_gain().unwrap();

        osc1.set_type(OscillatorType::Sine);
        osc1.frequency().set_value(200.0);

        gain.gain().set_value(0.5);
        osc1.connect_with_audio_node(&analyser);
        analyser.connect_with_audio_node(&gain);
        gain.connect_with_audio_node(&ctx.destination());

        osc1.start();
        self.ctx.resume();
        self.tick();
        log!("stuff set up");


    }

    pub fn tick(&self) {
        let analyser = &self.analyser;
        let size: usize = analyser.frequency_bin_count() as usize;
        // let mut freq_data: Vec<u8> = vec![0; size];
        let mut freq_data: [u8; BIN_COUNT] = [0; BIN_COUNT];
        // let &mut freq_data_slice = freq_data[..];
        analyser.get_byte_frequency_data(&mut freq_data);
        // log!("{:?}", freq_data);
        // self.fft_data = freq_data;
        // self.fft_data = freq_data.iter().cloned().collect();
    }

    pub fn get_fft_data(&self) -> *const u8 {
        // self.fft_data.as_ptr()
        // log!("{:?}", self.fft_data);
        self.fft_data.as_ptr()
    }

    pub fn get_buffer_size(&self) -> usize {
        BUFFER_SIZE
    }
}