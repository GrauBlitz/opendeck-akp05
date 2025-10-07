use mirajazz::{
    device::DeviceQuery,
    types::{HidDeviceInfo, ImageFormat, ImageMirroring, ImageMode, ImageRotation},
};

// Must be unique between all the plugins, 2 characters long and match `DeviceNamespace` field in `manifest.json`
pub const DEVICE_NAMESPACE: &str = "n4";

pub const ROW_COUNT: usize = 3;
pub const COL_COUNT: usize = 5;
pub const KEY_COUNT: usize = 14;
pub const ENCODER_COUNT: usize = 4;

#[derive(Debug, Clone)]
pub enum Kind {
    N4
}

pub const MIRABOX_VID: u16 = 0x6603;

pub const N4_PID: u16 = 0x1007;

// Map all queries to usage page 65440 and usage id 1 for now
pub const N4_QUERY: DeviceQuery = DeviceQuery::new(65440, 1, MIRABOX_VID, N4_PID);

pub const QUERIES: [DeviceQuery; 1] = [
    N4_QUERY,
];

impl Kind {
    /// Matches devices VID+PID pairs to correct kinds
    pub fn from_vid_pid(vid: u16, pid: u16) -> Option<Self> {
        match vid {
            MIRABOX_VID => match pid {
                N4_PID => Some(Kind::N4),
                _ => None,
            },

            _ => None,
        }
    }

    /// There is no point relying on manufacturer/device names reported by the USB stack,
    /// so we return custom names for all the kinds of devices
    pub fn human_name(&self) -> String {
        match &self {
            Self::N4 => "Mirabox N4",
        }
        .to_string()
    }

    /// Returns protocol version for device
    pub fn protocol_version(&self) -> usize {
        match self {
            _ => 3,
        }
    }

    pub fn image_format(&self) -> ImageFormat {
        return ImageFormat {
            mode: ImageMode::JPEG,
            size: (112, 112),
            rotation: ImageRotation::Rot180,
            mirror: ImageMirroring::None,
        };
    }
    pub fn image_format_secondscreen(&self) -> ImageFormat {
        return ImageFormat {
            mode: ImageMode::JPEG,
            size: (176, 112),
            rotation: ImageRotation::Rot180,
            mirror: ImageMirroring::None,
        };
    }
}

#[derive(Debug, Clone)]
pub struct CandidateDevice {
    pub id: String,
    pub dev: HidDeviceInfo,
    pub kind: Kind,
}
