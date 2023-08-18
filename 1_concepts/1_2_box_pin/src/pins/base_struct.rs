use std::fmt::{Debug};

#[derive(Debug, Default, Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

pub struct MeasurableFuture<Fut> {
    pub(crate) inner_future: Fut,
    pub(crate) started_at: Option<std::time::Instant>,
}