mod cpu1;
mod cpu2;

use cpu1::RIA1;
use cpu2::RIA2;

fn main() {
    let mut cpu1 = RIA1 {
        current_operation: 0,
        registers: [0; 2],
    };

    cpu1.current_operation = 0x8014;
    cpu1.registers[0] = 5;
    cpu1.registers[1] = 10;

    cpu1.run();

    println!("{}", cpu1.registers[0]);
}