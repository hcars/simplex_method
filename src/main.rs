use crate::linear_program::LinearProgram;
pub mod linear_program;

fn main() {
    // let constraints = vec![12.0, 16.0];
    // let coeffs = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
    // let obj_weights = vec![40.0, 30.0];
    // let mut my_lin_prog = LinearProgram::new(2, constraints, coeffs, obj_weights);
    // println!("Is this a feasible solution? {}", my_lin_prog.is_feasible());
    // my_lin_prog.run();
    // println!("Is this a feasible solution? {}", my_lin_prog.is_feasible());
    // my_lin_prog.get_obj_val();
    // my_lin_prog.get_var_vals();

    let constraints = vec![5.0, -4.0, -2.0];
    let coeffs = vec![vec![1.0, 0.0], vec![-1.0, -1.0], vec![-1.0, 1.0]];
    let obj_weights = vec![3.0, 4.0];
    let mut my_lin_prog = LinearProgram::new(2, constraints, coeffs, obj_weights);
    my_lin_prog.get_obj_val();
    println!("Is this a feasible solution? {}", my_lin_prog.is_feasible());
    my_lin_prog.run();
    println!("Is this a feasible solution? {}", my_lin_prog.is_feasible());
    my_lin_prog.get_obj_val();
    my_lin_prog.get_var_vals();
    println!("Is this a feasible solution? {}", my_lin_prog.is_feasible());
}
