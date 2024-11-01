use vectoria::matrix::Mat4;
#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_identity()
    {
        let identity = Mat4::new_identity();
        assert_eq!(identity.data, [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    #[test]
    fn test_new_zero()
    {
        let zero = Mat4::new_zero();
        assert_eq!(zero.data, [[0.0; 4]; 4]);
    }

    #[test]
    fn test_add()
    {
        let mat1 = Mat4::new_identity();
        let mat2 = Mat4::new_identity();
        let result = mat1.add(&mat2);
        assert_eq!(result.data, [
            [2.0, 0.0, 0.0, 0.0],
            [0.0, 2.0, 0.0, 0.0],
            [0.0, 0.0, 2.0, 0.0],
            [0.0, 0.0, 0.0, 2.0],
        ]);
    }

    #[test]
    fn test_mul()
    {
        let mat1 = Mat4::new_identity();
        let mat2 = Mat4::new_identity();
        let result = mat1.mul(&mat2);
        assert_eq!(result.data, [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    #[test]
    fn test_determinant()
    {
        let mat = Mat4::new_identity();
        let det = mat.determinant();
        assert_eq!(det, 1.0);
    }

    #[test]
    fn test_invert()
    {
        let mat = Mat4::new_identity();
        let inv = mat.invert().unwrap();
        assert_eq!(inv.data, mat.data);
    }
}