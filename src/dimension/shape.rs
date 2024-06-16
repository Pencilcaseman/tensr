#[cfg(feature = "small_matrix_dimensions")]
pub type DimensionType = i16;

#[cfg(not(feature = "small_matrix_dimensions"))]
pub type DimensionType = i64;

#[cfg(all(feature = "max_array_dim_2", not(feature = "max_array_dim_3")))]
pub const MAX_SHAPE_DIMS: usize = 2;

#[cfg(all(feature = "max_array_dim_3", not(feature = "max_array_dim_4")))]
pub const MAX_SHAPE_DIMS: usize = 3;

#[cfg(all(feature = "max_array_dim_4", not(feature = "max_array_dim_5")))]
pub const MAX_SHAPE_DIMS: usize = 4;

#[cfg(all(feature = "max_array_dim_5", not(feature = "max_array_dim_6")))]
pub const MAX_SHAPE_DIMS: usize = 5;

#[cfg(all(feature = "max_array_dim_6", not(feature = "max_array_dim_7")))]
pub const MAX_SHAPE_DIMS: usize = 6;

#[cfg(all(feature = "max_array_dim_7", not(feature = "max_array_dim_8")))]
pub const MAX_SHAPE_DIMS: usize = 7;

#[cfg(feature = "max_array_dim_8")]
pub const MAX_SHAPE_DIMS: usize = 8;

pub struct Shape {
    len: usize,
    dims: [DimensionType; MAX_SHAPE_DIMS],
}
