use std::ops::Range;

pub struct SpectreError {
    pub msg: String,
    pub reason: String,
    pub range: Range<usize>,
}
