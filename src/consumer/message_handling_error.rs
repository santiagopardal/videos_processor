use std::io;
use crate::node::video_download_error::VideoDownloadError;

pub enum MessageHandlingError {
    VideoDownloadError,
    IoError,
    NodeConnectionError
}

impl From<io::Error> for MessageHandlingError {
    fn from(_: io::Error) -> Self {
        return MessageHandlingError::IoError;
    }
}

impl From<VideoDownloadError> for MessageHandlingError {
    fn from(_: VideoDownloadError) -> Self {
        return MessageHandlingError::VideoDownloadError;
    }
}

impl From<tonic::transport::Error> for MessageHandlingError {
    fn from(_: tonic::transport::Error) -> Self {
        return MessageHandlingError::NodeConnectionError;
    }
}
