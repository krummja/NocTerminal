
pub trait Within<T> {
    fn within(self: &Self, other: T) -> bool;
}

pub trait Coordinate<T> {
    type X;
    type Y;

    fn position(&self) -> (T, T);
}

pub trait Dimension<T> {
    type Width;
    type Height;
}
