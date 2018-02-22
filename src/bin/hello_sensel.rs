extern crate sensel;

use std::io::stdin;

fn main() {
    let mut input = String::new();

    let list = sensel::device::get_device_list().unwrap();

    let list_slice = list.as_slice();

    if list_slice.len() == 0 {
        println!("No device found");
        println!("Press Enter to exit example");
        stdin().read_line(&mut input).unwrap();
        return;
    }

    let device_id = list_slice[0];
    let device = device_id.open().unwrap();

    println!("Sensel Device: {}" , device_id.get_serial_num() );
    println!("COM port: {}" , device_id.get_com_port() );
    println!("Firmware Version: {}.{}.{}", device.fw_info.fw_version_major, device.fw_info.fw_version_minor, device.fw_info.fw_version_build);
    println!("Width: {}mm", device.sensor_info.width);
    println!("Height: {}mm", device.sensor_info.height);
    println!("Cols: {}", device.sensor_info.num_cols);
    println!("Rows: {}", device.sensor_info.num_rows);

    println!("Press Enter to exit example");
    stdin().read_line(&mut input).unwrap();
}
