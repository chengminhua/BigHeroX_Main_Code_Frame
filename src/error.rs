/// 将程序所有的错误都放在这。
#[derive(Debug, Clone)]
pub enum BigHeroXError {
    DialogClosed,
    IoError(std::io::ErrorKind),
    OpenCVError(OpenCVError),
}

impl From<std::io::Error> for BigHeroXError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value.kind())
    }
}

impl From<std::io::ErrorKind> for BigHeroXError {
    fn from(value: std::io::ErrorKind) -> Self {
        Self::IoError(value)
    }
}

impl From<OpenCVError> for BigHeroXError {
    fn from(value: OpenCVError) -> Self {
        Self::OpenCVError(value)
    }
}

/// OpenCV Error Wrapping. (因为opencv::Error目前不是Debug/Clone的)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpenCVError {
    pub code: i32,
    pub message: String,
}

/// 重置Result
#[allow(unused)]
pub type BigHeroXResult<T> = Result<T, BigHeroXError>;
