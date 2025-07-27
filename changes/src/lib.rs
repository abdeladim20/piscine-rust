#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Light {
	pub alias: String,
	pub brightness: u8,
}

impl Light {
    pub fn new(alias: &str) -> Light {
        Light {
            alias: alias.to_string(),
            brightness:0,
        }
    }
}



pub fn change_brightness(lights: &mut [Light], alias: &str, value: u8) {
    for val in lights {
        if val.alias == alias {
            val.brightness = value
        }
    }
}
