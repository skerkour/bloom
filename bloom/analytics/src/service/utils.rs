use crate::{entities::DeviceType, Service};

impl Service {
    pub fn device_type_from_screen_size(&self, screen_width: i64, _screen_height: i64) -> DeviceType {
        if screen_width < 600 {
            return DeviceType::Phone;
        } else if screen_width > 600 && screen_width <= 960 {
            return DeviceType::Tablet;
        } else if screen_width > 960 && screen_width <= 2080 {
            return DeviceType::Desktop;
        } else {
            return DeviceType::Other;
        }
    }
}
