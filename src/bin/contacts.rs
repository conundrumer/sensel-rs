extern crate sensel;

use sensel::*;
use std::mem::zeroed;
use std::io::stdin;
use std::thread::spawn;
use std::sync::mpsc::channel;

static CONTACT_STATE_STRING: [&'static str; 4] = [
    "CONTACT_INVALID",
    "CONTACT_START",
    "CONTACT_MOVE",
    "CONTACT_END"
];

fn main() {
    unsafe {
        //Handle that references a Sensel device
        let mut handle = zeroed();
        //List of all available Sensel devices
        let mut list = zeroed();
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

        //Set the frame content to scan contact data
        senselSetFrameContent(handle, FRAME_CONTENT_CONTACTS_MASK as u8);
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
                //Print out contact data
                let frame = *frame;
                if frame.n_contacts > 0 {
                    println!("Num Contacts: {}", frame.n_contacts);
                    for c in 0..frame.n_contacts {
                        let contact = *frame.contacts.offset(c as isize);
                        let state = contact.state;
                        println!("Contact ID: {} State: {}", contact.id, CONTACT_STATE_STRING[state as usize]);

                        //Turn on LED for CONTACT_START
                        if state == SenselContactState_CONTACT_START {
                            senselSetLEDBrightness(handle, contact.id, 100);
                        }
                        //Turn off LED for CONTACT_END
                        else if state == SenselContactState_CONTACT_END {
                            senselSetLEDBrightness(handle, contact.id, 0);
                        }
                    }
                    println!();
                }
            }
        }
    }
}
