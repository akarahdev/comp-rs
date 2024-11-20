use std::sync::atomic::AtomicU64;

static idx: AtomicU64 = AtomicU64::new(0);

pub fn new_id() -> u64 {
    idx.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
}