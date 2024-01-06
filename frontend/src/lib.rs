use chip8::{opcode::OpCode, Chip8};

use std::collections::VecDeque;

use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Uint8Array, CanvasRenderingContext2d, HtmlCanvasElement, KeyboardEvent};

#[wasm_bindgen]
pub struct Emulator {
    chip8: Chip8,
    ctx: CanvasRenderingContext2d,
    insts: VecDeque<OpCode>,
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
            insts: VecDeque::with_capacity(32),
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

        self.insts.push_back(self.chip8.last_executed_instruction);

        if self.insts.len() >= 32 {
            self.insts.pop_front();
        }

        // Display the last executed instructions.
        for (n, i) in self.insts.iter().enumerate() {
            let op_code = format!("{i:?}");
            let line_height_position = 15. * n as f64;

            _ = self.ctx.fill_text(&op_code, 10., line_height_position);
        }
    }
}
