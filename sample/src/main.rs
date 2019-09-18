use array2d::Array2D;

fn main() {
    let rows: Vec<Vec<u8>> = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
    //let array = Array2D::from_rows(&rows).as_columns();
    
    let trns: Vec<Vec<u8>> = array
            .into_iter()
            .rev()
            .collect();
    println!("{:?}", trns);
    let result = Array2D::from_columns(&trns[0..]);
    println!("{:?}", result.as_rows());
}