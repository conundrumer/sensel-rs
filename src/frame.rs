use std::iter;
use std::slice;

use bindings::*;
use contact::Contact;

bitflags! {
    pub struct Mask: u8 {
        #[cfg(feature = "forces")]
        const PRESSURE = FRAME_CONTENT_PRESSURE_MASK as u8;
        #[cfg(feature = "forces")]
        const LABELS = FRAME_CONTENT_LABELS_MASK as u8;
        const CONTACTS = FRAME_CONTENT_CONTACTS_MASK as u8;
        const ACCEL = FRAME_CONTENT_ACCEL_MASK as u8;
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Frame<'a> {
    pub lost_frame_count: i32,
    contacts: Option<&'a [SenselContact]>,
    #[cfg(feature = "forces")]
    pub force_array: Option<&'a [f32]>,
    #[cfg(feature = "forces")]
    pub labels_array: Option<&'a [u8]>,
    pub accel_data: Option<SenselAccelData>
}


type MapContact = fn(&SenselContact) -> Contact;
fn map_contact (&c: &SenselContact) -> Contact { c.into() }

impl <'a> Frame<'a> {
    pub fn get_contacts(&self) -> Option<iter::Map<slice::Iter<SenselContact>, MapContact>> {
        self.contacts.map(|contacts| contacts.iter().map(map_contact as MapContact))
    }
}

pub(crate) fn from_frame_data<'a>(data: SenselFrameData, sensor: SenselSensorInfo) -> Frame<'a> {
    let SenselFrameData {
        content_bit_mask,
        lost_frame_count,
        n_contacts,
        contacts,
        #[cfg(feature = "forces")]
        force_array,
        #[cfg(feature = "forces")]
        labels_array,
        accel_data,
        ..
    } = data;

    let mask = Mask::from_bits_truncate(content_bit_mask);

    let contacts = if mask.contains(Mask::CONTACTS) {
        unsafe {
            Some(slice::from_raw_parts(contacts, n_contacts as usize))
        }
    } else {
        None
    };

    #[cfg(not(feature = "forces"))]
    let _ = sensor;

    #[cfg(feature = "forces")]
    let force_array = if mask.contains(Mask::PRESSURE) {
        unsafe {
            Some(slice::from_raw_parts(force_array, sensor.get_num_sensors()))
        }
    } else {
        None
    };

    #[cfg(feature = "forces")]
    let labels_array = if mask.contains(Mask::LABELS) {
        unsafe {
            Some(slice::from_raw_parts(labels_array, sensor.get_num_sensors()))
        }
    } else {
        None
    };

    let accel_data = if mask.contains(Mask::ACCEL) {
        unsafe {
            Some(*accel_data)
        }
    } else {
        None
    };

    Frame {
        lost_frame_count,
        contacts,
        #[cfg(feature = "forces")]
        force_array,
        #[cfg(feature = "forces")]
        labels_array,
        accel_data
    }
}
