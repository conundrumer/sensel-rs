extern crate sensel;

use std::{thread, time};

fn main() {
    loop {
        match run() {
            Ok(tested) => if tested { break },
            Err(_) => println!("Something went wrong with the device")
        }
        thread::sleep(time::Duration::from_millis(1000));
        println!()
    }
}

fn run() -> Result<bool, sensel::SenselError> {
    let list = sensel::device::get_device_list()?;

    let list_slice = list.as_slice();

    if list_slice.len() == 0 {
        println!("No device found");
        return Ok(false)
    }

    let device_id = list_slice[0];

    println!("device_id {} {}", device_id.get_com_port(), device_id.get_serial_num());

    let device = device_id.open()?;

    println!("{:?}", device.sensor_info);
    println!("{:?}", device.fw_info);
    println!("num_leds: {}", device.num_leds);
    println!("max_led_brightness: {}", device.max_led_brightness);
    println!("supported frame content: {:?}", device.supported_frame_content);

    // thread::sleep(time::Duration::from_millis(1000));

    let pressed = device.get_power_button_pressed()?;
    println!("power_button_pressed: {}", pressed);

    let tests: Vec<Box<Fn() -> Result<_, _>>> = vec![Box::new(|| {
        device.set_led_brightness(0, 100)?;
        assert!(device.get_led_brightness(0)? == 100);
        Ok(())
    }), Box::new(|| {
        device.set_scan_mode(sensel::scan_mode::SCAN_MODE_ASYNC)?;
        assert!(device.get_scan_mode()? == sensel::scan_mode::SCAN_MODE_ASYNC);
        Ok(())
    }), Box::new(|| {
        device.set_scan_detail(sensel::scan_detail::SCAN_DETAIL_LOW)?;
        assert!(device.get_scan_detail()? == sensel::scan_detail::SCAN_DETAIL_LOW);
        Ok(())
    }), Box::new(|| {
        device.set_buffer_control(42)?;
        assert!(device.get_buffer_control()? == 42);
        Ok(())
    }), Box::new(|| {
        device.set_max_frame_rate(42)?;
        assert!(device.get_max_frame_rate()? == 42);
        Ok(())
    }), Box::new(|| {
        device.set_frame_content(sensel::frame::Mask::ACCEL)?;
        assert!(device.get_frame_content()? == sensel::frame::Mask::ACCEL);
        Ok(())
    }), Box::new(|| {
        device.set_contacts_mask(sensel::contact::Mask::PEAK)?;
        assert!(device.get_contacts_mask()? == sensel::contact::Mask::PEAK);
        Ok(())
    }), Box::new(|| {
        device.set_contacts_min_force(42)?;
        assert!(device.get_contacts_min_force()? == 42);
        Ok(())
    }), Box::new(|| {
        device.set_contacts_enable_blob_merge(false)?;
        assert!(device.get_contacts_enable_blob_merge()? == false);
        Ok(())
    }), Box::new(|| {
        device.set_dynamic_baseline_enabled(false)?;
        assert!(device.get_dynamic_baseline_enabled()? == false);
        Ok(())
    })];

    for (i, test) in tests.iter().enumerate() {
        test()?;
        device.set_led_brightness(i as u8, 100)?;
    }

    device.set_led_array(&vec![100; device.num_leds])?;

    device.soft_reset()?;

    device.set_frame_content(sensel::frame::Mask::CONTACTS | sensel::frame::Mask::ACCEL)?;

    device.start_scanning()?; // TODO: make sensor struct

    device.read_sensor()?;

    let num_frames = device.get_num_available_frames()?;

    for _ in 0..num_frames {
        let frame = device.get_frame()?;
        println!("{:?}", frame)
    }

    device.stop_scanning()?; // TODO: consume sensor struct

    device.set_led_array(&vec![0; device.num_leds])?;

    // implicitly closed when dropped
    // device.close();

    Ok(true)
}
