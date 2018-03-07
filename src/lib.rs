extern crate rand;

#[macro_use]
pub mod cpu;
pub mod ram;
pub mod instruction;

#[cfg(test)]
mod cpu_test;
mod ram_test;
mod instruction_test;