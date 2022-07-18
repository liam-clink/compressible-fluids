fn main()
{
    let test = vec![1.342, 12342.2, 324522.1231, 543.12343];
    let _write_success = write_to_file(&test);
}

type UpdateFunction = fn(&Vec<f64>, &mut Vec<f64>, fn(f64) -> f64, f64);

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

fn lf_update(u: &Vec<f64>, u_new: &mut Vec<f64>, f: fn(f64) -> f64, l: f64)
{
    let n = u.len();
    // Periodic boundary
    u_new[0] = 0.5 * (u[1] - u[n - 1]) - l * 0.5 * (f(u[1]) - f(u[n - 1]));
    u_new[n - 1] = 0.5 * (u[0] - u[n - 2]) - l * 0.5 * (f(u[0]) - f(u[n - 2]));

    for i in 1..n - 1
    {
        u_new[i] = 0.5 * (u[i + 1] - u[i - 1]) - l * 0.5 * (f(u[i + 1]) - f(u[i - 1]));
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
    let _remove_success = std::fs::remove_file("test_data/data.tsv");
    
    let functions_to_test = [ftcs_update, lf_update, lw_update];

    for func in functions_to_test
    {
        _run_cases(func);
    }
}


fn _run_cases(update_func: UpdateFunction)
{
    // Case 1
    let lambda = 0.8;
    let grid_size: usize = 40;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let tmax = 30.;
    let times: Vec<f64> =
        linspace::<f64>(0., tmax, (x[x.len() - 1] - x[0] / tmax * lambda) as usize).collect();
    let mut u_initial = vec![0.; grid_size];
    // Initialize u to a sine wave initial condition
    u_initial.iter_mut().zip(&x).for_each(|(u_ele, x_ele)| *u_ele = (PI * x_ele).sin());
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 2
    let tmax = 4.;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> =
        linspace::<f64>(0., tmax, (x[x.len() - 1] - x[0] / tmax * lambda) as usize).collect();
    let mut u_initial = vec![0.; grid_size];
    // Should define a macro for this, or see if one exists
    // The python equivalent is u[np.abs(x)<1/3] = 1.
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 3
    // plot for t=4 and 40
    let tmax = 40.;
    let grid_size: usize = 600;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> =
        linspace::<f64>(0., tmax, (x[x.len() - 1] - x[0] / tmax * lambda) as usize).collect();
    let mut u_initial = vec![0.; grid_size];
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 4
    let tmax = 0.6;
    let grid_size: usize = 40;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> =
        linspace::<f64>(0., tmax, (x[x.len() - 1] - x[0] / tmax * lambda) as usize).collect();
    let mut u_initial = vec![0.; grid_size];
    u_initial
        .iter_mut()
        .zip(&x)
        .filter(|(_u_ele, x_ele)| x_ele.abs() < 1. / 3.)
        .for_each(|(u_ele, _x_ele)| *u_ele = 1.);
    _test_case(lambda, times, x, u_initial, update_func);

    // Case 5
    let grid_size: usize = 40;
    let x: Vec<f64> = linspace::<f64>(-1., 1., grid_size).collect();
    let times: Vec<f64> =
        linspace::<f64>(0., tmax, (x[x.len() - 1] - x[0] / tmax * lambda) as usize).collect();
    let mut u_initial = vec![-1.; grid_size];
    u_initial
    .iter_mut()
    .filter(|x: &&mut f64| x.abs() < 1. / 3.)
    .for_each(|x: &mut f64| *x = 1.);
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
    /*
    for _t in times
    {
        update_func(&u_old, &mut u_new, |x| x, lambda);
        std::mem::swap(&mut u_old, &mut u_new);
    }
    */
    update_func(&u_old, &mut u_new, |x| x, lambda);

    // Write data and then call basic_plot.py using Command::new()
    let _write_success = write_to_file(&u_old);
}

use std::error::Error;

// Write to a file
fn write_to_file(data_line: &Vec<f64>) -> Result<(), Box<dyn Error>>
{
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("test_data/data.tsv")
        .unwrap();

    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .quote_style(csv::QuoteStyle::NonNumeric)
        .from_writer(file);

    let mut data_line_str: Vec<String> = Vec::with_capacity(data_line.len());

    data_line
        .iter()
        .for_each(|x| data_line_str.push(x.to_string()));

    wtr.write_record(&data_line_str)?;

    wtr.flush()?;
    Ok(())
}

// User assertions to check for problems
#[test]
fn test_write() -> Result<(), Box<dyn Error>>
{
    let test: Vec<f64> = vec![1.423, 0.61324, 123.865];
    write_to_file(&test)?;
    // Could use assert_eq! and open file and check matching
    Ok(())
}

// Read from a file
use std::io::Read;
use std::vec;
fn read_from_file()
{
    let mut file = std::fs::File::open("data.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);
}
