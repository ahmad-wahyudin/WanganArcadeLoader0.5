use std::io::Write;
use std::sync::Mutex;

use crate::*;
const INIT: u8 = 0x10;
const READSTATUS: u8 = 0x20;
const CANCEL: u8 = 0x40;
const EJECT: u8 = 0x80;
const DISPENSECARD: u8 = 0xB0;
const PRINTSETTING: u8 = 0x78;
const READ: u8 = 0x33;
const WRITE: u8 = 0x53;

const TRACK_SIZE: usize = 0x45;

pub static mut CARD_DATA: Mutex<Vec<u8>> = Mutex::new(Vec::new());

unsafe extern "C" fn exec(card_printer: *mut u32) {
	let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
		exec_inner(card_printer);
	}));
	if let Err(e) = result {
		eprintln!("card exec panicked: {:?}", e);
	}
}

fn write_at(card_data: &mut Vec<u8>, offset: usize, src: &[u8]) {
	let needed = offset + src.len();
	if card_data.len() < needed {
		card_data.resize(needed, 0);
	}
	card_data[offset..needed].copy_from_slice(src);
}

fn track_ranges(track: u8) -> Option<Vec<(usize, usize)>> {
	match track {
		0x30 => Some(vec![(0, TRACK_SIZE)]),                       // Track 1
		0x31 => Some(vec![(TRACK_SIZE, TRACK_SIZE)]),              // Track 2
		0x32 => Some(vec![(2 * TRACK_SIZE, TRACK_SIZE)]),          // Track 3
		0x33 => Some(vec![(0, 2 * TRACK_SIZE)]),                   // Track 1+2 (contiguous)
		0x34 => Some(vec![(0, TRACK_SIZE), (2 * TRACK_SIZE, TRACK_SIZE)]), // Track 1+3 (not contiguous)
		0x35 => Some(vec![(TRACK_SIZE, 2 * TRACK_SIZE)]),          // Track 2+3 (contiguous)
		0x36 => Some(vec![(0, 3 * TRACK_SIZE)]),                   // Track 1+2+3
		_ => None,
	}
}

unsafe fn exec_inner(card_printer: *mut u32) {
	let has_command = card_printer.read();
	if has_command == 0 {
		return;
	}
	let request = card_printer.byte_add(0x08).read() as *mut u8;
	let start = request.byte_add(0).read();
	if start != 2 {
		dbg!(start);
		return;
	}
	let count = request.byte_add(1).read();
	if count < 3 {
		eprintln!("card exec: command too short (count={count})");
		return;
	}
	let command = request.byte_add(2).read();
	let mut data = Vec::new();
	for i in 6..count {
		data.push(request.byte_add(i as usize).read())
	}

	match command {
		INIT => card_printer.write(0x00),
		READSTATUS => card_printer.write(0x00),
		DISPENSECARD => {
			let Some(&first) = data.get(0) else {
				eprintln!("card exec: DISPENSECARD missing data");
				return;
			};
			let check = first == 0x32;
			card_printer.write(0x00);
			if check {
				card_printer.byte_add(0x06).write(0x37);
			} else {
				card_printer.byte_add(0x04).write(0x33);
			}
		}
		READ => {
			card_printer.write(0x00);
			card_printer.byte_add(0x04).write(0x31);

			let Some(&mode) = data.get(0) else {
				eprintln!("card exec: READ missing mode byte");
				return;
			};
			if mode == 0x32 {
				// CardCapture check: report "waiting for card" if we have none
				if CARD_DATA.lock().unwrap().len() == 0 {
					card_printer.byte_add(0x04).write(0x30);
					card_printer.byte_add(0x06).write(0x34);
				}
				return;
			}

			let Some(&track) = data.get(2) else {
				eprintln!("card exec: READ missing track byte");
				return;
			};
			let Some(ranges) = track_ranges(track) else {
				eprintln!("card exec: READ unknown track option {track:#x}");
				return;
			};

			let card_data = CARD_DATA.get_mut().unwrap();
			let max_needed = ranges.iter().map(|(off, len)| off + len).max().unwrap_or(0);
			if card_data.len() < max_needed {
				// Not enough stored card data to satisfy this track request
				card_printer.byte_add(0x04).write(0x30);
				card_printer.byte_add(0x06).write(0x34);
				return;
			}

			let write_buf = card_printer.byte_add(0x10).read() as *mut u8;
			if write_buf.is_null() {
				eprintln!("card exec: READ got null write_buf");
				return;
			}
			write_buf.write(0x00);
			write_buf.byte_add(0x04).write(0x33);
			write_buf.byte_add(0x05).write(0x30);
			write_buf.byte_add(0x06).write(0x30);

			let mut idx = 0usize;
			for (off, len) in ranges {
				for b in &card_data[off..off + len] {
					write_buf.byte_add(0x06 + idx).write(*b);
					idx += 1;
				}
			}
		}
		WRITE => {
			let card_data = CARD_DATA.get_mut().unwrap();
			card_printer.write(0x00);
			let Some(&track) = data.get(2) else {
				eprintln!("card exec: WRITE missing track byte");
				return;
			};

			let track_bytes = &data[3.min(data.len())..];
			match track {
				0x30 => write_at(card_data, 0, &track_bytes[..TRACK_SIZE.min(track_bytes.len())]),
				0x31 => write_at(card_data, TRACK_SIZE, &track_bytes[..TRACK_SIZE.min(track_bytes.len())]),
				0x32 => write_at(card_data, 2 * TRACK_SIZE, &track_bytes[..TRACK_SIZE.min(track_bytes.len())]),
				0x33 => write_at(card_data, 0, &track_bytes[..(2 * TRACK_SIZE).min(track_bytes.len())]),
				0x34 => {
					let len = TRACK_SIZE.min(track_bytes.len());
					write_at(card_data, 0, &track_bytes[..len]);
					if track_bytes.len() > TRACK_SIZE {
						let rest = &track_bytes[TRACK_SIZE..];
						write_at(card_data, 2 * TRACK_SIZE, &rest[..TRACK_SIZE.min(rest.len())]);
					}
				}
				0x35 => write_at(card_data, TRACK_SIZE, &track_bytes[..(2 * TRACK_SIZE).min(track_bytes.len())]),
				0x36 => {
					card_data.clear();
					card_data.extend(track_bytes.iter().take(3 * TRACK_SIZE));
				}
				_ => {
					eprintln!("card exec: unknown track combination {track:#x}");
					return;
				}
			}

			_ = std::fs::remove_file("card.bin");
			let mut file = std::fs::File::create("card.bin").unwrap();
			file.write_all(card_data).unwrap();
		}
		CANCEL => card_printer.write(0x00),
		EJECT => {
			CARD_DATA.get_mut().unwrap().clear();
			card_printer.write(0x00);
			card_printer.byte_add(0x04).write(0x30);
		}
		PRINTSETTING => card_printer.write(0x00),

		_ => eprintln!("card exec: unhandled command {:#0x}", command),
	}
}

pub unsafe fn init() {
	hook::hook_symbol("_ZN13clCardPrinter4execEv", exec as *const ());
}