/// A marker trait for types that can be used with tensr. This is limiting, since
/// the end user must implement this trait for their own types. Unfortunately, it
/// is necessary for the lazy-evaluation system to work due to limitations in
/// Rust's type system.
///
/// As a user of Tensr, feel free to implement this trait for your own types to use
/// them in Tensr arrays.
pub trait TensrType {}

impl TensrType for i16 {}
impl TensrType for i32 {}
impl TensrType for i64 {}
impl TensrType for u16 {}
impl TensrType for u32 {}
impl TensrType for u64 {}

impl TensrType for f32 {}
impl TensrType for f64 {}
