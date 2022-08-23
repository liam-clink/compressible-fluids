// Euler equation solving assuming periodic boundary in 1D
mod io;

fn main()
{
    let mut source = ndarray::Array1::<f64>::zeros(100);
    source[50] = 1.;
    let source = source;
    let mut pressure = solve_laplace(source, 0.1, 0.1, 1000);

    pressure.iter();
    io::write_to_file(&pressure);
}

// Should use traits for dimensionality right?
// Like define an extension to arrays that has a Solve_Laplace trait?
fn solve_laplace(
    laplacian: ndarray::Array1<f64>,
    dx: f64,
    tolerance: f64,
    tries: u64,
) -> ndarray::Array1<f64>
{
    let n = laplacian.len();
    let mut f = ndarray::Array1::<f64>::zeros(n);
    let mut delta: f64;

    for _j in 0..tries
    {
        let mut error = 0.;

        // Boundary handling (periodic boundary for now)
        f[0] = 0.5 * (f[n - 1] + f[1]) - dx.powi(2) * laplacian[0];
        f[n - 1] = 0.5 * (f[n - 2] + f[0]) - dx.powi(2) * laplacian[n - 1];
        // Bulk handling
        for i in 1..n - 1
        {
            delta = 0.5 * (f[i - 1] + f[i + 1]) - dx.powi(2) * laplacian[i] - f[i];
            f[i] += delta;
            error += delta;
        }

        if error / (n as f64) < tolerance
        {
            break;
        }
    }

    return f;
}

/*
let grid_size = 100;
let dx: f64 = 0.1;
let density = 1.0;
let mut pressure = ndarray::Array1::<f64>::zeros(grid_size);
let mut pressure_laplacian = ndarray::Array1::<f64>::zeros(grid_size);
let mut velocity = ndarray::Array1::<f64>::zeros(grid_size);

velocity.iter_mut().for_each(|ele| {
    *ele = 1.0;
});

// Boundary handling
pressure_laplacian[0] = (velocity[1] - velocity[grid_size - 1]).powi(2)
    + 4. * velocity[0] * (velocity[1] - 2. * velocity[0] + velocity[grid_size - 1]);
pressure_laplacian[grid_size] = (velocity[0] - velocity[grid_size - 1]).powi(2)
    + 4. * velocity[grid_size]
        * (velocity[0] - 2. * velocity[grid_size] + velocity[grid_size - 1]);
// Bulk handling
for i in 1..grid_size - 1
{
    pressure_laplacian[i] = (velocity[i + 1] - velocity[i - 1]).powi(2)
        + 4. * velocity[i] * (velocity[i + 1] - 2. * velocity[i] + velocity[i - 1]);
}
pressure_laplacian *= -density * (2. * dx).powi(-2);

// Boundary handling
pressure[0] =
    0.5 * (pressure[grid_size - 1] + pressure[1]) - dx.powi(2) * pressure_laplacian[0];
pressure[grid_size - 1] = 0.5 * (pressure[grid_size - 2] + pressure[0])
    - dx.powi(2) * pressure_laplacian[grid_size - 1];
// Bulk handling
for i in 1..pressure.len() - 1
{
    pressure[i] =
        0.5 * (pressure[i - 1] + pressure[i + 1]) - dx.powi(2) * pressure_laplacian[i];
}
*/
