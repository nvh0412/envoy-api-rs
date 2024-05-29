// @generated
// This file is @generated by prost-build.
// \[#protodoc-title: Overload Manager\]

// The Overload Manager provides an extensible framework to protect Envoy instances
// from overload of various resources (memory, cpu, file descriptors, etc).
// It monitors a configurable set of resources and notifies registered listeners
// when triggers related to those resources fire.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceMonitor {
    /// The name of the resource monitor to instantiate. Must match a registered
    /// resource monitor type. The built-in resource monitors are:
    ///
    /// * :ref:`envoy.resource_monitors.fixed_heap
    ///    <envoy_api_msg_config.resource_monitor.fixed_heap.v2alpha.FixedHeapConfig>`
    /// * :ref:`envoy.resource_monitors.injected_resource
    ///    <envoy_api_msg_config.resource_monitor.injected_resource.v2alpha.InjectedResourceConfig>`
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Configuration for the resource monitor being instantiated.
    #[prost(oneof="resource_monitor::ConfigType", tags="2, 3")]
    pub config_type: ::core::option::Option<resource_monitor::ConfigType>,
}
/// Nested message and enum types in `ResourceMonitor`.
pub mod resource_monitor {
    /// Configuration for the resource monitor being instantiated.
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConfigType {
        #[prost(message, tag="2")]
        Config(::prost_types::Struct),
        #[prost(message, tag="3")]
        TypedConfig(::prost_types::Any),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ThresholdTrigger {
    /// If the resource pressure is greater than or equal to this value, the trigger
    /// will fire.
    #[prost(double, tag="1")]
    pub value: f64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Trigger {
    /// The name of the resource this is a trigger for.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(oneof="trigger::TriggerOneof", tags="2")]
    pub trigger_oneof: ::core::option::Option<trigger::TriggerOneof>,
}
/// Nested message and enum types in `Trigger`.
pub mod trigger {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum TriggerOneof {
        #[prost(message, tag="2")]
        Threshold(super::ThresholdTrigger),
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverloadAction {
    /// The name of the overload action. This is just a well-known string that listeners can
    /// use for registering callbacks. Custom overload actions should be named using reverse
    /// DNS to ensure uniqueness.
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// A set of triggers for this action. If any of these triggers fire the overload action
    /// is activated. Listeners are notified when the overload action transitions from
    /// inactivated to activated, or vice versa.
    #[prost(message, repeated, tag="2")]
    pub triggers: ::prost::alloc::vec::Vec<Trigger>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OverloadManager {
    /// The interval for refreshing resource usage.
    #[prost(message, optional, tag="1")]
    pub refresh_interval: ::core::option::Option<::prost_types::Duration>,
    /// The set of resources to monitor.
    #[prost(message, repeated, tag="2")]
    pub resource_monitors: ::prost::alloc::vec::Vec<ResourceMonitor>,
    /// The set of overload actions.
    #[prost(message, repeated, tag="3")]
    pub actions: ::prost::alloc::vec::Vec<OverloadAction>,
}
/// Encoded file descriptor set for the `envoy.config.overload.v2alpha` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0x96, 0x1f, 0x0a, 0x2c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2f, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x2f, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x12, 0x1d, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e,
    0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61,
    0x1a, 0x19, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2f, 0x61, 0x6e, 0x79, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x64, 0x75, 0x72,
    0x61, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1c, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x73, 0x74, 0x72,
    0x75, 0x63, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1d, 0x75, 0x64, 0x70, 0x61, 0x2f,
    0x61, 0x6e, 0x6e, 0x6f, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2f, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x17, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61,
    0x74, 0x65, 0x2f, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x61, 0x74, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xaf, 0x01, 0x0a, 0x0f, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d, 0x6f,
    0x6e, 0x69, 0x74, 0x6f, 0x72, 0x12, 0x1b, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52, 0x04, 0x6e, 0x61,
    0x6d, 0x65, 0x12, 0x35, 0x0a, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x17, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x62, 0x75, 0x66, 0x2e, 0x53, 0x74, 0x72, 0x75, 0x63, 0x74, 0x42, 0x02, 0x18, 0x01, 0x48,
    0x00, 0x52, 0x06, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x39, 0x0a, 0x0c, 0x74, 0x79, 0x70,
    0x65, 0x64, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x14, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75,
    0x66, 0x2e, 0x41, 0x6e, 0x79, 0x48, 0x00, 0x52, 0x0b, 0x74, 0x79, 0x70, 0x65, 0x64, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x42, 0x0d, 0x0a, 0x0b, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x22, 0x41, 0x0a, 0x10, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64,
    0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x12, 0x2d, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x01, 0x42, 0x17, 0xfa, 0x42, 0x14, 0x12, 0x12, 0x19, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0xf0, 0x3f, 0x29, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x52,
    0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x8d, 0x01, 0x0a, 0x07, 0x54, 0x72, 0x69, 0x67, 0x67,
    0x65, 0x72, 0x12, 0x1b, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12,
    0x4f, 0x0a, 0x09, 0x74, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x2f, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x2e, 0x54, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64, 0x54, 0x72, 0x69, 0x67,
    0x67, 0x65, 0x72, 0x48, 0x00, 0x52, 0x09, 0x74, 0x68, 0x72, 0x65, 0x73, 0x68, 0x6f, 0x6c, 0x64,
    0x42, 0x14, 0x0a, 0x0d, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x5f, 0x6f, 0x6e, 0x65, 0x6f,
    0x66, 0x12, 0x03, 0xf8, 0x42, 0x01, 0x22, 0x7b, 0x0a, 0x0e, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f,
    0x61, 0x64, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x07, 0xfa, 0x42, 0x04, 0x72, 0x02, 0x20, 0x01, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x4c, 0x0a, 0x08, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x26, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2e,
    0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x54, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x42,
    0x08, 0xfa, 0x42, 0x05, 0x92, 0x01, 0x02, 0x08, 0x01, 0x52, 0x08, 0x74, 0x72, 0x69, 0x67, 0x67,
    0x65, 0x72, 0x73, 0x22, 0x87, 0x02, 0x0a, 0x0f, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64,
    0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x12, 0x44, 0x0a, 0x10, 0x72, 0x65, 0x66, 0x72, 0x65,
    0x73, 0x68, 0x5f, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x19, 0x2e, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x62, 0x75, 0x66, 0x2e, 0x44, 0x75, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x0f, 0x72, 0x65,
    0x66, 0x72, 0x65, 0x73, 0x68, 0x49, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x12, 0x65, 0x0a,
    0x11, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f,
    0x72, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2e, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79,
    0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64,
    0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63,
    0x65, 0x4d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x42, 0x08, 0xfa, 0x42, 0x05, 0x92, 0x01, 0x02,
    0x08, 0x01, 0x52, 0x10, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x4d, 0x6f, 0x6e, 0x69,
    0x74, 0x6f, 0x72, 0x73, 0x12, 0x47, 0x0a, 0x07, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18,
    0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x2d, 0x2e, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x76, 0x32,
    0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x41, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x07, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x42, 0xa1, 0x02,
    0xba, 0x80, 0xc8, 0xd1, 0x06, 0x02, 0x10, 0x01, 0x0a, 0x21, 0x63, 0x6f, 0x6d, 0x2e, 0x65, 0x6e,
    0x76, 0x6f, 0x79, 0x2e, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x6f, 0x76, 0x65, 0x72, 0x6c,
    0x6f, 0x61, 0x64, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x42, 0x0d, 0x4f, 0x76, 0x65,
    0x72, 0x6c, 0x6f, 0x61, 0x64, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x4e, 0x67, 0x69,
    0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x54, 0x68, 0x69, 0x6e, 0x6b, 0x65, 0x69,
    0x2f, 0x67, 0x72, 0x70, 0x63, 0x5f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x5f, 0x67, 0x6f, 0x2f, 0x65,
    0x6e, 0x76, 0x6f, 0x79, 0x2f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2f, 0x6f, 0x76, 0x65, 0x72,
    0x6c, 0x6f, 0x61, 0x64, 0x2f, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x3b, 0x6f, 0x76, 0x65,
    0x72, 0x6c, 0x6f, 0x61, 0x64, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0xa2, 0x02, 0x03, 0x45,
    0x43, 0x4f, 0xaa, 0x02, 0x1d, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x2e, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x2e, 0x56, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0xca, 0x02, 0x1d, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x5c, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x5c, 0x56, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0xe2, 0x02, 0x29, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x5c, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x5c, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x5c, 0x56, 0x32, 0x61, 0x6c, 0x70,
    0x68, 0x61, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02,
    0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3a, 0x3a,
    0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x3a, 0x3a, 0x56, 0x32, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x4a, 0xfd, 0x14, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x50, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x26,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x23, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x05, 0x00, 0x28, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00,
    0x26, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x03, 0x12, 0x03, 0x08, 0x00, 0x27, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x09, 0x00, 0x21, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0c, 0x00,
    0x2e, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x08, 0x12, 0x03, 0x0c, 0x00, 0x2e, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0d, 0x00, 0x22, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0a, 0x12, 0x03, 0x0d, 0x00,
    0x22, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0x87, 0x80, 0x99, 0x6a, 0x02, 0x12, 0x03, 0x0f, 0x00, 0x46, 0x0a, 0xcf, 0x02, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x18, 0x00, 0x28, 0x01, 0x32, 0x25, 0x20, 0x5b, 0x23, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x64, 0x6f, 0x63, 0x2d, 0x74, 0x69, 0x74, 0x6c, 0x65, 0x3a, 0x20, 0x4f, 0x76, 0x65, 0x72,
    0x6c, 0x6f, 0x61, 0x64, 0x20, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x5d, 0x0a, 0x32, 0x9b,
    0x02, 0x20, 0x54, 0x68, 0x65, 0x20, 0x4f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x4d,
    0x61, 0x6e, 0x61, 0x67, 0x65, 0x72, 0x20, 0x70, 0x72, 0x6f, 0x76, 0x69, 0x64, 0x65, 0x73, 0x20,
    0x61, 0x6e, 0x20, 0x65, 0x78, 0x74, 0x65, 0x6e, 0x73, 0x69, 0x62, 0x6c, 0x65, 0x20, 0x66, 0x72,
    0x61, 0x6d, 0x65, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x72, 0x6f, 0x74, 0x65,
    0x63, 0x74, 0x20, 0x45, 0x6e, 0x76, 0x6f, 0x79, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x63,
    0x65, 0x73, 0x0a, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61,
    0x64, 0x20, 0x6f, 0x66, 0x20, 0x76, 0x61, 0x72, 0x69, 0x6f, 0x75, 0x73, 0x20, 0x72, 0x65, 0x73,
    0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x28, 0x6d, 0x65, 0x6d, 0x6f, 0x72, 0x79, 0x2c, 0x20,
    0x63, 0x70, 0x75, 0x2c, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x73, 0x63, 0x72, 0x69,
    0x70, 0x74, 0x6f, 0x72, 0x73, 0x2c, 0x20, 0x65, 0x74, 0x63, 0x29, 0x2e, 0x0a, 0x20, 0x49, 0x74,
    0x20, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x73, 0x20, 0x61, 0x20, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x75, 0x72, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20,
    0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6e, 0x6f,
    0x74, 0x69, 0x66, 0x69, 0x65, 0x73, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72, 0x65,
    0x64, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x73, 0x0a, 0x20, 0x77, 0x68, 0x65,
    0x6e, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x73, 0x20, 0x72, 0x65, 0x6c, 0x61, 0x74,
    0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x6f, 0x73, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f,
    0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x66, 0x69, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x18, 0x08, 0x17, 0x0a, 0xa0, 0x03, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x20, 0x02, 0x3d, 0x1a, 0x92, 0x03, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x61,
    0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x20, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e,
    0x73, 0x74, 0x61, 0x6e, 0x74, 0x69, 0x61, 0x74, 0x65, 0x2e, 0x20, 0x4d, 0x75, 0x73, 0x74, 0x20,
    0x6d, 0x61, 0x74, 0x63, 0x68, 0x20, 0x61, 0x20, 0x72, 0x65, 0x67, 0x69, 0x73, 0x74, 0x65, 0x72,
    0x65, 0x64, 0x0a, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x20, 0x6d, 0x6f, 0x6e,
    0x69, 0x74, 0x6f, 0x72, 0x20, 0x74, 0x79, 0x70, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x62,
    0x75, 0x69, 0x6c, 0x74, 0x2d, 0x69, 0x6e, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x20, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65, 0x3a, 0x0a, 0x0a,
    0x20, 0x2a, 0x20, 0x3a, 0x72, 0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x72,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x73,
    0x2e, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70, 0x0a, 0x20, 0x20, 0x20, 0x3c,
    0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d, 0x73, 0x67, 0x5f, 0x63, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f, 0x6d, 0x6f,
    0x6e, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x66, 0x69, 0x78, 0x65, 0x64, 0x5f, 0x68, 0x65, 0x61, 0x70,
    0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68, 0x61, 0x2e, 0x46, 0x69, 0x78, 0x65, 0x64, 0x48, 0x65,
    0x61, 0x70, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3e, 0x60, 0x0a, 0x20, 0x2a, 0x20, 0x3a, 0x72,
    0x65, 0x66, 0x3a, 0x60, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x5f, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x73, 0x2e, 0x69, 0x6e, 0x6a, 0x65,
    0x63, 0x74, 0x65, 0x64, 0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x0a, 0x20, 0x20,
    0x20, 0x3c, 0x65, 0x6e, 0x76, 0x6f, 0x79, 0x5f, 0x61, 0x70, 0x69, 0x5f, 0x6d, 0x73, 0x67, 0x5f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x2e, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x5f,
    0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x2e, 0x69, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64,
    0x5f, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x2e, 0x76, 0x32, 0x61, 0x6c, 0x70, 0x68,
    0x61, 0x2e, 0x49, 0x6e, 0x6a, 0x65, 0x63, 0x74, 0x65, 0x64, 0x52, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x3e, 0x60, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x20, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x20, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x20, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x20, 0x12, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12,
    0x03, 0x20, 0x13, 0x3b, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x23, 0x02,
    0x27, 0x03, 0x1a, 0x3c, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x75, 0x72, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x20, 0x6d, 0x6f, 0x6e, 0x69, 0x74, 0x6f, 0x72, 0x20, 0x62, 0x65, 0x69, 0x6e,
    0x67, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x61, 0x6e, 0x74, 0x69, 0x61, 0x74, 0x65, 0x64, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x23, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x24, 0x04, 0x3a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x24, 0x04, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x24, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x24, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x24, 0x26, 0x39, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x02, 0x01, 0x08, 0x03, 0x12, 0x03, 0x24,
    0x27, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x26, 0x04, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x26, 0x04, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x26, 0x18, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x26, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12,
    0x04, 0x2a, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x2a, 0x08,
    0x18, 0x0a, 0x67, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x2d, 0x02, 0x43, 0x1a, 0x5a,
    0x20, 0x49, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65,
    0x20, 0x70, 0x72, 0x65, 0x73, 0x73, 0x75, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x67, 0x72, 0x65,
    0x61, 0x74, 0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x6e, 0x20, 0x6f, 0x72, 0x20, 0x65, 0x71, 0x75,
    0x61, 0x6c, 0x20, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x2c, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x0a, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x66, 0x69, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x2d, 0x09, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x2d, 0x11, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2d,
    0x13, 0x42, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x02, 0x12, 0x03,
    0x2d, 0x14, 0x41, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x30, 0x00, 0x39, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x30, 0x08, 0x0f, 0x0a, 0x3e, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x32, 0x02, 0x3d, 0x1a, 0x31, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e,
    0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75,
    0x72, 0x63, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x61, 0x20, 0x74, 0x72,
    0x69, 0x67, 0x67, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x32, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x32, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x32, 0x10, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03,
    0x32, 0x12, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12,
    0x03, 0x32, 0x13, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x02, 0x08, 0x00, 0x12, 0x04, 0x34, 0x02,
    0x38, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x08, 0x00, 0x01, 0x12, 0x03, 0x34, 0x08, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x08, 0x00, 0x02, 0x12, 0x03, 0x35, 0x04, 0x26, 0x0a, 0x0e,
    0x0a, 0x07, 0x04, 0x02, 0x08, 0x00, 0x02, 0xaf, 0x08, 0x12, 0x03, 0x35, 0x04, 0x26, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x37, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x37, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x37, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x3b, 0x00, 0x45,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x16, 0x0a, 0xd5, 0x01,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x02, 0x3d, 0x1a, 0xc7, 0x01, 0x20, 0x54,
    0x68, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f,
    0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x20,
    0x54, 0x68, 0x69, 0x73, 0x20, 0x69, 0x73, 0x20, 0x6a, 0x75, 0x73, 0x74, 0x20, 0x61, 0x20, 0x77,
    0x65, 0x6c, 0x6c, 0x2d, 0x6b, 0x6e, 0x6f, 0x77, 0x6e, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6e, 0x67,
    0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x73, 0x20,
    0x63, 0x61, 0x6e, 0x0a, 0x20, 0x75, 0x73, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x67,
    0x69, 0x73, 0x74, 0x65, 0x72, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x62, 0x61, 0x63,
    0x6b, 0x73, 0x2e, 0x20, 0x43, 0x75, 0x73, 0x74, 0x6f, 0x6d, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x6c,
    0x6f, 0x61, 0x64, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x62, 0x65, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x64, 0x20, 0x75, 0x73, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x65, 0x76, 0x65, 0x72, 0x73, 0x65, 0x0a, 0x20, 0x44, 0x4e, 0x53, 0x20, 0x74,
    0x6f, 0x20, 0x65, 0x6e, 0x73, 0x75, 0x72, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x6e,
    0x65, 0x73, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x3f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x09,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3f, 0x10, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12, 0x03, 0x3f, 0x12, 0x3c, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xaf, 0x08, 0x0e, 0x12, 0x03, 0x3f, 0x13, 0x3b, 0x0a, 0xde,
    0x01, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x44, 0x02, 0x4d, 0x1a, 0xd0, 0x01, 0x20,
    0x41, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72,
    0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x2e, 0x20, 0x49, 0x66, 0x20, 0x61, 0x6e, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65,
    0x73, 0x65, 0x20, 0x74, 0x72, 0x69, 0x67, 0x67, 0x65, 0x72, 0x73, 0x20, 0x66, 0x69, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x20, 0x69, 0x73, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74,
    0x65, 0x64, 0x2e, 0x20, 0x4c, 0x69, 0x73, 0x74, 0x65, 0x6e, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72,
    0x65, 0x20, 0x6e, 0x6f, 0x74, 0x69, 0x66, 0x69, 0x65, 0x64, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x20, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x69, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20,
    0x66, 0x72, 0x6f, 0x6d, 0x0a, 0x20, 0x69, 0x6e, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x61, 0x63, 0x74, 0x69, 0x76, 0x61, 0x74, 0x65, 0x64, 0x2c, 0x20,
    0x6f, 0x72, 0x20, 0x76, 0x69, 0x63, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x61, 0x2e, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x44, 0x02, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x44, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x44, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x08,
    0x12, 0x03, 0x44, 0x20, 0x4c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x01, 0x08, 0xaf, 0x08,
    0x12, 0x12, 0x03, 0x44, 0x21, 0x4b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x47, 0x00,
    0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x47, 0x08, 0x17, 0x0a, 0x3a,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x49, 0x02, 0x30, 0x1a, 0x2d, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x76, 0x61, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72,
    0x65, 0x66, 0x72, 0x65, 0x73, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x73, 0x6f, 0x75, 0x72,
    0x63, 0x65, 0x20, 0x75, 0x73, 0x61, 0x67, 0x65, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x49, 0x02, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x49, 0x1b, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x49, 0x2e, 0x2f, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x02,
    0x5e, 0x1a, 0x22, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x72,
    0x65, 0x73, 0x6f, 0x75, 0x72, 0x63, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x6d, 0x6f, 0x6e, 0x69,
    0x74, 0x6f, 0x72, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x4c, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x4c, 0x0b,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4c, 0x1b, 0x2c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4c, 0x2f, 0x30, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x01, 0x08, 0x12, 0x03, 0x4c, 0x31, 0x5d, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x04, 0x02, 0x01, 0x08, 0xaf, 0x08, 0x12, 0x12, 0x03, 0x4c, 0x32, 0x5c, 0x0a, 0x2b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x02, 0x26, 0x1a, 0x1e, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x73, 0x65, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x6f, 0x76, 0x65, 0x72, 0x6c, 0x6f, 0x61, 0x64, 0x20,
    0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x4f, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x4f, 0x0b, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x4f, 0x1a, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4f, 0x24,
    0x25, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)