use std::fmt::{Debug, Formatter};

pub enum VideoDownloadError {
    NotConnected,
    VideoDownload
}

impl From<tonic::Status> for VideoDownloadError {
    fn from(_: tonic::Status) -> Self {
        return VideoDownloadError::VideoDownload;
    }
}

impl Debug for VideoDownloadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while downloading video")
    }
}
