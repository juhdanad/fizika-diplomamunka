use once_cell::sync::Lazy;
use once_cell::unsync::OnceCell;
use std::cell::RefCell;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct PhotonData {
	n_max: usize,
	in_dist: Vec<f64>,
	p_merge: Vec<f64>,
	total_chance: OnceCell<Vec<f64>>,
	total_chance_no_ones: OnceCell<Vec<f64>>,
	output: OnceCell<Vec<f64>>,
	remain: OnceCell<Vec<f64>>,
	missing: OnceCell<Vec<f64>>,
}

impl PhotonData {
	pub fn new() -> PhotonData {
		PhotonData::new_with_data(vec![0_f64; 1], vec![0_f64; 1])
	}
	pub fn new_with_data(in_dist: Vec<f64>, p_merge: Vec<f64>) -> PhotonData {
		let n_max_plus_1 = if in_dist.len() < p_merge.len() {
			in_dist.len()
		} else {
			p_merge.len()
		};
		if n_max_plus_1 < 1 {
			panic!("Empty array in PhotonData constructor");
		}
		PhotonData {
			n_max: n_max_plus_1 - 1,
			in_dist: in_dist,
			p_merge: p_merge,
			total_chance: OnceCell::new(),
			total_chance_no_ones: OnceCell::new(),
			output: OnceCell::new(),
			remain: OnceCell::new(),
			missing: OnceCell::new(),
		}
	}

	fn get_total_chance(&self) -> &Vec<f64> {
		self.total_chance.get_or_init(|| {
			let mut total_chance = vec![0_f64; self.n_max + 1];
			total_chance[0] = 1.0;
			for k in 1..=self.n_max {
				for n in k..=self.n_max {
					total_chance[n] += self.p_merge[k] * total_chance[n - k];
				}
			}
			total_chance
		})
	}

	fn get_total_chance_no_ones(&self) -> &Vec<f64> {
		self.total_chance_no_ones.get_or_init(|| {
			let mut total_chance_no_ones = vec![0_f64; self.n_max + 1];
			total_chance_no_ones[0] = 1.0;
			for k in 2..=self.n_max {
				for n in k..=self.n_max {
					total_chance_no_ones[n] += self.p_merge[k] * total_chance_no_ones[n - k];
				}
			}
			total_chance_no_ones
		})
	}

	fn get_output(&self) -> &Vec<f64> {
		self.output.get_or_init(|| {
			let total_chance = self.get_total_chance();
			let mut partial_output = vec![0_f64; self.n_max + 1];
			let mut output = vec![0_f64; self.n_max + 1];
			for k in 1..=self.n_max {
				for n in 1..k {
					partial_output[n] = 0.0;
				}
				for n in k..=self.n_max {
					partial_output[n] =
						self.p_merge[k] * (partial_output[n - k] + total_chance[n - k]);
				}
				//transform to probabilities/intensities
				for n in 1..=self.n_max {
					partial_output[n] = partial_output[n] / total_chance[n];
					//convolution to get output
					output[k] += partial_output[n] * self.in_dist[n];
				}
			}
			output
		})
	}

	fn get_remain(&self) -> &Vec<f64> {
		self.remain.get_or_init(|| {
			let total_chance = self.get_total_chance();
			let total_chance_no_ones = self.get_total_chance_no_ones();
			let mut remain = vec![0_f64; self.n_max + 1];
			if self.n_max == 0 {
				remain[0] = self.in_dist[0];
				return remain;
			}
			for n in 0..=self.n_max {
				let mut p_pow = 1_f64;
				let multiplier = self.in_dist[n] / total_chance[n];
				for k in 0..=n {
					remain[k] += multiplier * p_pow * total_chance_no_ones[n - k];
					p_pow *= self.p_merge[1];
				}
			}
			remain
		})
	}

	fn get_missing(&self) -> &Vec<f64> {
		self.missing.get_or_init(|| {
			let total_chance = self.get_total_chance();
			let total_chance_no_ones = self.get_total_chance_no_ones();
			let mut missing = vec![0_f64; self.n_max + 1];
			if self.n_max == 0 {
				missing[0] = self.in_dist[0];
				return missing;
			}
			for n in 0..=self.n_max {
				let mut p_pow = 1_f64;
				let multiplier = self.in_dist[n] / total_chance[n];
				for k in (0..=n).rev() {
					missing[k] += multiplier * p_pow * total_chance_no_ones[k];
					p_pow *= self.p_merge[1];
				}
			}
			missing
		})
	}
}

static DATA: Lazy<Mutex<RefCell<PhotonData>>> =
	Lazy::new(|| Mutex::new(RefCell::new(PhotonData::new())));

#[wasm_bindgen]
pub fn set_input(in_dist: Vec<f64>, p_merge: Vec<f64>) {
	DATA.lock()
		.unwrap()
		.replace(PhotonData::new_with_data(in_dist, p_merge));
}

#[wasm_bindgen]
pub fn get_in_dist() -> Vec<f64> {
	DATA.lock().unwrap().borrow().in_dist.clone()
}

#[wasm_bindgen]
pub fn get_p_merge() -> Vec<f64> {
	DATA.lock().unwrap().borrow().p_merge.clone()
}

#[wasm_bindgen]
pub fn get_total_chance() -> Vec<f64> {
	DATA.lock().unwrap().borrow().get_total_chance().clone()
}

#[wasm_bindgen]
pub fn get_total_chance_no_ones() -> Vec<f64> {
	DATA.lock()
		.unwrap()
		.borrow()
		.get_total_chance_no_ones()
		.clone()
}

#[wasm_bindgen]
pub fn get_output() -> Vec<f64> {
	DATA.lock().unwrap().borrow().get_output().clone()
}

#[wasm_bindgen]
pub fn get_remain() -> Vec<f64> {
	DATA.lock().unwrap().borrow().get_remain().clone()
}

#[wasm_bindgen]
pub fn get_missing() -> Vec<f64> {
	DATA.lock().unwrap().borrow().get_missing().clone()
}

#[cfg(feature = "console_error_panic_hook")]
extern crate console_error_panic_hook;
#[cfg(feature = "console_error_panic_hook")]
use std::panic;

#[cfg(feature = "console_error_panic_hook")]
#[wasm_bindgen]
pub fn set_panic_hook() {
	panic::set_hook(Box::new(console_error_panic_hook::hook));
}
