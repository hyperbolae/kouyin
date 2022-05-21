use wasm_bindgen::prelude::*;
use web_sys::AudioBuffer;

#[wasm_bindgen]
extern {
    // By default it appears that println! does not yield a log line in the browser,
    // so we will opt to reuse `console.log`
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[wasm_bindgen]
pub fn analyse_audio(buffer: AudioBuffer) {
    log(format!("Loaded an AudioBuffer with a sample rate of {}", buffer.sample_rate()).as_str());
}
