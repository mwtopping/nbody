use std::thread;
use rand_distr::{Normal, Distribution};
use rand::thread_rng;
use terminal_size::{terminal_size, Width, Height};
use std::env;



fn main() {

    let args: Vec<String> = env::args().collect();
    let n_obj: usize = args[1].trim().parse().expect("You need to provide a number");

    const dt: f64 = 0.1;
    let normal = Normal::new(0.0, 0.5).unwrap();
    let vnormal = Normal::new(0.0, 0.1).unwrap();

    // create vectors
    let mut positions = Vec::new();
    let mut velocities = Vec::new();
    let mut accelerations = Vec::new();


    // Loop through all of the objects and set random positions and velocities, 
    // and initialize acceleration array
    for ii in 0..n_obj {
        positions.push([normal.sample(&mut rand::thread_rng()), 
                        normal.sample(&mut rand::thread_rng())]);

        velocities.push([vnormal.sample(&mut rand::thread_rng()), 
                         vnormal.sample(&mut rand::thread_rng())]);

        accelerations.push([0.0, 0.0]);
    }

    // main loop
    loop {

        // find terminal size and change display grid appropriately
        let (Width(w), Height(h)) = terminal_size::terminal_size().unwrap();
        let mut size: i32 = 0;
        if w > 2*h {
            size = (h-1).into();
        } else {
            size = (w/2-1).into();
        }

        // need to first zero out the accelerations
        for ii in 0..n_obj {
            accelerations[ii] = [0.0, 0.0]; 
        }
        
        // do the calculations here
        calc_accel(&mut positions, &mut accelerations, n_obj);
        update_velocity(&mut velocities, &mut accelerations, n_obj, dt);
        update_position(&mut positions, &mut velocities, n_obj, dt);

        // draw the points to the terminal
        print!("\x1B[2J\x1B[1;1H");
        show_grid(&mut positions, n_obj, size);
        thread::sleep_ms(50);
    }
}

fn calc_accel(positions: &mut [[f64; 2]], accelerations: &mut [[f64; 2]], n_obj: usize) {
    let G = 0.5;
    let soft = 0.2;

    for ii in 0..n_obj {
        for jj in 0..n_obj {
            if ii == jj {
                continue
            }
            let dist = ((positions[ii][0]-positions[jj][0]).powf(2.0) + (positions[ii][1]-positions[jj][1]).powf(2.0)).sqrt()+soft;
            accelerations[ii][0] += G*(positions[jj][0]-positions[ii][0])/dist.powf(3.0);
            accelerations[ii][1] += G*(positions[jj][1]-positions[ii][1])/dist.powf(3.0);
        }
    }
}




fn update_velocity(velocities: &mut [[f64; 2]], accelerations: &mut [[f64; 2]], n_obj: usize, dt: f64) {

    for ii in 0..n_obj {
        velocities[ii][0] += accelerations[ii][0] *dt;
        velocities[ii][1] += accelerations[ii][1] *dt;
    }
}


fn update_position(positions: &mut [[f64; 2]], velocities: &mut [[f64; 2]], n_obj: usize, dt: f64) {

    for ii in 0..n_obj {
        positions[ii][0] += velocities[ii][0] * dt;
        positions[ii][1] += velocities[ii][1] * dt;
    }
}



fn print_coords(positions: &mut [[f64; 2]], n_obj: usize) {
    for ii in 0..n_obj {
        println!("{} {}", positions[ii][0], positions[ii][1]);
    }   
}



fn map(x: f64, xmin: f64, xmax: f64, toxmin: f64, toxmax: f64) -> i32 {
    let dx = xmax-xmin;
    let xfrac = (x-xmin)/dx; 

    let todx = toxmax-toxmin;
    (xfrac*todx+toxmin).round() as i32
}


fn show_grid(positions: &mut [[f64; 2]], n_obj: usize, size: i32) {
    let gridsize = ((2*size+1)*size) as usize;
    let mut grid = String::from("");
    for ii in 0..size {
        for jj in 0..2*size {
            grid.push(' '); 
        }
        grid.push('\n');
    }

    for kk in 0..n_obj {
        let xind = map(positions[kk][0], -2.0, 2.0, 0.0,2.0*(size as f64));
        let yind = map(positions[kk][1], -2.0, 2.0, 0.0,1.0*(size as f64));
        let objind = (yind*(2*size+1)+xind) as usize;
        if (objind > 0) & (objind < gridsize) & (positions[kk][0] > -2.0) & (positions[kk][0] < 2.0)
        {
            grid.replace_range(objind..objind+1, ".");
        }
    }

    println!("{}", grid);
}
