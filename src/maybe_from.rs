pub trait MaybeFrom<T>: Sized {
    fn maybe_from(from: T) -> Option<Self>;
}
