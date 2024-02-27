

fn main() {

    const NOBJ: usize = 3;
    const Nsteps: usize = 4;

    let mut positions: [[f64; 2]; NOBJ] = [[0., 0.], [0., 0.], [0., 0.]];
    let mut velocities: [[f64; 2]; NOBJ] = [[0., 0.], [0., 0.], [0., 0.]];

    print_coords(&mut positions, NOBJ);

    for ii in 0..Nsteps {
        velocities = [[0.1, 0.1], [0.1, 0.1], [0.1, 0.1]];
        println!("--------------");
        update_position(&mut positions, &mut velocities, NOBJ);
        print_coords(&mut positions, NOBJ);
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
        println!("x:{} y:{}", positions[ii][0], positions[ii][1]);
    }   
}

