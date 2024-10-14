use std::sync::atomic::{AtomicI32, AtomicI64, Ordering};


static DEFAULT_ORDER: Ordering = Ordering::Relaxed;

pub trait Identity {
    type Id;

    fn seed(seed: Self::Id) -> Self;

    fn next(&self) -> Self::Id;

    fn current(&self) -> Self::Id {
        self.next()
    }
}

pub struct Int {
    counter: AtomicI32,
}

impl Int {
    pub fn new() -> Self {
        Int {
            counter: AtomicI32::new(1),
        }
    }
}

impl Identity for Int {
    type Id = i32;

    fn seed(seed: Self::Id) -> Self {
        Int {
            counter: AtomicI32::new(seed),
        }
    }

    fn next(&self) -> Self::Id {
        self.counter.fetch_add(1, DEFAULT_ORDER)
    }

    fn current(&self) -> Self::Id {
        self.counter.load(DEFAULT_ORDER)
    }
}


pub struct BigInt {
    counter: AtomicI64,
}

impl BigInt {
    pub fn new() -> Self {
        BigInt {
            counter: AtomicI64::new(1),
        }
    }
}

impl Identity for BigInt {
    type Id = i64;

    fn seed(seed: Self::Id) -> Self {
        BigInt {
            counter: AtomicI64::new(seed),
        }
    }

    fn next(&self) -> Self::Id {
        self.counter.fetch_add(1, DEFAULT_ORDER)
    }

    fn current(&self) -> Self::Id {
        self.counter.load(DEFAULT_ORDER)
    }
}