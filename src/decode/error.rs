#[derive(Debug)]
pub enum DecodeError<E> {
    Io(E),
    Decode(bincode::error::DecodeError),
    ReadZero,
    BufferIsFull,
}

#[cfg(feature = "std")]
const _: () = {
    impl<E: std::fmt::Display> std::fmt::Display for DecodeError<E> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Io(error) => write!(f, "IO error: {}", error),
                Self::Decode(error) => write!(f, "Decode error: {}", error),
                Self::ReadZero => write!(f, "Read zero bytes"),
                Self::BufferIsFull => write!(f, "Buffer is full"),
            }
        }
    }

    impl<E: std::error::Error> std::error::Error for DecodeError<E> {}
};
