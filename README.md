# Vectoria

Vectoria is a Rust library for working with vectors and matrices as well as basic arithmetic & geometry operations.
It is designed to be easy to use and efficient.

## Features

- Vectors and matrices of 3x3 and 4x4
- Basic arithmetic operations
- Dot product
- Cross product
- Matrix multiplication
- Transpose
- Inverse

You can use the modules in your code:
```rust
use vectoria::{ Vector, Matrix };

fn main() 
{
    let v1 = Vector::new(vec![1.0, 2.0, 3.0]);
    let v2 = Vector::new(vec![4.0, 5.0, 6.0]);
    let dot_product = v1.dot(&v2);
    println!("Dot product: {}", dot_product);

    let m1 = Matrix::new(vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let m2 = Matrix::new(vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    let product = m1.multiply(&m2);
    println!("Matrix product: {:?}", product);
}
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
