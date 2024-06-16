#[cfg(feature = "small_matrix_dimensions")]
pub type AxisType = u16;

#[cfg(not(feature = "small_matrix_dimensions"))]
pub type AxisType = u64;

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

#[derive(Debug)]
pub struct Shape {
    pub dims: u8,
    pub len: usize,
    pub axes: [AxisType; MAX_SHAPE_DIMS],
}

impl Shape {
    pub fn new(dimensions: &[AxisType]) -> Self {
        let dims = u8::try_from(dimensions.len())
            .map_err(|_| "Exceeded maximum number of array dimensions")
            .unwrap();
        let mut len = 1;
        let mut axes = [0; MAX_SHAPE_DIMS];

        for i in 0..dims {
            let i = i as usize;
            len *= dimensions[i];
            axes[i] = dimensions[i];
        }

        let len = len as usize;
        Self { dims, len, axes }
    }
}

#[macro_export]
macro_rules! shape {
    ($($dim:expr),*) => {{
        let mut dims = 0;
        let mut len = 1;
        let mut axes = [0; ::tensr::dimension::shape::MAX_SHAPE_DIMS];
        $(
            axes[dims] = $dim;
            len *= $dim;
            dims += 1;
            if dims > ::tensr::dimension::shape::MAX_SHAPE_DIMS {
                panic!("Exceeded maximum number of array dimensions");
            }
        )*

        let dims = u8::try_from(dims).map_err(|_| "Exceeded maximum number of array dimensions").unwrap();

        ::tensr::dimension::shape::Shape {
            dims,
            len,
            axes
        }
    }};
}
