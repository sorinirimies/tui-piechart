#![no_main]
mod common;
use common::*;
use libfuzzer_sys::fuzz_target;

fuzz_target!(|input: ChartIn| run_chart(&input));
