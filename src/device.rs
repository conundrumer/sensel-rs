use bindings::*;
use result::*;
use frame;

pub const MAX_DEVICES: usize = SENSEL_MAX_DEVICES as usize;

pub struct DeviceList(SenselDeviceList);

pub struct Device {
    handle: *mut ::std::os::raw::c_void,
    frame_data: *mut SenselFrameData,
    pub id: SenselDeviceID,
    pub sensor_info: SenselSensorInfo,
    pub fw_info: SenselFirmwareInfo
}

pub fn get_device_list() -> Result<DeviceList, SenselError> {
    unsafe {
        let mut list = ::std::mem::zeroed();
        sensel_result(senselGetDeviceList(&mut list))
            .and(Ok(list.into()))
    }
}

impl SenselSensorInfo {
    pub fn get_num_sensors(&self) -> usize {
        self.num_rows as usize * self.num_cols as usize
    }
}

impl DeviceList {
    pub fn as_slice(&self) -> &[SenselDeviceID] {
        &self.0.devices[..self.0.num_devices as usize]
    }
}

impl Into<DeviceList> for SenselDeviceList {
    fn into(self) -> DeviceList {
        DeviceList(self)
    }
}

impl SenselDeviceID {
    pub fn get_serial_num(&self) -> &str {
        ::std::str::from_utf8(&self.serial_num).unwrap()
    }
    pub fn get_com_port(&self) -> &str {
        ::std::str::from_utf8(&self.com_port).unwrap()
    }
    pub fn open(self) -> Result<Device, SenselError> {
        unsafe {
            let mut handle = ::std::mem::zeroed();
            let mut fw_info = ::std::mem::zeroed();
            let mut sensor_info = ::std::mem::zeroed();
            let mut frame_data = ::std::mem::zeroed();

            sensel_result(senselOpenDeviceByID(&mut handle, self.idx))
                .and_then(|_| sensel_result(senselGetFirmwareInfo(handle, &mut fw_info)))
                .and_then(|_| sensel_result(senselGetSensorInfo(handle, &mut sensor_info)))
                .and_then(|_| sensel_result(senselAllocateFrameData(handle, &mut frame_data)))
                .and(Ok(Device {
                    handle,
                    frame_data,
                    id: self,
                    sensor_info,
                    fw_info
                }))
        }
    }
}

impl Device {
    pub fn set_frame_content(&self, mask: frame::Mask) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetFrameContent(self.handle, mask.bits()))
        }
    }
    pub fn start_scanning(&self) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselStartScanning(self.handle))
        }
    }
    pub fn read_sensor(&self) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselReadSensor(self.handle))
        }
    }
    pub fn get_num_available_frames(&self) -> Result<usize, SenselError> {
        let mut num_frames = 0;
        unsafe {
            sensel_result(senselGetNumAvailableFrames(self.handle, &mut num_frames))
                .and(Ok(num_frames as usize))
        }
    }
    pub fn get_frame(&self) -> Result<frame::Frame, SenselError> {
        unsafe {
            sensel_result(senselGetFrame(self.handle, self.frame_data))
                .and(Ok(frame::from_frame_data(*self.frame_data, self.sensor_info)))
        }
    }
    pub fn set_led_brightness(&self, led_id: u8, brightness: u16) -> Result<(), SenselError> {
        unsafe {
            sensel_result(senselSetLEDBrightness(self.handle, led_id, brightness))
        }
    }
    pub fn close(self) -> () {
        // move and drop the device
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        unsafe {
            // always close on drop to prevent memory leaks
            // never throws error
            senselClose(self.handle);
        }
    }
}
