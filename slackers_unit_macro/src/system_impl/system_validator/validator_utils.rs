#[allow(unused)]
pub(crate) enum AcceptOnlyOne<T> {
    None,
    One,
    Two(T),
}
#[allow(unused)]
impl<T> AcceptOnlyOne<T> {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_one(&self) -> bool {
        matches!(self, Self::One)
    }

    pub fn is_two(&self) -> bool {
        matches!(self, Self::Two(_))
    }
}
