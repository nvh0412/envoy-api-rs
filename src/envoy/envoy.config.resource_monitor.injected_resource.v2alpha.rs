// @generated
// This file is @generated by prost-build.
// \[#protodoc-title: Injected resource\]
// \[#extension: envoy.resource_monitors.injected_resource\]

/// The injected resource monitor allows injecting a synthetic resource pressure into Envoy
/// via a text file, which must contain a floating-point number in the range \[0..1\] representing
/// the resource pressure and be updated atomically by a symbolic link swap.
/// This is intended primarily for integration tests to force Envoy into an overloaded state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InjectedResourceConfig {
    #[prost(string, tag="1")]
    pub filename: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `envoy.config.resource_monitor.injected_resource.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xfb, 0x0a, 0x0a, 0x4f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74,
    0x6f, 0x72, 0x2f, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2f, 0x69, 0x6e, 0x6a,
    0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x37, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69,
    0x74, 0x6f, 0x72, 0x2e, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x1a, 0x1d, 0x75,
    0x64, 0x70, 0x61, 0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61,
    0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3d, 0x0a, 0x16, 0x49, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12,
    0x23, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x65,
    0x6e, 0x61, 0x6d, 0x65, 0x42, 0xc8, 0x03, 0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x02, 0x0a,
    0x3b, 0x63, 0x6f, 0x6d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74,
    0x6f, 0x72, 0x2e, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x15, 0x49, 0x6e,
    0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x50, 0x72,
    0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x71, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f,
    0x6d, 0x2f, 0x54, 0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f,
    0x6e, 0x69, 0x74, 0x6f, 0x72, 0x2f, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x3b,
    0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xa2, 0x02, 0x04, 0x45, 0x43, 0x52, 0x49, 0xaa,
    0x02, 0x35, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x52,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x49,
    0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2e,
    0x56, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xca, 0x02, 0x35, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x5c,
    0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d,
    0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x5c, 0x49, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x52,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5c, 0x56, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xe2,
    0x02, 0x41, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x52,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x5c, 0x49,
    0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5c,
    0x56, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64,
    0x61, 0x74, 0x61, 0xea, 0x02, 0x39, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x3a, 0x3a, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x3a, 0x3a, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d, 0x6f, 0x6e,
    0x69, 0x74, 0x6f, 0x72, 0x3a, 0x3a, 0x49, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x52, 0x65,
    0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x3a, 0x3a, 0x56, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x4a,
    0xa4, 0x05, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x16, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x40, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x05, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x36, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x08, 0x00, 0x36, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0x87, 0x80, 0x99,
    0x6a, 0x02, 0x12, 0x03, 0x0b, 0x00, 0x46, 0x0a, 0xca, 0x03, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04,
    0x14, 0x00, 0x16, 0x01, 0x1a, 0xdc, 0x02, 0x20, 0x54, 0x68, 0x65, 0x20, 0x69, 0x6e, 0x6a, 0x65,
    0x63, 0x74, 0x65, 0x64, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6d, 0x6f,
    0x6e, 0x69, 0x74, 0x6f, 0x72, 0x20, 0x61, 0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x69, 0x6e, 0x6a,
    0x65, 0x63, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x20, 0x73, 0x79, 0x6e, 0x74, 0x68, 0x65, 0x74,
    0x69, 0x63, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x75, 0x72, 0x65, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x0a,
    0x20, 0x76, 0x69, 0x61, 0x20, 0x61, 0x20, 0x74, 0x65, 0x78, 0x74, 0x20, 0x66, 0x69, 0x6c, 0x65,
    0x2c, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x63, 0x6f, 0x6e,
    0x74, 0x61, 0x69, 0x6e, 0x20, 0x61, 0x20, 0x66, 0x6c, 0x6f, 0x61, 0x74, 0x69, 0x6e, 0x67, 0x2d,
    0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x69, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x5b, 0x30, 0x2e, 0x2e, 0x31, 0x5d,
    0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x67, 0x0a, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x70, 0x72, 0x65, 0x73,
    0x73, 0x75, 0x72, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x62, 0x65, 0x20, 0x75, 0x70, 0x64, 0x61,
    0x74, 0x65, 0x64, 0x20, 0x61, 0x74, 0x6f, 0x6d, 0x69, 0x63, 0x61, 0x6c, 0x6c, 0x79, 0x20, 0x62,
    0x79, 0x20, 0x61, 0x20, 0x73, 0x79, 0x6d, 0x62, 0x6f, 0x6c, 0x69, 0x63, 0x20, 0x6c, 0x69, 0x6e,
    0x6b, 0x20, 0x73, 0x77, 0x61, 0x70, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73,
    0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72,
    0x69, 0x6c, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x67, 0x72, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x74, 0x65, 0x73, 0x74, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x66, 0x6f, 0x72,
    0x63, 0x65, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x61, 0x6e,
    0x20, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x65, 0x64, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x65, 0x2e, 0x0a, 0x32, 0x5f, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63,
    0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x49, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78,
    0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x72,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x73,
    0x2e, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x41, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x15, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x15, 0x14, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08,
    0x12, 0x03, 0x15, 0x16, 0x40, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08,
    0x0e, 0x12, 0x03, 0x15, 0x17, 0x3f, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)