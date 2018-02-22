extern crate sensel;

use std::io::stdin;
use std::thread::spawn;
use std::sync::mpsc::channel;

fn main() {
    let list = sensel::device::get_device_list().unwrap();

    let list_slice = list.as_slice();

    if list_slice.len() == 0 {
        println!("No device found");
        println!("Press Enter to exit example");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        return;
    }

    let device = list_slice[0].open().unwrap();

    device.set_frame_content(sensel::frame::Mask::PRESSURE).unwrap();

    device.start_scanning().unwrap();

    println!("Press Enter to exit example");
    let (sender, receiver) = channel();
    spawn(move || {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        sender.send(()).unwrap();
    });

    while receiver.try_recv().is_err() {
        device.read_sensor().unwrap();

        let num_frames = device.get_num_available_frames().unwrap();

        for _ in 0..num_frames {
            let frame = device.get_frame().unwrap();
            let force_array = frame.force_array.unwrap();

            let mut total_force = 0.0;

            for &force in force_array {
                total_force += force;
            }
            println!("Total Force : {}", total_force);
        }
    }
}
