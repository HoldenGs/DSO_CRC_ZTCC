// use crate::poly_wrapper::{PolyWrapper, PolyToWrapped, WrappedToPoly};


// use nalgebra::SMatrix;
// use ::polynomen::{Poly, poly};


// #[cfg(test)]
// mod tests {


//     #[test]
//     fn test_wrapped_addition() {
//         let poly_1 = poly!(1, 3, 5).wrap();
//         let poly_2 = poly!(1, 0, 1).wrap();
//         assert_eq!(poly!(1, 3, 5) + poly!(1, 0, 1), poly_1 + poly_2);
//     }
    
//     #[test]
//     fn test_wrapped_multiplication() {
//         let poly_11 = poly!(1, 3, 5).wrap();
//         let poly_12 = poly!(1, 0, 1).wrap();
//         let poly_21 = poly!(2, 1).wrap();
//         let poly_22 = poly!(1).wrap();
//         let mat = SMatrix::<PolyWrapper<i32>, 2, 2>::new(poly_11, poly_12, poly_21, poly_22);
//         let mat_2 = mat.clone();
//         let res = mat.dot(&mat_2);
//         println!("{:?}", res);
//     }
    
//     #[test]
//     fn test_wrapped_matrix_multiplication() {
//         let poly_11 = poly!(1, 3, 5).wrap();
//         let poly_12 = poly!(1, 0, 1).wrap();
//         let poly_21 = poly!(2, 1).wrap();
//         let poly_22 = poly!(1).wrap();
//         let mat = SMatrix::<PolyWrapper<i32>, 2, 2>::new(poly_11, poly_12, poly_21, poly_22);
//         let mat_2 = mat.clone();
//         let res = mat.dot(&mat_2);
//         println!("{:?}", res);
//     }
// }