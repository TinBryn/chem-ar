pub mod vector;
pub use vector::*;

pub mod matrix;
pub use matrix::*;

mod ops_impl {
    #[macro_export]
    macro_rules! impl_vec_op {
        ($trait:tt for $vec:tt of $size:tt with $op:tt) => {
            impl $trait for $vec{
                
            }
        };
    }
}
