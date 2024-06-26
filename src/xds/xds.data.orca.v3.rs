// @generated
// This file is @generated by prost-build.
// See section `ORCA load report format` of the design document in
// :ref:`<https://github.com/envoyproxy/envoy/issues/6614`.>

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrcaLoadReport {
    /// CPU utilization expressed as a fraction of available CPU resources. This
    /// should be derived from the latest sample or measurement. The value may be
    /// larger than 1.0 when the usage exceeds the reporter dependent notion of
    /// soft limits.
    #[prost(double, tag="1")]
    pub cpu_utilization: f64,
    /// Memory utilization expressed as a fraction of available memory
    /// resources. This should be derived from the latest sample or measurement.
    #[prost(double, tag="2")]
    pub mem_utilization: f64,
    /// Total RPS being served by an endpoint. This should cover all services that an endpoint is
    /// responsible for.
    /// Deprecated -- use ``rps_fractional`` field instead.
    #[deprecated]
    #[prost(uint64, tag="3")]
    pub rps: u64,
    /// Application specific requests costs. Each value is an absolute cost (e.g. 3487 bytes of
    /// storage) associated with the request.
    #[prost(map="string, double", tag="4")]
    pub request_cost: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    /// Resource utilization values. Each value is expressed as a fraction of total resources
    /// available, derived from the latest sample or measurement.
    #[prost(map="string, double", tag="5")]
    pub utilization: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    /// Total RPS being served by an endpoint. This should cover all services that an endpoint is
    /// responsible for.
    #[prost(double, tag="6")]
    pub rps_fractional: f64,
    /// Total EPS (errors/second) being served by an endpoint. This should cover
    /// all services that an endpoint is responsible for.
    #[prost(double, tag="7")]
    pub eps: f64,
    /// Application specific opaque metrics.
    #[prost(map="string, double", tag="8")]
    pub named_metrics: ::std::collections::HashMap<::prost::alloc::string::String, f64>,
    /// Application specific utilization expressed as a fraction of available
    /// resources. For example, an application may report the max of CPU and memory
    /// utilization for better load balancing if it is both CPU and memory bound.
    /// This should be derived from the latest sample or measurement.
    /// The value may be larger than 1.0 when the usage exceeds the reporter
    /// dependent notion of soft limits.
    #[prost(double, tag="9")]
    pub application_utilization: f64,
}
/// Encoded file descriptor set for the `xds.data.orca.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x8d, 0x1c, 0x0a, 0x27, 0x78, 0x64, 0x73, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x6f, 0x72,
    0x63, 0x61, 0x2f, 0x76, 0x33, 0x2f, 0x6f, 0x72, 0x63, 0x61, 0x5f, 0x6c, 0x6f, 0x61, 0x64, 0x5f,
    0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x10, 0x78, 0x64,
    0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x6f, 0x72, 0x63, 0x61, 0x2e, 0x76, 0x33, 0x1a, 0x17,
    0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74,
    0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xa6, 0x06, 0x0a, 0x0e, 0x4f, 0x72, 0x63, 0x61,
    0x4c, 0x6f, 0x61, 0x64, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x12, 0x37, 0x0a, 0x0f, 0x63, 0x70,
    0x75, 0x5f, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x01, 0x42, 0x0e, 0xfa, 0x42, 0x0b, 0x12, 0x09, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x52, 0x0e, 0x63, 0x70, 0x75, 0x55, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x12, 0x40, 0x0a, 0x0f, 0x6d, 0x65, 0x6d, 0x5f, 0x75, 0x74, 0x69, 0x6c, 0x69,
    0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x01, 0x42, 0x17, 0xfa, 0x42,
    0x14, 0x12, 0x12, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x29, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x0e, 0x6d, 0x65, 0x6d, 0x55, 0x74, 0x69, 0x6c, 0x69, 0x7a,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x14, 0x0a, 0x03, 0x72, 0x70, 0x73, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x04, 0x42, 0x02, 0x18, 0x01, 0x52, 0x03, 0x72, 0x70, 0x73, 0x12, 0x54, 0x0a, 0x0c, 0x72,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x5f, 0x63, 0x6f, 0x73, 0x74, 0x18, 0x04, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x31, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x6f, 0x72, 0x63,
    0x61, 0x2e, 0x76, 0x33, 0x2e, 0x4f, 0x72, 0x63, 0x61, 0x4c, 0x6f, 0x61, 0x64, 0x52, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x6f, 0x73, 0x74, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x52, 0x0b, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x6f, 0x73,
    0x74, 0x12, 0x71, 0x0a, 0x0b, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x18, 0x05, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x2e, 0x6f, 0x72, 0x63, 0x61, 0x2e, 0x76, 0x33, 0x2e, 0x4f, 0x72, 0x63, 0x61, 0x4c, 0x6f,
    0x61, 0x64, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x55, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x42, 0x1c, 0xfa, 0x42, 0x19, 0x9a, 0x01,
    0x16, 0x2a, 0x14, 0x12, 0x12, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x29, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x0b, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x12, 0x35, 0x0a, 0x0e, 0x72, 0x70, 0x73, 0x5f, 0x66, 0x72, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x18, 0x06, 0x20, 0x01, 0x28, 0x01, 0x42, 0x0e, 0xfa, 0x42,
    0x0b, 0x12, 0x09, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x0d, 0x72, 0x70,
    0x73, 0x46, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x12, 0x20, 0x0a, 0x03, 0x65,
    0x70, 0x73, 0x18, 0x07, 0x20, 0x01, 0x28, 0x01, 0x42, 0x0e, 0xfa, 0x42, 0x0b, 0x12, 0x09, 0x29,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x03, 0x65, 0x70, 0x73, 0x12, 0x57, 0x0a,
    0x0d, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x5f, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x18, 0x08,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x32, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x64, 0x61, 0x74, 0x61, 0x2e,
    0x6f, 0x72, 0x63, 0x61, 0x2e, 0x76, 0x33, 0x2e, 0x4f, 0x72, 0x63, 0x61, 0x4c, 0x6f, 0x61, 0x64,
    0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x2e, 0x4e, 0x61, 0x6d, 0x65, 0x64, 0x4d, 0x65, 0x74, 0x72,
    0x69, 0x63, 0x73, 0x45, 0x6e, 0x74, 0x72, 0x79, 0x52, 0x0c, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x4d,
    0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x12, 0x47, 0x0a, 0x17, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x5f, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f,
    0x6e, 0x18, 0x09, 0x20, 0x01, 0x28, 0x01, 0x42, 0x0e, 0xfa, 0x42, 0x0b, 0x12, 0x09, 0x29, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52, 0x16, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x55, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x1a,
    0x3e, 0x0a, 0x10, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x43, 0x6f, 0x73, 0x74, 0x45, 0x6e,
    0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a,
    0x3e, 0x0a, 0x10, 0x55, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x45, 0x6e,
    0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01, 0x1a,
    0x3f, 0x0a, 0x11, 0x4e, 0x61, 0x6d, 0x65, 0x64, 0x4d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x45,
    0x6e, 0x74, 0x72, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x01, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x3a, 0x02, 0x38, 0x01,
    0x42, 0xc8, 0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d, 0x2e, 0x78, 0x64, 0x73, 0x2e, 0x64, 0x61, 0x74,
    0x61, 0x2e, 0x6f, 0x72, 0x63, 0x61, 0x2e, 0x76, 0x33, 0x42, 0x13, 0x4f, 0x72, 0x63, 0x61, 0x4c,
    0x6f, 0x61, 0x64, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01,
    0x5a, 0x38, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x54, 0x68, 0x69,
    0x6e, 0x6b, 0x65, 0x69, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f,
    0x67, 0x6f, 0x2f, 0x78, 0x64, 0x73, 0x2f, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x6f, 0x72, 0x63, 0x61,
    0x2f, 0x76, 0x33, 0x3b, 0x6f, 0x72, 0x63, 0x61, 0x76, 0x33, 0xa2, 0x02, 0x03, 0x58, 0x44, 0x4f,
    0xaa, 0x02, 0x10, 0x58, 0x64, 0x73, 0x2e, 0x44, 0x61, 0x74, 0x61, 0x2e, 0x4f, 0x72, 0x63, 0x61,
    0x2e, 0x56, 0x33, 0xca, 0x02, 0x10, 0x58, 0x64, 0x73, 0x5c, 0x44, 0x61, 0x74, 0x61, 0x5c, 0x4f,
    0x72, 0x63, 0x61, 0x5c, 0x56, 0x33, 0xe2, 0x02, 0x1c, 0x58, 0x64, 0x73, 0x5c, 0x44, 0x61, 0x74,
    0x61, 0x5c, 0x4f, 0x72, 0x63, 0x61, 0x5c, 0x56, 0x33, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x13, 0x58, 0x64, 0x73, 0x3a, 0x3a, 0x44, 0x61, 0x74,
    0x61, 0x3a, 0x3a, 0x4f, 0x72, 0x63, 0x61, 0x3a, 0x3a, 0x56, 0x33, 0x4a, 0xba, 0x13, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x39, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x19, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x04, 0x00, 0x34, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x04, 0x00, 0x34, 0x0a,
    0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x05, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12,
    0x03, 0x05, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x09, 0x00, 0x21, 0x0a,
    0x86, 0x01, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0e, 0x00, 0x39, 0x01, 0x32, 0x7a, 0x20, 0x53,
    0x65, 0x65, 0x20, 0x73, 0x65, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x60, 0x4f, 0x52, 0x43, 0x41,
    0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x66, 0x6f, 0x72,
    0x6d, 0x61, 0x74, 0x60, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x64, 0x65, 0x73, 0x69,
    0x67, 0x6e, 0x20, 0x64, 0x6f, 0x63, 0x75, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x0a, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x70, 0x72,
    0x6f, 0x78, 0x79, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x69, 0x73, 0x73, 0x75, 0x65, 0x73,
    0x2f, 0x36, 0x36, 0x31, 0x34, 0x60, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x0e, 0x08, 0x16, 0x0a, 0xfa, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x13,
    0x02, 0x3f, 0x1a, 0xec, 0x01, 0x20, 0x43, 0x50, 0x55, 0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20,
    0x61, 0x73, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x43, 0x50, 0x55, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x0a, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65,
    0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73,
    0x74, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x61, 0x73,
    0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x0a, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x31, 0x2e, 0x30, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x65, 0x64,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x72, 0x20, 0x64,
    0x65, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x0a, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x13, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x13, 0x09, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x13, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x13, 0x1d, 0x3e, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xaf, 0x08, 0x02, 0x05, 0x12, 0x03, 0x13, 0x1e, 0x3d, 0x0a, 0x98, 0x01, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x60, 0x1a, 0x8a, 0x01, 0x20, 0x4d, 0x65, 0x6d,
    0x6f, 0x72, 0x79, 0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x66,
    0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c,
    0x61, 0x62, 0x6c, 0x65, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x0a, 0x20, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65, 0x64, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x73,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x17, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17,
    0x09, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03, 0x17, 0x1d, 0x5f, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x02, 0x05, 0x12, 0x03, 0x17, 0x1e, 0x3d,
    0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x02, 0x03, 0x12, 0x03, 0x17,
    0x3f, 0x5e, 0x0a, 0xb0, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x02, 0x25,
    0x1a, 0xa2, 0x01, 0x20, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x52, 0x50, 0x53, 0x20, 0x62, 0x65,
    0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20,
    0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x20, 0x61, 0x6c, 0x6c,
    0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61,
    0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x72,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x2e, 0x0a,
    0x20, 0x44, 0x65, 0x70, 0x72, 0x65, 0x63, 0x61, 0x74, 0x65, 0x64, 0x20, 0x2d, 0x2d, 0x20, 0x75,
    0x73, 0x65, 0x20, 0x60, 0x60, 0x72, 0x70, 0x73, 0x5f, 0x66, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x61, 0x6c, 0x60, 0x60, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x74,
    0x65, 0x61, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x1c, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x09,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1c, 0x0f, 0x10, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x08, 0x12, 0x03, 0x1c, 0x11, 0x24, 0x0a, 0x0d, 0x0a,
    0x06, 0x04, 0x00, 0x02, 0x02, 0x08, 0x03, 0x12, 0x03, 0x1c, 0x12, 0x23, 0x0a, 0x8e, 0x01, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x20, 0x02, 0x27, 0x1a, 0x80, 0x01, 0x20, 0x41, 0x70,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x63, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x63, 0x6f, 0x73, 0x74,
    0x73, 0x2e, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x61, 0x6e, 0x20, 0x61, 0x62, 0x73, 0x6f, 0x6c, 0x75, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x73,
    0x74, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x20, 0x33, 0x34, 0x38, 0x37, 0x20, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x20, 0x6f, 0x66, 0x0a, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x61, 0x67, 0x65, 0x29, 0x20,
    0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x20, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x16, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x20, 0x25, 0x26, 0x0a, 0xa1, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04,
    0x12, 0x04, 0x24, 0x02, 0x25, 0x5f, 0x1a, 0x92, 0x01, 0x20, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x20, 0x75, 0x74, 0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x76,
    0x61, 0x6c, 0x75, 0x65, 0x73, 0x2e, 0x20, 0x45, 0x61, 0x63, 0x68, 0x20, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x20, 0x69, 0x73, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73, 0x65, 0x64, 0x20, 0x61,
    0x73, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x0a,
    0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x2c, 0x20, 0x64, 0x65, 0x72, 0x69,
    0x76, 0x65, 0x64, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74,
    0x65, 0x73, 0x74, 0x20, 0x73, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x65,
    0x61, 0x73, 0x75, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x24, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x04, 0x01, 0x12, 0x03, 0x24, 0x16, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03,
    0x12, 0x03, 0x24, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x08, 0x12, 0x03,
    0x25, 0x06, 0x5e, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x04, 0x08, 0xaf, 0x08, 0x13, 0x05,
    0x02, 0x05, 0x12, 0x03, 0x25, 0x07, 0x31, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x04, 0x08,
    0xaf, 0x08, 0x13, 0x05, 0x02, 0x03, 0x12, 0x03, 0x25, 0x33, 0x5d, 0x0a, 0x7a, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x05, 0x12, 0x03, 0x29, 0x02, 0x3e, 0x1a, 0x6d, 0x20, 0x54, 0x6f, 0x74, 0x61, 0x6c,
    0x20, 0x52, 0x50, 0x53, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65,
    0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74,
    0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f,
    0x76, 0x65, 0x72, 0x20, 0x61, 0x6c, 0x6c, 0x20, 0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6e, 0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e,
    0x74, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x69, 0x62, 0x6c,
    0x65, 0x20, 0x66, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x05,
    0x12, 0x03, 0x29, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03,
    0x29, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x03, 0x12, 0x03, 0x29, 0x1a,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x08, 0x12, 0x03, 0x29, 0x1c, 0x3d, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x05, 0x08, 0xaf, 0x08, 0x02, 0x05, 0x12, 0x03, 0x29, 0x1d,
    0x3c, 0x0a, 0x8a, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12, 0x03, 0x2d, 0x02, 0x33, 0x1a,
    0x7d, 0x20, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x45, 0x50, 0x53, 0x20, 0x28, 0x65, 0x72, 0x72,
    0x6f, 0x72, 0x73, 0x2f, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x29, 0x20, 0x62, 0x65, 0x69, 0x6e,
    0x67, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x6e, 0x20, 0x65,
    0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68,
    0x6f, 0x75, 0x6c, 0x64, 0x20, 0x63, 0x6f, 0x76, 0x65, 0x72, 0x0a, 0x20, 0x61, 0x6c, 0x6c, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x69, 0x63, 0x65, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x61, 0x6e,
    0x20, 0x65, 0x6e, 0x64, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x69, 0x73, 0x20, 0x72, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x2d, 0x09, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x06, 0x03, 0x12, 0x03, 0x2d, 0x0f, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x06,
    0x08, 0x12, 0x03, 0x2d, 0x11, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x06, 0x08, 0xaf,
    0x08, 0x02, 0x05, 0x12, 0x03, 0x2d, 0x12, 0x31, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07,
    0x12, 0x03, 0x30, 0x02, 0x28, 0x1a, 0x26, 0x20, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x6f, 0x70, 0x61,
    0x71, 0x75, 0x65, 0x20, 0x6d, 0x65, 0x74, 0x72, 0x69, 0x63, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x07, 0x06, 0x12, 0x03, 0x30, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x30, 0x16, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x07, 0x03, 0x12, 0x03, 0x30, 0x26, 0x27, 0x0a, 0x94, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x08,
    0x12, 0x03, 0x38, 0x02, 0x47, 0x1a, 0x86, 0x03, 0x20, 0x41, 0x70, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20, 0x75, 0x74,
    0x69, 0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x65, 0x64, 0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x66, 0x72, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x20,
    0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x2e, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x65,
    0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65, 0x2c, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72,
    0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x6f, 0x66, 0x20, 0x43, 0x50, 0x55,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x0a, 0x20, 0x75, 0x74, 0x69,
    0x6c, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x65, 0x74,
    0x74, 0x65, 0x72, 0x20, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63, 0x69,
    0x6e, 0x67, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x69, 0x73, 0x20, 0x62, 0x6f, 0x74, 0x68,
    0x20, 0x43, 0x50, 0x55, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x20,
    0x62, 0x6f, 0x75, 0x6e, 0x64, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x73, 0x68, 0x6f,
    0x75, 0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65, 0x64, 0x20, 0x66,
    0x72, 0x6f, 0x6d, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74, 0x20, 0x73,
    0x61, 0x6d, 0x70, 0x6c, 0x65, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x61, 0x73, 0x75, 0x72, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x61, 0x72, 0x67, 0x65, 0x72, 0x20, 0x74,
    0x68, 0x61, 0x6e, 0x20, 0x31, 0x2e, 0x30, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x65, 0x64, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x72, 0x0a, 0x20, 0x64, 0x65, 0x70,
    0x65, 0x6e, 0x64, 0x65, 0x6e, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66,
    0x20, 0x73, 0x6f, 0x66, 0x74, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x73, 0x2e, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x08, 0x05, 0x12, 0x03, 0x38, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x38, 0x09, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x08, 0x03, 0x12, 0x03, 0x38, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x08,
    0x08, 0x12, 0x03, 0x38, 0x25, 0x46, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x08, 0x08, 0xaf,
    0x08, 0x02, 0x05, 0x12, 0x03, 0x38, 0x26, 0x45, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)