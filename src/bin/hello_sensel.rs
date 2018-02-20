extern crate sensel;

use sensel::*;
use std::mem::zeroed;
use std::io::stdin;
use std::str::from_utf8;

fn main() {
    unsafe {
        let mut input = String::new();

        //Handle that references a Sensel device
        let mut handle = zeroed();
        //List of all available Sensel devices
        let mut list = zeroed();
        //Firmware info from the Sensel device
        let mut fw_info = zeroed();
        //Sensor info from the Sensel device
        let mut sensor_info = zeroed();

        //Get a list of avaialble Sensel devices
        senselGetDeviceList(&mut list);
        if list.num_devices == 0 {
            println!("No device found");
            println!("Press Enter to exit example");
            stdin().read_line(&mut input).unwrap();
            return;
        }

        //Open a Sensel device by the id in the SenselDeviceList, handle initialized
        senselOpenDeviceByID(&mut handle, list.devices[0].idx);
        //Get the firmware info
        senselGetFirmwareInfo(handle, &mut fw_info);
        //Get the sensor info
        senselGetSensorInfo(handle, &mut sensor_info);

        println!("Sensel Device: {}" , from_utf8(&list.devices[0].serial_num).unwrap() );
        println!("Firmware Version: {}.{}.{}", fw_info.fw_version_major, fw_info.fw_version_minor, fw_info.fw_version_build);
        println!("Width: {}mm", sensor_info.width);
        println!("Height: {}mm", sensor_info.height);
        println!("Cols: {}", sensor_info.num_cols);
        println!("Rows: {}", sensor_info.num_rows);

        println!("Press Enter to exit example");
        stdin().read_line(&mut input).unwrap();

        return;
    }
}
