extern crate sensel;

use sensel::*;
use std::mem::zeroed;
use std::io::stdin;
use std::thread::spawn;
use std::sync::mpsc::channel;

fn main() {
    unsafe {
        //Handle that references a Sensel device
        let mut handle = zeroed();
        //List of all available Sensel devices
        let mut list = zeroed();
        //Sensor info from the Sensel device
        let mut sensor_info = zeroed();
        //SenselFrame data that will hold the contacts
        let mut frame = zeroed();

        //Get a list of avaialble Sensel devices
        senselGetDeviceList(&mut list);
        if list.num_devices == 0 {
            println!("No device found");
            println!("Press Enter to exit example");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            return;
        }

        //Open a Sensel device by the id in the SenselDeviceList, handle initialized
        senselOpenDeviceByID(&mut handle, list.devices[0].idx);
        //Get the sensor info
        senselGetSensorInfo(handle, &mut sensor_info);

        //Set the frame content to scan contact data
        senselSetFrameContent(handle, FRAME_CONTENT_PRESSURE_MASK as u8);
        //Allocate a frame of data, must be done before reading frame data
        senselAllocateFrameData(handle, &mut frame);
        //Start scanning the Sensel device
        senselStartScanning(handle);

        println!("Press Enter to exit example");
        let (sender, receiver) = channel();
        spawn(move || {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            sender.send(()).unwrap();
        });

        while receiver.try_recv().is_err() {
            let mut num_frames = 0;
            //Read all available data from the Sensel device
            senselReadSensor(handle);
            //Get number of frames available in the data read from the sensor
            senselGetNumAvailableFrames(handle, &mut num_frames);
            for _ in 0..num_frames {
                //Read one frame of data
                senselGetFrame(handle, frame);
                let frame = *frame;
                //Calculate the total force
                let mut total_force = 0.0;
                for i in 0..sensor_info.num_cols*sensor_info.num_rows {
                    let force = *frame.force_array.offset(i as isize);
                    total_force = total_force + force;
                }
                println!("Total Force : {}", total_force);
            }
        }
    }
}
