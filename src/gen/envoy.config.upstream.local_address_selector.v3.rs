// @generated
// This file is @generated by prost-build.
// \[#protodoc-title: Default Local Address Selector\]
// \[#extension: envoy.upstream.local_address_selector.default_local_address_selector\]

/// Default implementation of a local address selector. This implementation is
/// used if :ref:`local_address_selector
/// <envoy_v3_api_field_config.core.v3.BindConfig.local_address_selector>` is not
/// specified.
/// This implementation supports the specification of only one address in
/// :ref:`extra_source_addresses
/// <envoy_v3_api_field_config.core.v3.BindConfig.extra_source_addresses>` which
/// is appended to the address specified in the
/// :ref:`source_address <envoy_v3_api_field_config.core.v3.BindConfig.source_address>`
/// field. The extra address should have a different IP version than the address in the
/// ``source_address`` field. The address which has the same IP
/// version with the target host's address IP version will be used as bind address.
/// If there is no same IP version address found, the address in the ``source_address`` field will
/// be returned.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DefaultLocalAddressSelector {
}
/// Encoded file descriptor set for the `envoy.config.upstream.local_address_selector.v3` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd9, 0x0d, 0x0a, 0x54, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c,
    0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f,
    0x72, 0x2f, 0x76, 0x33, 0x2f, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f, 0x6c, 0x6f, 0x63,
    0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63,
    0x74, 0x6f, 0x72, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x2f, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61,
    0x2f, 0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x1d, 0x0a, 0x1b, 0x44, 0x65, 0x66,
    0x61, 0x75, 0x6c, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x42, 0xa3, 0x03, 0xba, 0x80, 0xc8, 0xd1, 0x06,
    0x02, 0x10, 0x02, 0x0a, 0x33, 0x63, 0x6f, 0x6d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x76, 0x33, 0x42, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c,
    0x74, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x53, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x69, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x54, 0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69,
    0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x75, 0x70, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x2f, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2f, 0x76, 0x33, 0x3b, 0x6c,
    0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c,
    0x65, 0x63, 0x74, 0x6f, 0x72, 0x76, 0x33, 0xa2, 0x02, 0x04, 0x45, 0x43, 0x55, 0x4c, 0xaa, 0x02,
    0x2d, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x55, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x2e, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x56, 0x33, 0xca, 0x02,
    0x2d, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x55, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5c, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x5c, 0x56, 0x33, 0xe2, 0x02,
    0x39, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5c, 0x55, 0x70,
    0x73, 0x74, 0x72, 0x65, 0x61, 0x6d, 0x5c, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x5c, 0x56, 0x33, 0x5c, 0x47,
    0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x31, 0x45, 0x6e, 0x76,
    0x6f, 0x79, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x3a, 0x55, 0x70, 0x73, 0x74,
    0x72, 0x65, 0x61, 0x6d, 0x3a, 0x3a, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x41, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x3a, 0x3a, 0x56, 0x33, 0x4a, 0xe3,
    0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1e, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03,
    0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x38, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07,
    0x00, 0x41, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x07, 0x00, 0x41, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x08, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x08,
    0x00, 0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06,
    0x08, 0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0a, 0x00, 0x46, 0x0a, 0xea, 0x07, 0x0a, 0x02,
    0x04, 0x00, 0x12, 0x04, 0x1d, 0x00, 0x1e, 0x01, 0x1a, 0xd3, 0x06, 0x20, 0x44, 0x65, 0x66, 0x61,
    0x75, 0x6c, 0x74, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x20, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x20, 0x61, 0x64,
    0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x0a, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x66, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x0a, 0x20, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33,
    0x2e, 0x42, 0x69, 0x6e, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x6c, 0x6f, 0x63, 0x61,
    0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x3e, 0x60, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74, 0x0a, 0x20, 0x73, 0x70, 0x65,
    0x63, 0x69, 0x66, 0x69, 0x65, 0x64, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x6d,
    0x70, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x75, 0x70,
    0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66,
    0x69, 0x63, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20,
    0x6f, 0x6e, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x0a, 0x20,
    0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x78, 0x74, 0x72, 0x61, 0x5f, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x65, 0x73, 0x0a, 0x20, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33,
    0x2e, 0x42, 0x69, 0x6e, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x65, 0x78, 0x74, 0x72,
    0x61, 0x5f, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x65, 0x73, 0x3e, 0x60, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x61,
    0x70, 0x70, 0x65, 0x6e, 0x64, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x65, 0x64,
    0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x3c, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x76, 0x33, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x66, 0x69, 0x65, 0x6c,
    0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x63, 0x6f, 0x72, 0x65, 0x2e, 0x76, 0x33,
    0x2e, 0x42, 0x69, 0x6e, 0x64, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x3e, 0x60, 0x0a, 0x20, 0x66, 0x69,
    0x65, 0x6c, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x65, 0x78, 0x74, 0x72, 0x61, 0x20, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x68, 0x61,
    0x76, 0x65, 0x20, 0x61, 0x20, 0x64, 0x69, 0x66, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x49,
    0x50, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x60, 0x60, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72,
    0x65, 0x73, 0x73, 0x60, 0x60, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x2e, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x77, 0x68, 0x69, 0x63, 0x68, 0x20, 0x68,
    0x61, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x61, 0x6d, 0x65, 0x20, 0x49, 0x50, 0x0a, 0x20,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x68, 0x6f, 0x73, 0x74, 0x27, 0x73, 0x20, 0x61,
    0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x49, 0x50, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x61,
    0x73, 0x20, 0x62, 0x69, 0x6e, 0x64, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x2e, 0x0a,
    0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x20,
    0x73, 0x61, 0x6d, 0x65, 0x20, 0x49, 0x50, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20,
    0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x2c, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x60, 0x60, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65,
    0x73, 0x73, 0x60, 0x60, 0x20, 0x66, 0x69, 0x65, 0x6c, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x0a,
    0x20, 0x62, 0x65, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x65, 0x64, 0x2e, 0x0a, 0x32, 0x87,
    0x01, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74,
    0x6c, 0x65, 0x3a, 0x20, 0x44, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x20, 0x4c, 0x6f, 0x63, 0x61,
    0x6c, 0x20, 0x41, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x20, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74,
    0x6f, 0x72, 0x5d, 0x0a, 0x20, 0x5b, 0x23, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x6f, 0x6e,
    0x3a, 0x20, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x75, 0x70, 0x73, 0x74, 0x72, 0x65, 0x61, 0x6d,
    0x2e, 0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73,
    0x65, 0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x2e, 0x64, 0x65, 0x66, 0x61, 0x75, 0x6c, 0x74, 0x5f,
    0x6c, 0x6f, 0x63, 0x61, 0x6c, 0x5f, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73, 0x5f, 0x73, 0x65,
    0x6c, 0x65, 0x63, 0x74, 0x6f, 0x72, 0x5d, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x1d, 0x08, 0x23, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("envoy.config.upstream.local_address_selector.v3.serde.rs");
// @@protoc_insertion_point(module)