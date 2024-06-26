// @generated
// This file is @generated by prost-build.
/// A TypedStruct contains an arbitrary JSON serialized protocol buffer message with a URL that
/// describes the type of the serialized message. This is very similar to google.protobuf.Any,
/// instead of having protocol buffer binary, this employs google.protobuf.Struct as value.
///
/// This message is intended to be embedded inside Any, so it shouldn't be directly referred
/// from other UDPA messages.
///
/// When packing an opaque extension config, packing the expected type into Any is preferred
/// wherever possible for its efficiency. TypedStruct should be used only if a proto descriptor
/// is not available, for example if:
/// - A control plane sends opaque message that is originally from external source in human readable
///    format such as JSON or YAML.
/// - The control plane doesn't have the knowledge of the protocol buffer schema hence it cannot
///    serialize the message in protocol buffer binary format.
/// - The DPLB doesn't have have the knowledge of the protocol buffer schema its plugin or extension
///    uses. This has to be indicated in the DPLB capability negotiation.
///
/// When a DPLB receives a TypedStruct in Any, it should:
/// - Check if the type_url of the TypedStruct matches the type the extension expects.
/// - Convert value to the type described in type_url and perform validation.
/// TODO(lizan): Figure out how TypeStruct should be used with DPLB extensions that doesn't link
/// protobuf descriptor with DPLB itself, (e.g. gRPC LB Plugin, Envoy WASM extensions).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TypedStruct {
    /// A URL that uniquely identifies the type of the serialize protocol buffer message.
    /// This has same semantics and format described in google.protobuf.Any:
    /// <https://github.com/protocolbuffers/protobuf/blob/master/src/google/protobuf/any.proto>
    #[prost(string, tag="1")]
    pub type_url: ::prost::alloc::string::String,
    /// A JSON representation of the above specified type.
    #[prost(message, optional, tag="2")]
    pub value: ::core::option::Option<::prost_types::Struct>,
}
/// Encoded file descriptor set for the `udpa.type.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x9c, 0x13, 0x0a, 0x1f, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76,
    0x31, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0c, 0x75, 0x64, 0x70, 0x61, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e,
    0x76, 0x31, 0x1a, 0x1c, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72, 0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x22, 0x57, 0x0a, 0x0b, 0x54, 0x79, 0x70, 0x65, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x12,
    0x19, 0x0a, 0x08, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x07, 0x74, 0x79, 0x70, 0x65, 0x55, 0x72, 0x6c, 0x12, 0x2d, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67,
    0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75,
    0x63, 0x74, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x42, 0xac, 0x01, 0x0a, 0x10, 0x63, 0x6f,
    0x6d, 0x2e, 0x75, 0x64, 0x70, 0x61, 0x2e, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x76, 0x31, 0x42, 0x10,
    0x54, 0x79, 0x70, 0x65, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x34, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x54,
    0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69, 0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x75, 0x64, 0x70, 0x61, 0x2f, 0x74, 0x79, 0x70, 0x65, 0x2f, 0x76,
    0x31, 0x3b, 0x74, 0x79, 0x70, 0x65, 0x76, 0x31, 0xa2, 0x02, 0x03, 0x55, 0x54, 0x58, 0xaa, 0x02,
    0x0c, 0x55, 0x64, 0x70, 0x61, 0x2e, 0x54, 0x79, 0x70, 0x65, 0x2e, 0x56, 0x31, 0xca, 0x02, 0x0c,
    0x55, 0x64, 0x70, 0x61, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x5c, 0x56, 0x31, 0xe2, 0x02, 0x18, 0x55,
    0x64, 0x70, 0x61, 0x5c, 0x54, 0x79, 0x70, 0x65, 0x5c, 0x56, 0x31, 0x5c, 0x47, 0x50, 0x42, 0x4d,
    0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x0e, 0x55, 0x64, 0x70, 0x61, 0x3a, 0x3a,
    0x54, 0x79, 0x70, 0x65, 0x3a, 0x3a, 0x56, 0x31, 0x4a, 0xbc, 0x10, 0x0a, 0x06, 0x12, 0x04, 0x04,
    0x00, 0x2d, 0x01, 0x0a, 0x8e, 0x01, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x04, 0x00, 0x12, 0x32, 0x83,
    0x01, 0x20, 0x54, 0x48, 0x49, 0x53, 0x20, 0x46, 0x49, 0x4c, 0x45, 0x20, 0x49, 0x53, 0x20, 0x44,
    0x45, 0x50, 0x52, 0x45, 0x43, 0x41, 0x54, 0x45, 0x44, 0x0a, 0x20, 0x55, 0x73, 0x65, 0x72, 0x73,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x20,
    0x75, 0x73, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x63, 0x6f, 0x72, 0x72, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x78, 0x64, 0x73, 0x20, 0x74, 0x72, 0x65, 0x65, 0x2e, 0x0a, 0x20, 0x4e, 0x6f,
    0x20, 0x6e, 0x65, 0x77, 0x20, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x63, 0x63, 0x65, 0x70, 0x74, 0x65, 0x64, 0x20, 0x68, 0x65,
    0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x06, 0x00, 0x15, 0x0a, 0x08,
    0x0a, 0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x31, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03,
    0x08, 0x00, 0x31, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a,
    0x02, 0x08, 0x0a, 0x12, 0x03, 0x09, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03,
    0x0d, 0x00, 0x26, 0x0a, 0xbd, 0x0b, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x25, 0x00, 0x2d, 0x01,
    0x1a, 0xb0, 0x0b, 0x20, 0x41, 0x20, 0x54, 0x79, 0x70, 0x65, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63,
    0x74, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x72,
    0x62, 0x69, 0x74, 0x72, 0x61, 0x72, 0x79, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x20, 0x73, 0x65, 0x72,
    0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c,
    0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x77, 0x69, 0x74, 0x68, 0x20, 0x61, 0x20, 0x55, 0x52, 0x4c, 0x20, 0x74, 0x68, 0x61, 0x74, 0x0a,
    0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61,
    0x6c, 0x69, 0x7a, 0x65, 0x64, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x76, 0x65, 0x72, 0x79, 0x20, 0x73, 0x69, 0x6d, 0x69,
    0x6c, 0x61, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x2c, 0x0a, 0x20, 0x69, 0x6e, 0x73,
    0x74, 0x65, 0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x68, 0x61, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x62,
    0x69, 0x6e, 0x61, 0x72, 0x79, 0x2c, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x65, 0x6d, 0x70, 0x6c,
    0x6f, 0x79, 0x73, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x61, 0x73, 0x20, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x2e, 0x0a, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x20, 0x69, 0x73, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20,
    0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x65, 0x6d, 0x62, 0x65, 0x64, 0x64, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x73, 0x69, 0x64, 0x65, 0x20, 0x41, 0x6e, 0x79, 0x2c, 0x20, 0x73, 0x6f, 0x20, 0x69, 0x74,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x6e, 0x27, 0x74, 0x20, 0x62, 0x65, 0x20, 0x64, 0x69,
    0x72, 0x65, 0x63, 0x74, 0x6c, 0x79, 0x20, 0x72, 0x65, 0x66, 0x65, 0x72, 0x72, 0x65, 0x64, 0x0a,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x55, 0x44, 0x50, 0x41,
    0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x20, 0x57, 0x68, 0x65,
    0x6e, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x61, 0x6e, 0x20, 0x6f, 0x70, 0x61,
    0x71, 0x75, 0x65, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2c, 0x20, 0x70, 0x61, 0x63, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x65, 0x78, 0x70, 0x65, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20,
    0x69, 0x6e, 0x74, 0x6f, 0x20, 0x41, 0x6e, 0x79, 0x20, 0x69, 0x73, 0x20, 0x70, 0x72, 0x65, 0x66,
    0x65, 0x72, 0x72, 0x65, 0x64, 0x0a, 0x20, 0x77, 0x68, 0x65, 0x72, 0x65, 0x76, 0x65, 0x72, 0x20,
    0x70, 0x6f, 0x73, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x69, 0x74, 0x73,
    0x20, 0x65, 0x66, 0x66, 0x69, 0x63, 0x69, 0x65, 0x6e, 0x63, 0x79, 0x2e, 0x20, 0x54, 0x79, 0x70,
    0x65, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x69, 0x66, 0x20,
    0x61, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x61, 0x76, 0x61, 0x69, 0x6c,
    0x61, 0x62, 0x6c, 0x65, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c,
    0x65, 0x20, 0x69, 0x66, 0x3a, 0x0a, 0x20, 0x2d, 0x20, 0x41, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72,
    0x6f, 0x6c, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x73, 0x20, 0x6f,
    0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x68,
    0x61, 0x74, 0x20, 0x69, 0x73, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x61, 0x6c, 0x6c, 0x79,
    0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x65, 0x78, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x69, 0x6e, 0x20, 0x68, 0x75, 0x6d, 0x61, 0x6e, 0x20, 0x72,
    0x65, 0x61, 0x64, 0x61, 0x62, 0x6c, 0x65, 0x0a, 0x20, 0x20, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61,
    0x74, 0x20, 0x73, 0x75, 0x63, 0x68, 0x20, 0x61, 0x73, 0x20, 0x4a, 0x53, 0x4f, 0x4e, 0x20, 0x6f,
    0x72, 0x20, 0x59, 0x41, 0x4d, 0x4c, 0x2e, 0x0a, 0x20, 0x2d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x63,
    0x6f, 0x6e, 0x74, 0x72, 0x6f, 0x6c, 0x20, 0x70, 0x6c, 0x61, 0x6e, 0x65, 0x20, 0x64, 0x6f, 0x65,
    0x73, 0x6e, 0x27, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x6e,
    0x6f, 0x77, 0x6c, 0x65, 0x64, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x73,
    0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x68, 0x65, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x74, 0x20, 0x63,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x20, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69,
    0x7a, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x69,
    0x6e, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65,
    0x72, 0x20, 0x62, 0x69, 0x6e, 0x61, 0x72, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x2e,
    0x0a, 0x20, 0x2d, 0x20, 0x54, 0x68, 0x65, 0x20, 0x44, 0x50, 0x4c, 0x42, 0x20, 0x64, 0x6f, 0x65,
    0x73, 0x6e, 0x27, 0x74, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x68, 0x61, 0x76, 0x65, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6b, 0x6e, 0x6f, 0x77, 0x6c, 0x65, 0x64, 0x67, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66,
    0x66, 0x65, 0x72, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x69, 0x74, 0x73, 0x20, 0x70,
    0x6c, 0x75, 0x67, 0x69, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69,
    0x6f, 0x6e, 0x0a, 0x20, 0x20, 0x20, 0x75, 0x73, 0x65, 0x73, 0x2e, 0x20, 0x54, 0x68, 0x69, 0x73,
    0x20, 0x68, 0x61, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e, 0x64, 0x69, 0x63,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x44, 0x50, 0x4c, 0x42,
    0x20, 0x63, 0x61, 0x70, 0x61, 0x62, 0x69, 0x6c, 0x69, 0x74, 0x79, 0x20, 0x6e, 0x65, 0x67, 0x6f,
    0x74, 0x69, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20,
    0x61, 0x20, 0x44, 0x50, 0x4c, 0x42, 0x20, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x65, 0x73, 0x20,
    0x61, 0x20, 0x54, 0x79, 0x70, 0x65, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x69, 0x6e,
    0x20, 0x41, 0x6e, 0x79, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x3a,
    0x0a, 0x20, 0x2d, 0x20, 0x43, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x54, 0x79, 0x70, 0x65, 0x64, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x6d, 0x61, 0x74,
    0x63, 0x68, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x78, 0x70, 0x65,
    0x63, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x2d, 0x20, 0x43, 0x6f, 0x6e, 0x76, 0x65, 0x72, 0x74, 0x20,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x5f, 0x75, 0x72, 0x6c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x70, 0x65, 0x72, 0x66,
    0x6f, 0x72, 0x6d, 0x20, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x20, 0x54, 0x4f, 0x44, 0x4f, 0x28, 0x6c, 0x69, 0x7a, 0x61, 0x6e, 0x29, 0x3a, 0x20, 0x46, 0x69,
    0x67, 0x75, 0x72, 0x65, 0x20, 0x6f, 0x75, 0x74, 0x20, 0x68, 0x6f, 0x77, 0x20, 0x54, 0x79, 0x70,
    0x65, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62,
    0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x44, 0x50, 0x4c, 0x42,
    0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x64, 0x6f, 0x65, 0x73, 0x6e, 0x27, 0x74, 0x20, 0x6c, 0x69, 0x6e, 0x6b, 0x0a, 0x20, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x70, 0x74,
    0x6f, 0x72, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x44, 0x50, 0x4c, 0x42, 0x20, 0x69, 0x74, 0x73,
    0x65, 0x6c, 0x66, 0x2c, 0x20, 0x28, 0x65, 0x2e, 0x67, 0x2e, 0x20, 0x67, 0x52, 0x50, 0x43, 0x20,
    0x4c, 0x42, 0x20, 0x50, 0x6c, 0x75, 0x67, 0x69, 0x6e, 0x2c, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79,
    0x20, 0x57, 0x41, 0x53, 0x4d, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e, 0x73,
    0x29, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x25, 0x08, 0x13, 0x0a,
    0xfe, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x29, 0x02, 0x16, 0x1a, 0xf0, 0x01,
    0x20, 0x41, 0x20, 0x55, 0x52, 0x4c, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x75, 0x6e, 0x69, 0x71,
    0x75, 0x65, 0x6c, 0x79, 0x20, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x7a, 0x65, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x63,
    0x6f, 0x6c, 0x20, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x68, 0x61, 0x73, 0x20, 0x73, 0x61, 0x6d,
    0x65, 0x20, 0x73, 0x65, 0x6d, 0x61, 0x6e, 0x74, 0x69, 0x63, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69, 0x62, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x41, 0x6e, 0x79, 0x3a, 0x0a, 0x20, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a,
    0x2f, 0x2f, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x63, 0x6f, 0x6c, 0x62, 0x75, 0x66, 0x66, 0x65, 0x72, 0x73, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x62, 0x6c, 0x6f, 0x62, 0x2f, 0x6d, 0x61, 0x73, 0x74, 0x65,
    0x72, 0x2f, 0x73, 0x72, 0x63, 0x2f, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x29, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x29, 0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x29, 0x14, 0x15, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x2c, 0x02, 0x23, 0x1a, 0x34, 0x20, 0x41, 0x20, 0x4a, 0x53, 0x4f, 0x4e,
    0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20,
    0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x62, 0x6f, 0x76, 0x65, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x2c, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x19, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x2c, 0x21, 0x22, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)