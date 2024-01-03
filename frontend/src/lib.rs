use chip8::Chip8;

use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Uint8Array, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct Emulator {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
}

#[wasm_bindgen]
impl Emulator {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let ctx = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Emulator {
            chip8: Chip8::new(),
            ctx,
        }
    }

    #[wasm_bindgen]
    pub fn cycle(&mut self) {
        self.chip8.cycle();
    }

    #[wasm_bindgen]
    pub fn keypress(&mut self, evt: KeyboardEvent, pressed: bool) {
        let key = evt.key();

        if let Some(k) = Chip8::key2btn(&key) {
            self.chip8.keypress(k, pressed);
        }
    }

    #[wasm_bindgen]
    pub fn load_game(&mut self, data: Uint8Array) {
        self.chip8.load(&data.to_vec());
    }

    #[wasm_bindgen]
    pub fn reset(&mut self) {
        self.chip8.reset();
    }

    #[wasm_bindgen]
    pub fn draw_screen(&mut self, scale: usize) {
        self.chip8.pixel_cooridinates().iter().for_each(|(x, y)| {
            self.ctx.fill_rect(
                (y * scale) as f64,
                (x * scale) as f64,
                scale as f64,
                scale as f64,
            );
        });
    }
}
