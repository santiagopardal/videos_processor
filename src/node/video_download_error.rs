use std::fmt::{Debug};

#[derive(Debug)]
pub enum VideoDownloadError {
    NotConnected,
    VideoDownload(tonic::Status)
}

impl From<tonic::Status> for VideoDownloadError {
    fn from(value: tonic::Status) -> Self {
        return VideoDownloadError::VideoDownload(value);
    }
}
