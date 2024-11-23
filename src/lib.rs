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
pub fn analyse_audio(buffer: AudioBuffer) -> Vec<f32> {
    log(format!("Loaded an AudioBuffer with a sample rate of {}", buffer.sample_rate()).as_str());
    let data: Vec<f32> = buffer.get_channel_data(0).ok().unwrap();
    let mut count = 0;
    let mut vec = Vec::new();

    for chunk in data.chunks(250) {
        let mut total: f32 = 0.0;
        for sample in chunk {
            total += sample;
        }

        count = count + 1;
        vec.push(total / 250.0);
    }
    return vec;
}
