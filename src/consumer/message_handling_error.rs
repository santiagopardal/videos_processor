use std::{ fmt::Debug, io};
use crate::consumer::node_creation_error::NodeCreationError;
use crate::node::video_download_error::VideoDownloadError;

#[derive(Debug)]
pub enum MessageHandlingError {
    VideoDownloadError(VideoDownloadError),
    IoError(io::Error),
    NodeConnectionError(NodeCreationError)
}

impl From<io::Error> for MessageHandlingError {
    fn from(error: io::Error) -> Self {
        return MessageHandlingError::IoError(error);
    }
}

impl From<VideoDownloadError> for MessageHandlingError {
    fn from(error: VideoDownloadError) -> Self {
        return MessageHandlingError::VideoDownloadError(error);
    }
}

impl From<NodeCreationError> for MessageHandlingError {
    fn from(error: NodeCreationError) -> Self {
        return MessageHandlingError::NodeConnectionError(error);
    }
}
