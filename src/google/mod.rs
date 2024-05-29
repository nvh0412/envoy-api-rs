// @generated
pub mod google {
    #[cfg(feature = "google-rpc")]
    // @@protoc_insertion_point(attribute:google.rpc)
    pub mod rpc {
        include!("google.rpc.rs");
        // @@protoc_insertion_point(google.rpc)
        #[cfg(feature = "google-rpc-context")]
        // @@protoc_insertion_point(attribute:google.rpc.context)
        pub mod context {
            include!("google.rpc.context.rs");
            // @@protoc_insertion_point(google.rpc.context)
        }
    }
}