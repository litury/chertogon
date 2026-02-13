/// Pseudo-random [0.0, 1.0) — xorshift64, seed из адреса стека (WASM-safe)
pub fn rand_01() -> f32 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static RNG_STATE: AtomicU64 = AtomicU64::new(0);

    let mut state = RNG_STATE.load(Ordering::Relaxed);
    if state == 0 {
        let stack_var: u64 = 0;
        let addr = &stack_var as *const u64 as u64;
        state = addr.wrapping_mul(0x517cc1b727220a95).wrapping_add(0xDEAD_BEEF_CAFE_BABE);
        if state == 0 { state = 1; }
    }
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    RNG_STATE.store(state, Ordering::Relaxed);
    ((state % 10000) as f32) / 10000.0
}
