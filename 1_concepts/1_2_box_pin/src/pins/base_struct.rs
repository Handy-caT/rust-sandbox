use std::fmt::{Debug};
use pin_project::pin_project;

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

#[pin_project]
pub struct MeasurableFuture<Fut> {
    #[pin]
    pub(crate) inner_future: Fut,
    pub(crate) started_at: Option<std::time::Instant>,
}