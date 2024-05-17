use std::fmt::{Debug, Formatter};
use std::io;
use crate::consumer::node_creation_error::NodeCreationError;
use crate::node::video_download_error::VideoDownloadError;

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

impl Debug for MessageHandlingError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self  {
            MessageHandlingError::VideoDownloadError(error) => write!(f, "Error while downloading \
            video"),
            MessageHandlingError::IoError(error) => write!(f, "Error while saving video"),
            MessageHandlingError::NodeConnectionError(error) => write!(f, "Error connecting to \
            node")
        }
    }
}
