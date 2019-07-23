use crate::capture::results::CaptureResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OtherResult {
    pub timestamp: i64,
}

impl OtherResult {
    pub fn new(timestamp: i64) -> Self {
        OtherResult { timestamp }
    }

    pub fn launch(timestamp: i64) -> CaptureResult {
        CaptureResult::Other(OtherResult::new(timestamp))
    }
}
