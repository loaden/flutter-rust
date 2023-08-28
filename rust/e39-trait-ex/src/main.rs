fn main() {
    slider(0..=10, 5, StepMessage::SliderChanged);
    slider(0..=10, 5, |_| StepMessage::SliderChanged(8));
}

#[derive(Clone)]
enum StepMessage {
    SliderChanged(u8),
}

#[allow(unused_variables)]
pub fn slider<'a, T, Message>(
    range: std::ops::RangeInclusive<T>,
    value: T,
    on_change: impl Fn(T) -> Message + 'a,
) where
    T: Copy + From<u8> + std::cmp::PartialOrd,
    Message: Clone,
{
}
