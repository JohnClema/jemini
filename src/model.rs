pub struct GeminiModel;

impl GeminiModel {
	pub const PRO: &'static str = "models/gemini-pro:generateContent";
	pub const PRO_VISION: &'static str = "models/gemini-pro-vision:generateContent";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemini_model_pro() {
        assert_eq!(GeminiModel::PRO, "models/gemini-pro:generateContent");
    }

    #[test]
    fn test_gemini_model_pro_vision() {
        assert_eq!(GeminiModel::PRO_VISION, "models/gemini-pro-vision:generateContent");
    }
}