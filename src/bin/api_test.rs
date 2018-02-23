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

    thread::sleep(time::Duration::from_millis(1000));

    let pressed = device.get_power_button_pressed()?;
    println!("power_button_pressed: {}", pressed);

    device.set_led_brightness(0, 100)?;
    assert!(device.get_led_brightness(0)? == 100);

    device.soft_reset()?;

    device.set_scan_mode(sensel::scan_mode::SCAN_MODE_ASYNC)?;
    assert!(device.get_scan_mode()? == sensel::scan_mode::SCAN_MODE_ASYNC);

    device.soft_reset()?;

    device.set_scan_detail(sensel::scan_detail::SCAN_DETAIL_LOW)?;
    assert!(device.get_scan_detail()? == sensel::scan_detail::SCAN_DETAIL_LOW);

    device.soft_reset()?;

    device.set_buffer_control(42)?;
    assert!(device.get_buffer_control()? == 42);

    device.soft_reset()?;

    device.set_max_frame_rate(42)?;
    assert!(device.get_max_frame_rate()? == 42);

    device.soft_reset()?;

    device.set_frame_content(sensel::frame::Mask::ACCEL)?;
    assert!(device.get_frame_content()? == sensel::frame::Mask::ACCEL);

    device.soft_reset()?;

    device.set_contacts_mask(sensel::contact::Mask::PEAK)?;
    assert!(device.get_contacts_mask()? == sensel::contact::Mask::PEAK);

    device.soft_reset()?;

    device.set_contacts_min_force(42)?;
    assert!(device.get_contacts_min_force()? == 42);

    device.soft_reset()?;

    device.set_contacts_enable_blob_merge(false)?;
    assert!(device.get_contacts_enable_blob_merge()? == false);

    device.soft_reset()?;

    device.set_dynamic_baseline_enabled(false)?;
    assert!(device.get_dynamic_baseline_enabled()? == false);

    device.soft_reset()?;

    // device.write_reg()?;
    // device.read_reg()?;

    // device.write_reg_vs()?;
    // device.read_reg_vs()?;

    device.start_scanning()?; // TODO: make sensor struct

    device.read_sensor()?;

    let num_frames = device.get_num_available_frames()?;

    for _ in 0..num_frames {
        let frame = device.get_frame()?;
        println!("{:?}", frame)
    }

    device.stop_scanning()?; // TODO: consume sensor struct

    // implicitly closed when dropped
    // device.close();

    Ok(true)
}
