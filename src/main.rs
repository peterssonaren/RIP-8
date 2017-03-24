mod cpu;
mod debugger;
mod opcodes;
mod display;

use cpu::CPU;
use debugger::Debugger;

pub const DEBUG: bool = true;


fn main() {
	let connect4 = "/Users/patallen/Code/Rust/chip8/src/roms/15PUZZLE.ch8";
	let mut cpu = CPU::new();
	cpu.load_rom(connect4);
	// let mut bugger = Debugger::new(connect4);
	// bugger.run();
	cpu.run();
}