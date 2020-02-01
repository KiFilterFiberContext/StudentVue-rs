use std::borrow::Cow;

// POST request endpoint (ProcessWebServiceRequest is redundant for SOAP requests)
// In this case since we are solely making regular POST requests it is required
pub const ENDPOINT: &str = "/Service/PXPCommunication.asmx/ProcessWebServiceRequest";

/// Struct which connects to the StudentVUE service
pub struct Client<'a> {
    uri: Cow<'a, str>
}
