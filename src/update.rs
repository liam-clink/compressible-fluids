type UpdateFunction = fn(&Vec<f64>, &mut Vec<f64>, fn(f64) -> f64, f64);

// Forward Time Central Space is unconditionally unstable for the advection equation
fn ftcs_update(u: &Vec<f64>, u_new: &mut Vec<f64>, f: fn(f64) -> f64, l: f64)
{
    let n = u.len();
    // Periodic boundary
    u_new[0] = u[0] - l * 0.5 * (f(u[1]) - f(u[n - 1]));
    u_new[n - 1] = u[n - 1] - l * 0.5 * (f(u[0]) - f(u[n - 2]));

    for i in 1..n - 1
    {
        u_new[i] = u[i] - l * 0.5 * (f(u[i + 1]) - f(u[i - 1]));
    }
}

// Lax-Friedrichs is a modification of FTCS
fn lf_update(u: &Vec<f64>, u_new: &mut Vec<f64>, f: fn(f64) -> f64, l: f64)
{
    let n = u.len();
    // Periodic boundary
    u_new[0] = 0.5 * (u[1] + u[n - 1]) - l * 0.5 * (f(u[1]) - f(u[n - 1]));
    u_new[n - 1] = 0.5 * (u[0] + u[n - 2]) - l * 0.5 * (f(u[0]) - f(u[n - 2]));

    for i in 1..n - 1
    {
        u_new[i] = 0.5 * (u[i + 1] + u[i - 1]) - l * 0.5 * (f(u[i + 1]) - f(u[i - 1]));
    }
}

fn lw_update(u: &Vec<f64>, u_new: &mut Vec<f64>, f: fn(f64) -> f64, l: f64)
{
    // Periodic boundary
    let n = u.len(); // number of grid points
    u_new[0] = u[0] - l * 0.5 * (f(u[1]) - f(u[n - 1]))
        + 0.5
            * l.powi(2)
            * ((f(u[1]) - f(u[0])).powi(2) / (u[1] - u[0])
                - (f(u[0]) - f(u[n - 1])).powi(2) / (u[0] - u[n - 1]));
    u_new[n - 1] = u[n - 1] - l * 0.5 * (f(u[0]) - f(u[n - 2]))
        + 0.5
            * l.powi(2)
            * ((f(u[0]) - f(u[n - 1])).powi(2) / (u[0] - u[n - 1])
                - (f(u[n - 1]) - f(u[n - 2])).powi(2) / (u[n - 1] - u[n - 2]));

    for i in 1..n - 1
    {
        u_new[i] = u[i] - l * 0.5 * (f(u[i + 1]) - f(u[i - 1]))
            + 0.5
                * l.powi(2)
                * ((f(u[i + 1]) - f(u[i])).powi(2) / (u[i + 1] - u[i])
                    - (f(u[i]) - f(u[i - 1])).powi(2) / (u[i] - u[i - 1]));
    }
}

// Test cases
use itertools_num::linspace;
use std::f64::consts::PI;

#[test]
fn run_tests()
{
    let _folder_success = std::fs::create_dir_all("test_data");
    let _remove_success = std::fs::remove_file("test_data/data.tsv");

    let functions_to_test = [lf_update];

    for func in functions_to_test
    {
        _run_cases(func);
    }
}

use math::round;
fn _run_cases(update_func: UpdateFunction)
{
    // Case 1
    let lambda = 0.8; // dt/dx
    let grid_size: usize = 40;
    let x: Vec<f64> = linspace::<f64>(-1., 1. - 2. / grid_size as f64, grid_size - 1).collect();
    let tmax = 30.;
    let times: Vec<f64> = linspace::<f64>(
        0.,
        tmax,
        round::ceil(tmax / (2. / grid_size as f64 * lambda), 0) as usize,
    )
    .collect();
    println!("{}", times.len());
    let mut u_initial = vec![0.; grid_size - 1];
    // Initialize u to a sine wave initial condition
    u_initial
        .iter_mut()
        .zip(&x)
        .for_each(|(u_ele, x_ele)| *u_ele = -(PI * x_ele).sin());
    let _write_success = write_to_file(&u_initial);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 2
    let tmax = 4.;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> = linspace::<f64>(
        0.,
        tmax,
        (tmax / (x[x.len() - 1] - x[0]) * grid_size as f64 / lambda) as usize,
    )
    .collect();
    let mut u_initial = vec![0.; grid_size];
    // Should define a macro for this, or see if one exists
    // The python equivalent is u[np.abs(x)<1/3] = 1.
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    let _write_success = write_to_file(&u_initial);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 3
    // plot for t=4 and 40
    let tmax = 40.;
    let grid_size: usize = 600;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> = linspace::<f64>(
        0.,
        tmax,
        (tmax / (x[x.len() - 1] - x[0]) * grid_size as f64 / lambda) as usize,
    )
    .collect();
    let mut u_initial = vec![0.; grid_size];
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    let _write_success = write_to_file(&u_initial);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 4
    let tmax = 0.6;
    let grid_size: usize = 40;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> = linspace::<f64>(
        0.,
        tmax,
        (tmax / (x[x.len() - 1] - x[0]) * grid_size as f64 / lambda) as usize,
    )
    .collect();
    let mut u_initial = vec![0.; grid_size];
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    let _write_success = write_to_file(&u_initial);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 5
    let grid_size: usize = 40;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> = linspace::<f64>(
        0.,
        tmax,
        (tmax / (x[x.len() - 1] - x[0]) * grid_size as f64 / lambda) as usize,
    )
    .collect();
    let mut u_initial = vec![-1.; grid_size];
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    let _write_success = write_to_file(&u_initial);
    _test_case(lambda, times, x, u_initial, update_func);
}

fn _test_case(
    lambda: f64,
    times: Vec<f64>,
    x: Vec<f64>,
    mut u_old: Vec<f64>,
    update_func: UpdateFunction,
)
{
    let mut u_new = vec![0.; u_old.len()];

    // Do loop with swapping
    for _t in times
    {
        update_func(&u_old, &mut u_new, |x| x, lambda);
        std::mem::swap(&mut u_old, &mut u_new);
    }

    // Write data and then call basic_plot.py using Command::new()
    let _write_success = write_to_file(&u_old);
}
