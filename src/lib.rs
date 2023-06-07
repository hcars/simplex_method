mod linear_algebra;

use crate::linear_algebra::matrix_vec_product;
use crate::linear_algebra::dot_product;
use crate::linear_algebra::min_arg;

use self::linear_algebra::pretty_print;

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

    pub fn get_var_vals(& self) -> Vec::<f64> {
        self.vars.clone()
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
        self.get_basic_soln();

        let mut tableau_mat = Vec::from_iter(self.coeffs.iter().cloned());
        for i in 0..tableau_mat.len() {
            let mut slacks = vec![0.0; self.constraints.len()];
            slacks.append(& mut vec![self.constraints[i]]);
            tableau_mat[i].append(& mut slacks);
            tableau_mat[i][i + self.vars.len()] = 1.0;
        }
        let mut phis: Vec<f64> = self.obj_weights.iter().map(|x| -x).collect();
        phis.append(& mut vec![0.0; self.constraints.len() + 1]);
        tableau_mat.append(& mut vec![phis]);



        let mut basic_vars: Vec<bool> = (0..(self.vars.len() + self.slacks.len())).map(|x| x >= self.vars.len()).collect();
        let mut all_vars = Vec::from_iter(self.vars.iter().cloned());
        all_vars.append(& mut Vec::from_iter(self.slacks.iter().cloned()));
        let last_col = self.vars.len() + self.constraints.len();
        let last_row = tableau_mat.len() - 1;
        let mut pivot_col = min_arg(& tableau_mat[last_row]);
        let mut pivot_row = self.get_pivot_row(pivot_col, & tableau_mat);
        while ! non_negative(& tableau_mat[last_row]) {
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
                }
            }

            pretty_print(& tableau_mat);
            
            basic_vars[pivot_col] = true;
            basic_vars[pivot_row + self.vars.len()] = false;
            // zero-out non basic vars.
            all_vars[pivot_row + self.vars.len()] = 0.0;
            // Find the new value of the basic var.
            let mut val = tableau_mat[pivot_row][last_col];
            println!("val {}", val);
            for i in 0..all_vars.len() {
                if i != pivot_col {
                    val -= all_vars[i] * tableau_mat[pivot_row][i];
                }
            }
            val /= tableau_mat[pivot_row][pivot_col];
            all_vars[pivot_col] = val;
            
    
            

            pivot_col = min_arg(& tableau_mat[tableau_mat.len() - 1]);

            pivot_row = self.get_pivot_row(pivot_col, & tableau_mat);
            

        }

        for i in 0..all_vars.len() {
            if i < self.vars.len() {
                
                for j in 0..self.vars.len() {
                    if tableau_mat[i][j] != 0.0 {
                        self.vars[j] = tableau_mat[i][last_col];
                    }
                }
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
            let min_canditate = curr_mat[i][curr_mat[i].len() - 1]  / curr_mat[i][pivot_col];
            if min_canditate < curr_min {
                curr_min = curr_mat[i][curr_mat[i].len() - 1] ;
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



