pub use bytes_derive_internal::FromBytes;
pub trait DeriveFromBytes {
    fn from_bytes(value: &[u8]) -> Self;
}