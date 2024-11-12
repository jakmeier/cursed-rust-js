use std::vec;

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

mod jitter;
// BUG 0
// Without this, println doesn't work,
mod web_utils;

// BUG 3
// Silent overflow
pub type Timestamp = u32;

// BUG 3.1
// can't convert 1731071638989 to BigInt
// pub type Timestamp = u64;

// BUG 3 solution
// JS number only converts cleanly to f64
// pub type Timestamp = f64;

#[wasm_bindgen]
pub struct MyBotDetection {
    // all detected events
    events: Vec<Event>,
    // isBot() results that were explicitly saved
    saved_results: Vec<BotDetectionOutput>,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct Event {
    pub timestamp: Timestamp,
    pub coordinate: Coordinate,
}

#[derive(Debug, Clone, Copy)]
#[wasm_bindgen]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct BotDetectionOutput {
    #[wasm_bindgen(js_name = humanScore)]
    pub human_score: f32,
    pub timestamp: Timestamp,
    result_text: String,
}

#[wasm_bindgen]
impl MyBotDetection {
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
    pub fn add_event(
        &mut self,
        timestamp: Timestamp,
        event: web_sys::MouseEvent,
    ) -> Result<(), JsValue> {
        let new_event = Event {
            timestamp,
            coordinate: Coordinate {
                x: event.client_x(),
                y: event.client_y(),
            },
        };
        // println!("got {new_event:?}");
        self.events.push(new_event);
        Ok(())
    }

    #[wasm_bindgen(js_name = saveResult)]
    pub fn save_result(&mut self, result: BotDetectionOutput) {
        self.saved_results.push(result);
    }

    #[wasm_bindgen(js_name = saveBorrowedResult)]
    pub fn save_borrowed_result(&mut self, result: &BotDetectionOutput) {
        self.saved_results.push(result.clone());
    }

    // BUG 1 part of solution
    pub fn events(&self, start: usize, end: usize) -> Vec<Event> {
        self.events[start..end].to_vec()
    }

    pub fn num_events(&self) -> usize {
        self.events.len()
    }

    #[wasm_bindgen(getter)]
    pub fn results(&self) -> Vec<BotDetectionOutput> {
        self.saved_results.clone()
    }

    #[wasm_bindgen(js_name = isBot)]
    pub fn is_bot(&self) -> BotDetectionOutput {
        let human_score = self.jitter();
        let result_text = if human_score < 0.5 { "Robot" } else { "Human" }.to_owned();
        let timestamp = self.events.last().map(|e| e.timestamp).unwrap_or_default();
        BotDetectionOutput {
            timestamp,
            human_score,
            result_text,
        }
    }
}

#[wasm_bindgen]
impl BotDetectionOutput {
    pub fn text(&self) -> String {
        self.result_text.clone()
    }
}

#[wasm_bindgen]
impl Coordinate {
    #[wasm_bindgen(constructor)]
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
