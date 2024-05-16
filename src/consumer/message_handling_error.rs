use std::io;
use crate::consumer::node_creation_error::NodeCreationError;
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

impl From<NodeCreationError> for MessageHandlingError {
    fn from(_: NodeCreationError) -> Self {
        return MessageHandlingError::NodeConnectionError;
    }
}
