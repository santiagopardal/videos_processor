use crate::api::api_call_error;

pub enum NodeCreationError {
    ConnectionError(tonic::transport::Error),
    NodeRetrievalError(api_call_error::APICallError)
}

impl From<tonic::transport::Error> for NodeCreationError {
    fn from(value: tonic::transport::Error) -> Self {
        return NodeCreationError::ConnectionError(value)
    }
}

impl From<api_call_error::APICallError> for NodeCreationError {
    fn from(value: api_call_error::APICallError) -> Self {
        return NodeCreationError::NodeRetrievalError(value)
    }
}
