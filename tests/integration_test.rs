use simplex_method_lib::LinearProgram;


#[test]
fn test_linear_pg_1(){
    let constraints = vec![12.0, 16.0];
    let coeffs = vec![vec![1.0, 1.0], vec![2.0, 1.0]];
    let obj_weights = vec![40.0, 30.0];
    let mut my_lin_prog = LinearProgram::new(2, constraints, coeffs, obj_weights);
    assert_eq!(false, my_lin_prog.is_feasible());
    my_lin_prog.run();
    assert_eq!(true, my_lin_prog.is_feasible());
    // my_lin_prog.get_obj_val();
    let var_vals = my_lin_prog.get_var_vals();
    assert_eq!(var_vals[0], 4.0);
    assert_eq!(var_vals[1], 8.0);
}