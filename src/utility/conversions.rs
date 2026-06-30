pub enum BytesConversion {}

impl BytesConversion {
    pub fn from_mb(mb: f32) -> f32 {
        mb * 1000000.0
    }

    pub fn to_mb(bytes: f32) -> f32 {
        bytes / 1000000.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_mb_is_si_not_binary() {
        // 1 MB = 1_000_000 bytes (SI), not 1_048_576 (binary)
        assert_eq!(BytesConversion::from_mb(1.0), 1_000_000.0);
    }

    #[test]
    fn mb_round_trip() {
        let bytes = BytesConversion::from_mb(20.0);
        assert!((BytesConversion::to_mb(bytes) - 20.0).abs() < 1e-3);
    }
}
