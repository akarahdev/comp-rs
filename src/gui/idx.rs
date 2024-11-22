use std::sync::atomic::AtomicU64;

static ID_INDEX: AtomicU64 = AtomicU64::new(0);

pub fn new_id() -> u64 {
    ID_INDEX.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}
