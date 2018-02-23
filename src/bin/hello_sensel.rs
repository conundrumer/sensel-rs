extern crate sensel;

use std::io::stdin;

use sensel::device::Device;

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
    let info = device.get_info();

    println!("Sensel Device: {}" , device_id.get_serial_num() );
    println!("COM port: {}" , device_id.get_com_port() );
    println!("Firmware Version: {}.{}.{}", info.fw_info.fw_version_major, info.fw_info.fw_version_minor, info.fw_info.fw_version_build);
    println!("Width: {}mm", info.sensor_info.width);
    println!("Height: {}mm", info.sensor_info.height);
    println!("Cols: {}", info.sensor_info.num_cols);
    println!("Rows: {}", info.sensor_info.num_rows);

    println!("Press Enter to exit example");
    stdin().read_line(&mut input).unwrap();
}
