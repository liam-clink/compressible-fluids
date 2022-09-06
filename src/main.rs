// Euler equation solving assuming periodic boundary in 1D
mod io;

fn main()
{
    let mut source = ndarray::Array2::<f64>::zeros((100, 100));
    source[(50, 50)] = 1.;
    let source = source;

    let pressure = source.solve_laplace(0.1, 1.0e-6, 10000);
    // let zip_iter = std::iter::zip(0..pressure.len(), &pressure);
    // zip_iter.for_each(|(idx, val)| println!("{}: {:?}", idx, val));
    for row in pressure.axis_iter(ndarray::Axis(0))
    {
        let _result = io::write_to_file(row);
    }
}

pub trait SolveLaplace
{
    // Gives the solution to the laplace equation from the source this is applied to
    fn solve_laplace(&self, dx: f64, tolerance: f64, tries: u64) -> Self;
}

impl SolveLaplace for ndarray::Array1<f64>
{
    fn solve_laplace(&self, dx: f64, tolerance: f64, tries: u64) -> ndarray::Array1<f64>
    {
        let n = self.shape()[0];
        let mut f = ndarray::Array1::<f64>::zeros(n);
        let mut delta: f64;

        // Boundary handling (Dirichlet currently)
        f[0] = 1.;
        f[n - 1] = 0.;

        for _try in 0..tries
        {
            let mut error = 0.;

            // Bulk handling
            for i in 1..n - 1
            {
                delta = 0.5 * (f[i - 1] + f[i + 1] - dx.powi(2) * self[i]) - f[i];
                f[i] += delta;
                error += delta.abs();
            }

            if error / (n as f64) < tolerance
            {
                println!("Tolerance reached\n");
                break;
            }
        }

        return f;
    }
}

impl SolveLaplace for ndarray::Array2<f64>
{
    fn solve_laplace(&self, dx: f64, tolerance: f64, tries: u64) -> Self
    {
        let xn = self.shape()[0];
        let yn = self.shape()[1];
        let mut f = ndarray::Array2::<f64>::zeros((xn, yn));
        let mut delta: f64;

        // Boundary handling (Dirichlet currently)
        for i in 0..xn
        {
            f[(i, 0)] = 1. - ((i as f64) / (xn as f64));
            f[(i, yn - 1)] = 0. + ((i as f64) / (xn as f64));
        }
        for j in 0..yn
        {
            f[(0, j)] = 1. - ((j as f64) / (yn as f64));
            f[(xn - 1, j)] = 0. + ((j as f64) / (yn as f64));
        }

        for _try in 0..tries
        {
            let mut error = 0.;

            // Bulk handling
            for i in 1..xn - 1
            {
                for j in 1..yn - 1
                {
                    delta = 0.25
                        * (f[(i + 1, j)] + f[(i - 1, j)] + f[(i, j + 1)] + f[(i, j - 1)]
                            - dx.powi(2) * self[(i, j)])
                        - f[(i, j)];
                    f[(i, j)] += delta;
                    error += delta.abs();
                }
            }

            if error / ((xn * yn) as f64) < tolerance
            {
                println!("Tolerance reached\n");
                break;
            }
        }

        return f;
    }
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
