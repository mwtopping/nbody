use std::thread;

fn main() {

    const n_obj: usize = 3;
    const Nsteps: usize = 10000;
    const dt: f64 = 0.1;

    let mut positions: [[f64; 2]; n_obj] = [[-1., 0.], [1., 0.], [0., -0.5]];
    let mut velocities: [[f64; 2]; n_obj] = [[0.0, -0.3], [0.0, 0.3], [0.0, 0.0]];
    let mut accelerations: [[f64; 2]; n_obj] = [[0., 0.], [0., 0.], [0., 0.]];


    loop {
        accelerations = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        calc_accel(&mut positions, &mut accelerations, n_obj);
        update_velocity(&mut velocities, &mut accelerations, n_obj, dt);
        update_position(&mut positions, &mut velocities, n_obj, dt);

        print_coords(&mut positions, n_obj);


        print!("\x1B[2J\x1B[1;1H");
        show_grid(&mut positions, n_obj);
        thread::sleep_ms(10);
    }

    let tmp = map(0.5, -1.0, 1.0, -10.0, 10.0);    
    println!("{}", tmp);
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


fn show_grid(positions: &mut [[f64; 2]], n_obj: usize) {
    let size = 45;
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
        if (objind > 0) & (objind < gridsize) & (positions[kk][0] > -2.0)
        {
            grid.replace_range(objind..objind+1, "o");
        }
    }

    println!("{}", grid);
}

