#[derive(Copy, Clone, Debug)]
struct CubeSat {
    id: u64,
}

#[derive(Copy, Clone, Debug)]
enum StatusMessage {
    Ok,
}

// impl Copy for CubeSat {}
// impl Copy for StatusMessage {}

// impl Clone for CubeSat {
//     fn clone(&self) -> Self {
//         CubeSat { id: self.id }
//     }
// }

// impl Clone for StatusMessage {
//     fn clone(&self) -> Self {
//         *self
//     }
// }

fn check_status(sat: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

pub fn run() {
    let sat_a = CubeSat { id: 1 };
    let sat_b = CubeSat { id: 2 };
    let sat_c = sat_a;

    println!("sat_a: {:?}", sat_a);
    println!("sat_b: {:?}", sat_b);
    println!("sat_c: {:?}", sat_c);

    let status_a = check_status(sat_a);
    let status_b = check_status(sat_b.clone());
    let status_c = check_status(sat_c);

    println!("status_a: {:?}", status_a);
    println!("status_b: {:?}", status_b);
    println!("status_c: {:?}", status_c);
}
