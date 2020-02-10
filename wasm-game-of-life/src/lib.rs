mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(s: u32);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Cell {
    DEATH = 0,
    LIVE = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width,
            height,
            cells: (0..height * width)
                .map(|i| {
                    if i % 2 == 0 || i % 7 == 0 {
                        Cell::LIVE
                    } else {
                        Cell::DEATH
                    }
                })
                .collect(),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    fn get_cell_index(&self, y: u32, x: u32) -> usize {
        (y * self.width + x) as usize
    }

    fn get_nb_live_surrounding_cells(&self, y: u32, x: u32) -> u8 {
        let mut count = 0;

        let north = if y == 0 { self.height - 1 } else { y - 1 };
        let south = if y == self.height - 1 { 0 } else { y + 1 };
        let west = if x == 0 { self.width - 1 } else { x - 1 };
        let east = if x == self.width - 1 { 0 } else { x + 1 };

        let nw = self.get_cell_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_cell_index(north, x);
        count += self.cells[n] as u8;

        let ne = self.get_cell_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_cell_index(y, west);
        count += self.cells[w] as u8;

        let e = self.get_cell_index(y, east);
        count += self.cells[e] as u8;

        let sw = self.get_cell_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_cell_index(south, x);
        count += self.cells[s] as u8;

        let se = self.get_cell_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn next_step(&mut self) {
        let mut new_cell_vec = self.cells.clone();

       
        self.cells = new_cell_vec;
    }
}
