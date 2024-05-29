// @generated
pub mod udpa {
    #[cfg(feature = "udpa-annotations")]
    // @@protoc_insertion_point(attribute:udpa.annotations)
    pub mod annotations {
        include!("udpa.annotations.rs");
        // @@protoc_insertion_point(udpa.annotations)
    }
    pub mod data {
        pub mod orca {
            #[cfg(feature = "udpa-data-orca-v1")]
            // @@protoc_insertion_point(attribute:udpa.data.orca.v1)
            pub mod v1 {
                include!("udpa.data.orca.v1.rs");
                // @@protoc_insertion_point(udpa.data.orca.v1)
            }
        }
    }
    pub mod r#type {
        #[cfg(feature = "udpa-type-v1")]
        // @@protoc_insertion_point(attribute:udpa.type.v1)
        pub mod v1 {
            include!("udpa.type.v1.rs");
            // @@protoc_insertion_point(udpa.type.v1)
        }
    }
    pub mod service {
        pub mod orca {
            #[cfg(feature = "udpa-service-orca-v1")]
            // @@protoc_insertion_point(attribute:udpa.service.orca.v1)
            pub mod v1 {
                include!("udpa.service.orca.v1.rs");
                // @@protoc_insertion_point(udpa.service.orca.v1)
            }
        }
    }
}
pub mod xds {
    pub mod annotations {
        #[cfg(feature = "xds-annotations-v3")]
        // @@protoc_insertion_point(attribute:xds.annotations.v3)
        pub mod v3 {
            include!("xds.annotations.v3.rs");
            // @@protoc_insertion_point(xds.annotations.v3)
        }
    }
    pub mod core {
        #[cfg(feature = "xds-core-v3")]
        // @@protoc_insertion_point(attribute:xds.core.v3)
        pub mod v3 {
            include!("xds.core.v3.rs");
            // @@protoc_insertion_point(xds.core.v3)
        }
    }
    pub mod data {
        pub mod orca {
            #[cfg(feature = "xds-data-orca-v3")]
            // @@protoc_insertion_point(attribute:xds.data.orca.v3)
            pub mod v3 {
                include!("xds.data.orca.v3.rs");
                // @@protoc_insertion_point(xds.data.orca.v3)
            }
        }
    }
    pub mod service {
        pub mod orca {
            #[cfg(feature = "xds-service-orca-v3")]
            // @@protoc_insertion_point(attribute:xds.service.orca.v3)
            pub mod v3 {
                include!("xds.service.orca.v3.rs");
                // @@protoc_insertion_point(xds.service.orca.v3)
            }
        }
    }
}