// @generated
// This file is @generated by prost-build.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Foo {
    #[prost(string, optional, tag="1")]
    pub bar: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag="2")]
    pub baz: ::core::option::Option<::prost_types::Any>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FooCopy {
    #[prost(string, optional, tag="1")]
    pub bar: ::core::option::Option<::prost::alloc::string::String>,
}
/// Encoded file descriptor set for the `testdata.dynamic_descriptors` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc2, 0x05, 0x0a, 0x35, 0x62, 0x61, 0x7a, 0x65, 0x6c, 0x2f, 0x63, 0x63, 0x5f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x5f, 0x6c,
    0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2f,
    0x74, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x74, 0x65, 0x73, 0x74,
    0x64, 0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
    0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x22, 0x45, 0x0a, 0x03, 0x46, 0x6f, 0x6f, 0x12, 0x10, 0x0a, 0x03, 0x62, 0x61,
    0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x62, 0x61, 0x72, 0x12, 0x26, 0x0a, 0x03,
    0x62, 0x61, 0x7a, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x52,
    0x03, 0x62, 0x61, 0x7a, 0x2a, 0x04, 0x08, 0x0a, 0x10, 0x15, 0x42, 0x87, 0x02, 0x0a, 0x20, 0x63,
    0x6f, 0x6d, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x61,
    0x6d, 0x69, 0x63, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x42,
    0x09, 0x54, 0x65, 0x73, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4b, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x54, 0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69,
    0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x62,
    0x61, 0x7a, 0x65, 0x6c, 0x2f, 0x63, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79,
    0x2f, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0xa2, 0x02, 0x03, 0x54, 0x44, 0x58, 0xaa,
    0x02, 0x1b, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x44, 0x79, 0x6e, 0x61, 0x6d,
    0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0xca, 0x02, 0x1b,
    0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x5c, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63,
    0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0xe2, 0x02, 0x27, 0x54, 0x65,
    0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x5c, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65,
    0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74,
    0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1c, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61,
    0x3a, 0x3a, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x6f, 0x72, 0x73, 0x4a, 0xfe, 0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x0b, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x02, 0x00, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x04, 0x00, 0x25, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x0b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x06, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x07, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x07, 0x02,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x12, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x08, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x08, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x08, 0x0b, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x08,
    0x1f, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x08, 0x25, 0x26,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x05, 0x12, 0x03, 0x0a, 0x02, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x05, 0x00, 0x12, 0x03, 0x0a, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x05,
    0x00, 0x01, 0x12, 0x03, 0x0a, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x05, 0x00, 0x02,
    0x12, 0x03, 0x0a, 0x13, 0x15, 0x0a, 0xdf, 0x04, 0x0a, 0x3f, 0x62, 0x61, 0x7a, 0x65, 0x6c, 0x2f,
    0x63, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70,
    0x74, 0x6f, 0x72, 0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x74, 0x65, 0x73, 0x74,
    0x64, 0x61, 0x74, 0x61, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x2d, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73,
    0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x1c, 0x74, 0x65, 0x73, 0x74, 0x64,
    0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x1a, 0x35, 0x62, 0x61, 0x7a, 0x65, 0x6c, 0x2f, 0x63,
    0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x64,
    0x61, 0x74, 0x61, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x3a, 0x39,
    0x0a, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x21, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x64,
    0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x64, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x2e, 0x46, 0x6f, 0x6f, 0x18, 0x0b, 0x20, 0x01, 0x28,
    0x03, 0x52, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x42, 0x90, 0x02, 0x0a, 0x20, 0x63, 0x6f,
    0x6d, 0x2e, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d,
    0x69, 0x63, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x42, 0x12,
    0x54, 0x65, 0x73, 0x74, 0x45, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4b, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d,
    0x2f, 0x54, 0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x62, 0x61, 0x7a, 0x65, 0x6c, 0x2f, 0x63, 0x63, 0x5f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74,
    0x61, 0xa2, 0x02, 0x03, 0x54, 0x44, 0x58, 0xaa, 0x02, 0x1b, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61,
    0x74, 0x61, 0x2e, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x73, 0xca, 0x02, 0x1b, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61,
    0x5c, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x73, 0xe2, 0x02, 0x27, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x5c, 0x44,
    0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72,
    0x73, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x1c,
    0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x3a, 0x3a, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69,
    0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x4a, 0x79, 0x0a, 0x06,
    0x12, 0x04, 0x00, 0x00, 0x08, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12,
    0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x25, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00,
    0x12, 0x03, 0x04, 0x00, 0x3f, 0x0a, 0x09, 0x0a, 0x01, 0x07, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01,
    0x0a, 0x09, 0x0a, 0x02, 0x07, 0x00, 0x12, 0x03, 0x07, 0x02, 0x1d, 0x0a, 0x0a, 0x0a, 0x03, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x04, 0x12, 0x03,
    0x07, 0x02, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x07, 0x00, 0x05, 0x12, 0x03, 0x07, 0x0b, 0x10, 0x0a,
    0x0a, 0x0a, 0x03, 0x07, 0x00, 0x01, 0x12, 0x03, 0x07, 0x11, 0x17, 0x0a, 0x0a, 0x0a, 0x03, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x07, 0x1a, 0x1c, 0x0a, 0xb5, 0x04, 0x0a, 0x36, 0x62, 0x61, 0x7a, 0x65,
    0x6c, 0x2f, 0x63, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72,
    0x69, 0x70, 0x74, 0x6f, 0x72, 0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x74, 0x65,
    0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x31, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x1c, 0x74, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e,
    0x61, 0x6d, 0x69, 0x63, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73,
    0x22, 0x21, 0x0a, 0x07, 0x46, 0x6f, 0x6f, 0x43, 0x6f, 0x70, 0x79, 0x12, 0x10, 0x0a, 0x03, 0x62,
    0x61, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x62, 0x61, 0x72, 0x2a, 0x04, 0x08,
    0x0a, 0x10, 0x15, 0x42, 0x88, 0x02, 0x0a, 0x20, 0x63, 0x6f, 0x6d, 0x2e, 0x74, 0x65, 0x73, 0x74,
    0x64, 0x61, 0x74, 0x61, 0x2e, 0x64, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x5f, 0x64, 0x65, 0x73,
    0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x42, 0x0a, 0x54, 0x65, 0x73, 0x74, 0x31, 0x50,
    0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4b, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63,
    0x6f, 0x6d, 0x2f, 0x54, 0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x62, 0x61, 0x7a, 0x65, 0x6c, 0x2f, 0x63,
    0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x5f, 0x6c, 0x69, 0x62, 0x72, 0x61, 0x72, 0x79, 0x2f, 0x74, 0x65, 0x73, 0x74, 0x64,
    0x61, 0x74, 0x61, 0xa2, 0x02, 0x03, 0x54, 0x44, 0x58, 0xaa, 0x02, 0x1b, 0x54, 0x65, 0x73, 0x74,
    0x64, 0x61, 0x74, 0x61, 0x2e, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63,
    0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0xca, 0x02, 0x1b, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61,
    0x74, 0x61, 0x5c, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x73, 0xe2, 0x02, 0x27, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61,
    0x5c, 0x44, 0x79, 0x6e, 0x61, 0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x73, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea,
    0x02, 0x1c, 0x54, 0x65, 0x73, 0x74, 0x64, 0x61, 0x74, 0x61, 0x3a, 0x3a, 0x44, 0x79, 0x6e, 0x61,
    0x6d, 0x69, 0x63, 0x44, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74, 0x6f, 0x72, 0x73, 0x4a, 0xae,
    0x01, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x08, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x25, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x00, 0x12, 0x04, 0x04, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01,
    0x12, 0x03, 0x04, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05,
    0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x05, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x12, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x05, 0x12, 0x03, 0x07, 0x02, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x05, 0x00, 0x12, 0x03,
    0x07, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x05, 0x00, 0x01, 0x12, 0x03, 0x07, 0x0d,
    0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x05, 0x00, 0x02, 0x12, 0x03, 0x07, 0x13, 0x15,
];
// @@protoc_insertion_point(module)