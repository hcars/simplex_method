pub fn matrix_vec_product(mat: &Vec<Vec<f64>>, vec: &Vec<f64>) -> Vec<f64> {
    let mut product = Vec::<f64>::with_capacity(mat.len());
    for row in mat.iter() {
        product.push(dot_product(& row, vec));
    }
    product
}

pub fn dot_product(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let mut acc: f64 = 0.0;
    for i in 0..vec1.len() {
        acc += vec1[i] * vec2[i];
    }
    acc
}

pub fn pretty_print(mat: & Vec<Vec<f64>>) {
    println!("-----------------");
    for curr_vec in mat {
        let mut curr_row = String::new();
        for curr_val in curr_vec {
            curr_row.push_str(& curr_val.to_string());
            curr_row.push_str(" | ");
        }
       

        println!("{}", curr_row);
        println!("-----------------");
    }
    println!("");
}

fn id_mat(size: usize) -> Vec<Vec<f64>>{
    let mut id_mat = vec![vec![0.0; size]; size];
    for i in 0..size {
        id_mat[i][i] = 1.0;
    }
    id_mat
}

pub fn min_arg(vec: & Vec<f64>) -> usize {
    let mut min = std::f64::MAX;
    let mut pivot_col = 0;
    for i in 0..vec.len() {
        if vec[i] < min {
            min = vec[i];
            pivot_col = i;
        }
    }
    return pivot_col;
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::dot_product;
    use super::matrix_vec_product;

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![2.0, 3.0, 4.0];
        assert_eq!(dot_product(&a, &b), 20.0);
    }

    #[test]
    fn test_mat_vec_product() {
        let A = vec![vec![1.0, 2.0, 3.0], vec![2.0, 3.0, 4.0]];
        let b = vec![1.0, 0.0, 0.0];
        assert_eq!(matrix_vec_product(&A, &b), vec![1.0, 2.0]);
    }

    
}