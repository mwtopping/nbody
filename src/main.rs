

fn main() {

    const NOBJ: usize = 3;
    const Nsteps: usize = 100;

    let mut positions: [[f64; 2]; NOBJ] = [[-1., 0.], [1., 0.], [0., 1.]];
    let mut velocities: [[f64; 2]; NOBJ] = [[0.2, -0.1], [-0.3, 0.1], [0., 0.]];
    let mut accelerations: [[f64; 2]; NOBJ] = [[0., 0.], [0., 0.], [0., 0.]];

    print_coords(&mut positions, NOBJ);


    for ii in 0..Nsteps {
//        velocities = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
        accelerations = [[0.0, 0.0], [0.0, 0.0], [0.0, 0.0]];
//        println!("--------------");
        
        calc_accel(&mut positions, &mut accelerations, NOBJ);
        update_velocity(&mut velocities, &mut accelerations, NOBJ);
        update_position(&mut positions, &mut velocities, NOBJ);
        print_coords(&mut positions, NOBJ);
    }
}

fn calc_accel(positions: &mut [[f64; 2]], accelerations: &mut [[f64; 2]], NOBJ: usize) {
    let G = 0.05;
    let soft = 0.2;

    for ii in 0..NOBJ {
        for jj in 0..NOBJ {
            if ii == jj {
                continue
            }
            let dist = ((positions[ii][0]-positions[jj][0]).powf(2.0) + (positions[ii][1]-positions[jj][1]).powf(2.0)).sqrt()+soft;
            accelerations[ii][0] += G*(positions[jj][0]-positions[ii][0])/dist.powf(3.0);
            accelerations[ii][1] += G*(positions[jj][1]-positions[ii][1])/dist.powf(3.0);
//            println!("{} {} {} {} {}", ii, jj, dist, xaccel, yaccel);
            //velocities[ii][0] += accelerations[ii][0];
            //velocities[ii][1] += accelerations[ii][1];
        }
    }
}




fn update_velocity(velocities: &mut [[f64; 2]], accelerations: &mut [[f64; 2]], NOBJ: usize) {

    for ii in 0..NOBJ {
        velocities[ii][0] += accelerations[ii][0];
        velocities[ii][1] += accelerations[ii][1];
    }
}


fn update_position(positions: &mut [[f64; 2]], velocities: &mut [[f64; 2]], NOBJ: usize) {

    for ii in 0..NOBJ {
        positions[ii][0] += velocities[ii][0];
        positions[ii][1] += velocities[ii][1];
    }
}



fn print_coords(positions: &mut [[f64; 2]], NOBJ: usize) {
    for ii in 0..NOBJ {
 //       println!("x:{} y:{}", positions[ii][0], positions[ii][1]);
        println!("{} {}", positions[ii][0], positions[ii][1]);
    }   
}

