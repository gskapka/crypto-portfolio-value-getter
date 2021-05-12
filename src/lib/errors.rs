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
        DocoptError(err: docopt::Error) {
            from()
            display("✘ Docopt error: {}", err)
        }
        ParseFloat(err: std::num::ParseFloatError) {
            from()
            display("✘ Parse float error: {}", err)
        }
        FromUtf8Error(err: std::str::Utf8Error) {
            from()
            display("✘ From UTF8 error: {}", err)
        }
        FromIoError(err: std::io::Error) {
            from()
            display("✘ From I/O error: {}", err)
        }
        EnvVarError(err: std::env::VarError) {
            from()
            display("✘ Environment variable error: {}", err)
        }
        NoneError(err: &'static str) {
            display("✘ None error: {}", err)
        }
    }
}
