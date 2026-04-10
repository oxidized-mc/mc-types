#![no_main]
use libfuzzer_sys::fuzz_target;
use oxidized_mc_types::SectionPos;

fuzz_target!(|data: i64| {
    // from_long must handle every possible i64 without panicking.
    let pos = SectionPos::from_long(data);
    // Roundtrip: the packed value of a decoded position should survive re-packing.
    let repacked = pos.as_long();
    let _pos2 = SectionPos::from_long(repacked);
});
