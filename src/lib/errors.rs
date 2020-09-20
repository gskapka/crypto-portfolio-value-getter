quick_error! {
    #[derive(Debug)]
    pub enum AppError {
        Custom(err: String) {
            from()
            from(err: &str) -> (err.into())
            display("✘ {}", err)
        }
        ReqwestErr(err: reqwest::Error) {
            from()
            display("✘ Reqwest error: {}", err)
        }
        SerdeJsonErr(err: serde_json::Error) {
            from()
            display("✘ Serde_json error: {}", err)
        }
        ParseFloat(err: std::num::ParseFloatError) {
            from()
            display("✘ Parse float error: {}", err)
        }
        NoneError(err: &'static str) {
            display("✘ None error: {}", err)
        }
    }
}
