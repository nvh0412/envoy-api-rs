// @generated
pub mod envoy {
    #[cfg(feature = "envoy-annotations")]
    // @@protoc_insertion_point(attribute:envoy.annotations)
    pub mod annotations {
        include!("envoy.annotations.rs");
        // @@protoc_insertion_point(envoy.annotations)
    }
    pub mod config {
        pub mod core {
            #[cfg(feature = "envoy-config-core-v3")]
            // @@protoc_insertion_point(attribute:envoy.config.core.v3)
            pub mod v3 {
                include!("envoy.config.core.v3.rs");
                // @@protoc_insertion_point(envoy.config.core.v3)
            }
        }
        pub mod endpoint {
            #[cfg(feature = "envoy-config-endpoint-v3")]
            // @@protoc_insertion_point(attribute:envoy.config.endpoint.v3)
            pub mod v3 {
                include!("envoy.config.endpoint.v3.rs");
                // @@protoc_insertion_point(envoy.config.endpoint.v3)
            }
        }
        pub mod health_checker {
            pub mod redis {
                #[cfg(feature = "envoy-config-health_checker-redis-v2")]
                // @@protoc_insertion_point(attribute:envoy.config.health_checker.redis.v2)
                pub mod v2 {
                    include!("envoy.config.health_checker.redis.v2.rs");
                    // @@protoc_insertion_point(envoy.config.health_checker.redis.v2)
                }
            }
        }
        pub mod overload {
            #[cfg(feature = "envoy-config-overload-v2alpha")]
            // @@protoc_insertion_point(attribute:envoy.config.overload.v2alpha)
            pub mod v2alpha {
                include!("envoy.config.overload.v2alpha.rs");
                // @@protoc_insertion_point(envoy.config.overload.v2alpha)
            }
            #[cfg(feature = "envoy-config-overload-v3")]
            // @@protoc_insertion_point(attribute:envoy.config.overload.v3)
            pub mod v3 {
                include!("envoy.config.overload.v3.rs");
                // @@protoc_insertion_point(envoy.config.overload.v3)
            }
        }
        pub mod resource_monitor {
            pub mod fixed_heap {
                #[cfg(feature = "envoy-config-resource_monitor-fixed_heap-v2alpha")]
                // @@protoc_insertion_point(attribute:envoy.config.resource_monitor.fixed_heap.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.resource_monitor.fixed_heap.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.resource_monitor.fixed_heap.v2alpha)
                }
            }
            pub mod injected_resource {
                #[cfg(feature = "envoy-config-resource_monitor-injected_resource-v2alpha")]
                // @@protoc_insertion_point(attribute:envoy.config.resource_monitor.injected_resource.v2alpha)
                pub mod v2alpha {
                    include!("envoy.config.resource_monitor.injected_resource.v2alpha.rs");
                    // @@protoc_insertion_point(envoy.config.resource_monitor.injected_resource.v2alpha)
                }
            }
        }
        pub mod route {
            #[cfg(feature = "envoy-config-route-v3")]
            // @@protoc_insertion_point(attribute:envoy.config.route.v3)
            pub mod v3 {
                include!("envoy.config.route.v3.rs");
                // @@protoc_insertion_point(envoy.config.route.v3)
            }
        }
        pub mod upstream {
            pub mod local_address_selector {
                #[cfg(feature = "envoy-config-upstream-local_address_selector-v3")]
                // @@protoc_insertion_point(attribute:envoy.config.upstream.local_address_selector.v3)
                pub mod v3 {
                    include!("envoy.config.upstream.local_address_selector.v3.rs");
                    // @@protoc_insertion_point(envoy.config.upstream.local_address_selector.v3)
                }
            }
        }
    }
    #[cfg(feature = "envoy-type")]
    // @@protoc_insertion_point(attribute:envoy.type)
    pub mod r#type {
        include!("envoy.type.rs");
        // @@protoc_insertion_point(envoy.type)
        pub mod http {
            #[cfg(feature = "envoy-type-http-v3")]
            // @@protoc_insertion_point(attribute:envoy.type.http.v3)
            pub mod v3 {
                include!("envoy.type.http.v3.rs");
                // @@protoc_insertion_point(envoy.type.http.v3)
            }
        }
        #[cfg(feature = "envoy-type-matcher")]
        // @@protoc_insertion_point(attribute:envoy.type.matcher)
        pub mod matcher {
            include!("envoy.type.matcher.rs");
            // @@protoc_insertion_point(envoy.type.matcher)
            #[cfg(feature = "envoy-type-matcher-v3")]
            // @@protoc_insertion_point(attribute:envoy.type.matcher.v3)
            pub mod v3 {
                include!("envoy.type.matcher.v3.rs");
                // @@protoc_insertion_point(envoy.type.matcher.v3)
            }
        }
        pub mod metadata {
            #[cfg(feature = "envoy-type-metadata-v2")]
            // @@protoc_insertion_point(attribute:envoy.type.metadata.v2)
            pub mod v2 {
                include!("envoy.type.metadata.v2.rs");
                // @@protoc_insertion_point(envoy.type.metadata.v2)
            }
            #[cfg(feature = "envoy-type-metadata-v3")]
            // @@protoc_insertion_point(attribute:envoy.type.metadata.v3)
            pub mod v3 {
                include!("envoy.type.metadata.v3.rs");
                // @@protoc_insertion_point(envoy.type.metadata.v3)
            }
        }
        pub mod tracing {
            #[cfg(feature = "envoy-type-tracing-v2")]
            // @@protoc_insertion_point(attribute:envoy.type.tracing.v2)
            pub mod v2 {
                include!("envoy.type.tracing.v2.rs");
                // @@protoc_insertion_point(envoy.type.tracing.v2)
            }
            #[cfg(feature = "envoy-type-tracing-v3")]
            // @@protoc_insertion_point(attribute:envoy.type.tracing.v3)
            pub mod v3 {
                include!("envoy.type.tracing.v3.rs");
                // @@protoc_insertion_point(envoy.type.tracing.v3)
            }
        }
        #[cfg(feature = "envoy-type-v3")]
        // @@protoc_insertion_point(attribute:envoy.type.v3)
        pub mod v3 {
            include!("envoy.type.v3.rs");
            // @@protoc_insertion_point(envoy.type.v3)
        }
    }
    pub mod service {
        pub mod auth {
            #[cfg(feature = "envoy-service-auth-v3")]
            // @@protoc_insertion_point(attribute:envoy.service.auth.v3)
            pub mod v3 {
                include!("envoy.service.auth.v3.rs");
                // @@protoc_insertion_point(envoy.service.auth.v3)
            }
        }
        pub mod rate_limit_quota {
            #[cfg(feature = "envoy-service-rate_limit_quota-v3")]
            // @@protoc_insertion_point(attribute:envoy.service.rate_limit_quota.v3)
            pub mod v3 {
                include!("envoy.service.rate_limit_quota.v3.rs");
                // @@protoc_insertion_point(envoy.service.rate_limit_quota.v3)
            }
        }
    }
    pub mod watchdog {
        #[cfg(feature = "envoy-watchdog-v3")]
        // @@protoc_insertion_point(attribute:envoy.watchdog.v3)
        pub mod v3 {
            include!("envoy.watchdog.v3.rs");
            // @@protoc_insertion_point(envoy.watchdog.v3)
        }
    }
}
pub mod testdata {
    #[cfg(feature = "testdata-dynamic_descriptors")]
    // @@protoc_insertion_point(attribute:testdata.dynamic_descriptors)
    pub mod dynamic_descriptors {
        include!("testdata.dynamic_descriptors.rs");
        // @@protoc_insertion_point(testdata.dynamic_descriptors)
    }
}