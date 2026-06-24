pub enum BytesConversion {}

impl BytesConversion {
    pub fn from_gb(gb: f32) -> f32 {
        gb * 1000000000.0
    }

    pub fn from_mb(mb: f32) -> f32 {
        mb * 1000000.0
    }

    pub fn to_gb(bytes: f32) -> f32 {
        bytes / 1000000000.0
    }

    pub fn to_mb(bytes: f32) -> f32 {
        bytes / 1000000.0
    }
}
