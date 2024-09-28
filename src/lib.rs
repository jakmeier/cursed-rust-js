use std::vec;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod jitter;
// Without this, println doesn't work,
mod web_utils;

#[wasm_bindgen]
pub struct MyRoboDetection {
    // all detected events
    events: Vec<Event>,
    // isBot() results that were explicitly saved
    saved_results: Vec<RoboDetectionOutput>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Event {
    pub timestamp: u32,
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct RoboDetectionOutput {
    pub jitter: f32,
    #[wasm_bindgen(js_name = humanScore)]
    pub human_score: f32,
    result_text: String,
}

#[wasm_bindgen]
impl MyRoboDetection {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            events: vec![],
            saved_results: vec![],
        }
    }

    #[wasm_bindgen(js_name = fromEvents)]
    pub fn from_events(events: Vec<Event>) -> Self {
        Self {
            events,
            ..Self::new()
        }
    }

    #[wasm_bindgen(js_name = addEvent)]
    pub fn add_event(&mut self, timestamp: u32, event: web_sys::MouseEvent) -> Result<(), JsValue> {
        let new_event = Event {
            timestamp,
            x: event.client_x(),
            y: event.client_y(),
        };
        // println!("got {new_event:?}");
        self.events.push(new_event);
        Ok(())
    }

    #[wasm_bindgen(js_name = saveResult)]
    pub fn save_result(&mut self, result: RoboDetectionOutput) {
        self.saved_results.push(result);
    }

    #[wasm_bindgen(js_name = saveBorrowedResult)]
    pub fn save_borrowed_result(&mut self, result: &RoboDetectionOutput) {
        self.saved_results.push(result.clone());
    }

    #[wasm_bindgen(getter)]
    pub fn events(&self) -> Vec<Event> {
        self.events.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn results(&self) -> Vec<RoboDetectionOutput> {
        self.saved_results.clone()
    }

    #[wasm_bindgen(js_name = isBot)]
    pub fn is_bot(&self) -> RoboDetectionOutput {
        let jitter = self.jitter();
        let human_score = (jitter / 1000.0).min(1.0);
        let result_text = if human_score < 0.5 { "Robot" } else { "Human" }.to_owned();
        RoboDetectionOutput {
            jitter,
            human_score,
            result_text,
        }
    }
}

#[wasm_bindgen]
impl RoboDetectionOutput {
    pub fn text(&self) -> String {
        self.result_text.clone()
    }
}
