#![no_main]
use libfuzzer_sys::fuzz_target;
use oxidized_mc_types::ResourceLocation;

fuzz_target!(|data: &str| {
    // Exercise from_string with arbitrary input — must never panic.
    let _ = ResourceLocation::from_string(data);
});
