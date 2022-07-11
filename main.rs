fn main()
{
    let test_array = [1.; 10];

    ftcs_update(&test_array, square, 0.1);
}

fn square(x: f64) -> f64
{
    return x*x;
}

fn ftcs_update<const C: usize>(u: &[f64;C], f: fn(f64)->f64, l: f64)
{
    const grid_size: usize = &u.len();
    let u_new = [0.; grid_size];
/*
    // Periodic boundary
    u_new[0] = u[0] - l*0.5*(f(u[1]) - f(u[u.len()-1]));
    u_new[u.len()-1] = u[u.len()-1] - l*0.5*(f(u[0]) - f(u[u.len()-2]));

    for i in 1 .. grid_size-1
    {
        u_new[i] = u[i] - l*0.5*(f(u[i+1]) - f(u[i-1]));
    }

    return u_new;
    */
}

/*
def LF_update(u, f, l):
    grid_size = len(u)
    u_new = np.zeros_like(u)

    # Periodic boundary
    u_new[0] = .5*(u[1]-u[-1]) - l*.5*(f(u[1]) - f(u[-1]))
    u_new[-1] = .5*(u[0]-u[-2]) - l*.5*(f(u[0]) - f(u[-2]))

    for i in range(1,grid_size-1):
        u_new[i] = .5*(u[i+1]-u[i-1]) - l*.5*(f(u[i+1]) - f(u[i-1]))
    
    return u_new
    
def LW_update(u, f, l):
    grid_size = len(u)
    u_new = np.zeros_like(u)

    # Periodic boundary
    u_new[0] = u[0] - l*.5*(f(u[1]) - f(u[-1])) \
            + .5*l**2*((f(u[1])-f(u[0]))**2/(u[1]-u[0]) - (f(u[0])-f(u[-1]))**2/(u[0]-u[-1]))
    u_new[-1] = u[-1] - l*.5*(f(u[0]) - f(u[-2])) \
            + .5*l**2*((f(u[0])-f(u[-1]))**2/(u[0]-u[-1]) - (f(u[-1])-f(u[-2]))**2/(u[-1]-u[-2]))

    for i in range(1,grid_size-1):
        u_new[i] = u[i] - l*.5*(f(u[i+1]) - f(u[i-1])) \
            + .5*l**2*((f(u[i+1])-f(u[i]))**2/(u[i+1]-u[i]) - (f(u[i])-f(u[i-1]))**2/(u[i]-u[i-1]))


## Test cases

# Case 1
l = 0.8
x = np.linspace(-1,1,40)
t = np.linspace(0,30,int((x[-1]-x[0])/30*l))
u = -np.sin(np.pi*x)
# lambda f(u)=u

# Case 2
tmax = 4.
t = np.linspace(0,tmax,int((x[-1]-x[0])/tmax*l))
u = np.zeros_like(x)
u[np.abs(x)<1/3] = 1.

# Case 3
# plot for t=4 and 40
tmax = 40.
x = np.linspace(-1,1,600)
t = np.linspace(0,tmax,int((x[-1]-x[0])/tmax*l))
u = np.zeros_like(x)
u[np.abs(x)<1/3] = 1.

# Case 4
tmax = .6
x = np.linspace(-1,1,40)
t = np.linspace(0,tmax,int((x[-1]-x[0])/tmax*l))
u = np.zeros_like(x)
u[np.abs(x)<1/3] = 1.

# Case 5
tmax = .3
x = np.linspace(-1,1,40)
t = np.linspace(0,tmax,int((x[-1]-x[0])/tmax*l))
u = -np.ones_like(x)
u[np.abs(x)<1/3] = 1.
*/