
pub struct LinearProgram {
    constraints: Vec<f64>, // m x 1
    slacks:  Vec<f64>, // m x 1 
    coeffs: Vec<Vec<f64>>, // m x n 
    obj_weights: Vec<f64>, // n x 1
    vars: Vec<f64>, // n x 1
}

impl LinearProgram {
    pub fn new(num_vars: usize, constraints: Vec::<f64>, coeffs: Vec::<Vec<f64>>, obj_weights: Vec::<f64>) -> Self{
        Self {
            slacks: vec![0.0; constraints.len()],
            constraints,
            coeffs,
            obj_weights,
            vars: vec![0.0; num_vars],
        }
    }
    pub fn get_obj_val(& self) {
        let obj_val = dot_product(&self.obj_weights, &self.vars);
        println!("The value of the objective funciton is {}", obj_val);
    }

    pub fn get_var_vals(& self) {
        for i in 0..self.vars.len() {
            println!("Variable {}, Value {}", i, self.vars[i]);
        } 
    }

    pub fn is_feasible(& self) -> bool {
        let current_vals = matrix_vec_product(&self.coeffs, &self.vars);

        for i in 0..current_vals.len() {
            if current_vals[i] + self.slacks[i] != self.constraints[i]  {
                return false;
            }
        }
        return true;
    }

    fn get_basic_soln(& mut self) {
        let current_vals = matrix_vec_product(&self.coeffs, &self.vars);
        let  slacks =  &mut self.slacks;
        for i in 0..current_vals.len() {
            if current_vals[i] + slacks[i] != self.constraints[i]  {
                slacks[i] = self.constraints[i] - current_vals[i];
            }
        }
    }

    pub fn run(& mut self){
        let mut phis: Vec<f64> = self.obj_weights.iter().map(|x| -x).collect();
        self.get_basic_soln();

        let mut tableau_mat = Vec::from_iter(self.coeffs.iter().cloned());
        for i in 0..tableau_mat.len() {
            tableau_mat[i].append(& mut vec![0.0; self.constraints.len()]);
            tableau_mat[i][i + self.vars.len()] = 1.0;
        }

        let mut basic_vars: Vec<bool> = (0..(self.vars.len() + self.slacks.len())).map(|x| x >= self.vars.len()).collect();
        let mut all_vars = Vec::from_iter(self.vars.iter().cloned());
        all_vars.append(& mut Vec::from_iter(self.slacks.iter().cloned()));
        let mut pivot_col = min(& phis);
        let mut pivot_row = self.get_pivot_row(pivot_col, & tableau_mat);
        while ! non_negative(& phis) {
            // set pivot to 1
            let pivot_val = tableau_mat[pivot_row][pivot_col];
            tableau_mat[pivot_row] = tableau_mat[pivot_row].iter().map(|x| x / pivot_val).collect();

            let pivot_val = tableau_mat[pivot_row][pivot_col];
            for row in 0..tableau_mat.len() {
                if row != pivot_row {
                    let multiplier = tableau_mat[row][pivot_col] / pivot_val;
                    for col in 0..tableau_mat[row].len() {
                        tableau_mat[row][col] -= tableau_mat[pivot_row][col] * multiplier;
                    }
                    self.slacks[row] -= self.slacks[pivot_row] * multiplier;
                }
            }
            let mut multiplier = phis[pivot_col] / pivot_val;
            if phis[pivot_col] < 0.0 && tableau_mat[pivot_row][pivot_col] < 0.0 {
                multiplier *= -1.0;
            }
            for col in 0..phis.len() {
                phis[col] -= tableau_mat[pivot_row][col] * multiplier;
            }
            pretty_print(& tableau_mat);
            
            basic_vars[pivot_col] = true;
            basic_vars[pivot_row + self.vars.len()] = false;
            for i in 0..tableau_mat.len() {
                for j in 0..all_vars.len() {
                    if basic_vars[j] && tableau_mat[i][j] != 0.0 {
                        all_vars[j] = self.slacks[i] / tableau_mat[i][j];
                    }
                    if ! basic_vars[j]{
                        all_vars[j] = 0.0;
                    }
                }
            }
    
            

            pivot_col = min(& phis);

            pivot_row = self.get_pivot_row(pivot_col, & tableau_mat);
            

        }

        for i in 0..all_vars.len() {
            if i < self.vars.len() {
                self.vars[i] = all_vars[i];
            }
            else {
                self.slacks[i - self.vars.len()] = all_vars[i];
            }
        }
    }

    fn get_pivot_row(& mut self, pivot_col: usize, curr_mat: & Vec<Vec<f64>>) -> usize {
        let mut curr_min = std::f64::MAX;
        let mut curr_min_ind = 0;

        for i in 0..self.constraints.len() {
            let min_canditate = self.slacks[i]  / curr_mat[i][pivot_col];
            if min_canditate < curr_min {
                self.slacks[i] = min_canditate;
                curr_min = self.slacks[i];
                curr_min_ind = i;
            }
        }

        curr_min_ind
    
    
    }

}


fn non_negative(vec: &Vec<f64>) -> bool {
    let non_negative = true;
    for val in vec {

        if val < &0.0 {
            return false;
        }
    }
    non_negative
}

fn pretty_print(mat: & Vec<Vec<f64>>) {
    for curr_vec in mat {
        let mut curr_row = String::new();
        for curr_val in curr_vec {
            curr_row.push_str(& curr_val.to_string());
            curr_row.push_str(" | ");
        }
        curr_row.push_str("\n------");

        println!("{}", curr_row);
    }
}

fn id_mat(size: usize) -> Vec<Vec<f64>>{
    let mut id_mat = vec![vec![0.0; size]; size];
    for i in 0..size {
        id_mat[i][i] = 1.0;
    }
    id_mat
}

fn min(vec: & Vec<f64>) -> usize {
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

fn matrix_vec_product(mat: &Vec<Vec<f64>>, vec: &Vec<f64>) -> Vec<f64> {
    let mut product = vec![0.0; mat.len()];
    for i in 0..mat.len() {
        product[i] = dot_product(& mat[i], vec);
    }
    product
}

fn dot_product(vec1: &Vec<f64>, vec2: &Vec<f64>) -> f64 {
    let mut acc: f64 = 0.0;
    for i in 0..vec1.len() {
        acc += vec1[i] * vec2[i];
    }
    acc
}



#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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