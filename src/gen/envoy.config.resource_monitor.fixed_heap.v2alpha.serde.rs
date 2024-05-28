// @generated
impl serde::Serialize for FixedHeapConfig {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_heap_size_bytes != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("envoy.config.resource_monitor.fixed_heap.v2alpha.FixedHeapConfig", len)?;
        if self.max_heap_size_bytes != 0 {
            #[allow(clippy::needless_borrow)]
            struct_ser.serialize_field("maxHeapSizeBytes", ToString::to_string(&self.max_heap_size_bytes).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FixedHeapConfig {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_heap_size_bytes",
            "maxHeapSizeBytes",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxHeapSizeBytes,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "maxHeapSizeBytes" | "max_heap_size_bytes" => Ok(GeneratedField::MaxHeapSizeBytes),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FixedHeapConfig;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct envoy.config.resource_monitor.fixed_heap.v2alpha.FixedHeapConfig")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FixedHeapConfig, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_heap_size_bytes__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::MaxHeapSizeBytes => {
                            if max_heap_size_bytes__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxHeapSizeBytes"));
                            }
                            max_heap_size_bytes__ = 
                                Some(map_.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(FixedHeapConfig {
                    max_heap_size_bytes: max_heap_size_bytes__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("envoy.config.resource_monitor.fixed_heap.v2alpha.FixedHeapConfig", FIELDS, GeneratedVisitor)
    }
}
