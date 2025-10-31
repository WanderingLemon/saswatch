use std::ops::Range;

#[derive(Default)]
pub struct SliderState {
    position: usize,
    range: Range<usize>,
}

impl SliderState {
    pub fn new(position: usize, range: Range<usize>) -> Self {
        Self { position, range }
    }
}
