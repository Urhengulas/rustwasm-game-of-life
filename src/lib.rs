mod utils;

use rand::prelude::*;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

/// Private methods
impl Universe {
	fn get_index(&self, row: u32, col: u32) -> usize {
		(row * self.width + col) as usize
	}
	fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
		let mut count = 0;
		for delta_row in [self.height - 1, 0, 1].iter().cloned() {
			for delta_col in [self.width - 1, 0, 1].iter().cloned() {
				if delta_row == 0 && delta_col == 0 {
					continue;
				}

				let neighbor_row = (row + delta_row) % self.height;
				let neighbor_col = (column + delta_col) % self.width;
				let idx = self.get_index(neighbor_row, neighbor_col);
				count += self.cells[idx] as u8;
			}
		}
		count
	}

	/// Get the dead and alive values of the entire universe.
	pub fn get_cells(&self) -> &[Cell] {
		&self.cells
	}

	/// Set cells to be alive in a universe by passing the row and column of each cell as an array.
	pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
		for (row, col) in cells.iter().cloned() {
			let idx = self.get_index(row, col);
			self.cells[idx] = Cell::Alive;
		}
	}
}

/// Public methods, exported to JavaScript
#[wasm_bindgen]
impl Universe {
	#[wasm_bindgen(constructor)]
	/// Constructs a new `Universe`
	pub fn new() -> Universe {
		let width = 64;
		let height = 64;

		let mut rng = thread_rng();
		let cells = (0..(width * height))
			.map(|_| if rng.gen() { Cell::Alive } else { Cell::Dead })
			.collect();

		Universe {
			width,
			height,
			cells,
		}
	}

	/// Moves `Universe` one step into the future
	pub fn tick(&mut self) {
		let mut next = self.cells.clone();

		for row in 0..self.height {
			for col in 0..self.width {
				let idx = self.get_index(row, col);
				let cell = self.cells[idx];
				let live_neighbors = self.live_neighbor_count(row, col);

				let next_cell = match (cell, live_neighbors) {
					// Rule 1: Cell::Alive AND (live_neighbors < 2) -> dies (underpopulation)
					(Cell::Alive, x) if x < 2 => Cell::Dead,
					// Rule 2: Cell::Alive AND (2 <= live_neighbors <= 3) -> alive
					(Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
					// Rule 3: Cell::Alive AND (live_neighbors > 3) -> dies (overpopulation)
					(Cell::Alive, x) if x > 3 => Cell::Dead,
					// Rule 4: Cell::Dead AND (live_neighbors == 3) -> alive (reproduction)
					(Cell::Dead, 3) => Cell::Alive,
					// All other cells remain in the same state
					(otherwise, _) => otherwise,
				};

				next[idx] = next_cell;
			}
		}

		self.cells = next;
	}

	/// Get the width of the universe
	#[wasm_bindgen(method, getter)]
	pub fn width(&self) -> u32 {
		self.width
	}

	/// Set the width of the universe.
	///
	/// Resets all cells to the dead state.
	#[wasm_bindgen(method, setter)]
	pub fn set_width(&mut self, width: u32) {
		self.width = width;
		self.cells = (0..(width * self.height)).map(|_| Cell::Dead).collect();
	}

	/// Get the height of the universe
	#[wasm_bindgen(method, getter)]
	pub fn height(&self) -> u32 {
		self.height
	}

	/// Set the height of the universe.
	///
	/// Resets all cells to the dead state.
	#[wasm_bindgen(method, setter)]
	pub fn set_height(&mut self, height: u32) {
		self.height = height;
		self.cells = (0..(self.width * height)).map(|_| Cell::Dead).collect();
	}

	/// Get a pointer to the array of cells
	pub fn cells_ptr(&self) -> *const Cell {
		self.cells.as_ptr()
	}
}
