#![no_main]
use bytes::{Bytes, BytesMut};
use libfuzzer_sys::fuzz_target;
use oxidized_mc_types::{
    ChatVisibility, Difficulty, Direction, EntitySpawnReason, EquipmentSlot, GameType, HumanoidArm,
    InteractionHand, MobCategory, ParticleStatus, Pose, SoundSource,
};

fuzz_target!(|data: &[u8]| {
    let mut buf = Bytes::copy_from_slice(data);

    // Each wire enum read must not panic regardless of input.
    let _ = ChatVisibility::read(&mut buf.clone());
    let _ = Difficulty::read(&mut buf.clone());
    let _ = Direction::read(&mut buf.clone());
    let _ = EntitySpawnReason::read(&mut buf.clone());
    let _ = EquipmentSlot::read(&mut buf.clone());
    let _ = GameType::read(&mut buf.clone());
    let _ = HumanoidArm::read(&mut buf.clone());
    let _ = InteractionHand::read(&mut buf.clone());
    let _ = MobCategory::read(&mut buf.clone());
    let _ = ParticleStatus::read(&mut buf.clone());
    let _ = Pose::read(&mut buf.clone());
    let _ = SoundSource::read(&mut buf);
});
