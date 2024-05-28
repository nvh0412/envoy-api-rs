// @generated
impl serde::Serialize for Foo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bar.is_some() {
            len += 1;
        }
        if self.baz.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("testdata.dynamic_descriptors.Foo", len)?;
        if let Some(v) = self.bar.as_ref() {
            struct_ser.serialize_field("bar", v)?;
        }
        if let Some(v) = self.baz.as_ref() {
            struct_ser.serialize_field("baz", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Foo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bar",
            "baz",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bar,
            Baz,
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
                            "bar" => Ok(GeneratedField::Bar),
                            "baz" => Ok(GeneratedField::Baz),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Foo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct testdata.dynamic_descriptors.Foo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<Foo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bar__ = None;
                let mut baz__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bar => {
                            if bar__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bar"));
                            }
                            bar__ = map_.next_value()?;
                        }
                        GeneratedField::Baz => {
                            if baz__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baz"));
                            }
                            baz__ = map_.next_value()?;
                        }
                    }
                }
                Ok(Foo {
                    bar: bar__,
                    baz: baz__,
                })
            }
        }
        deserializer.deserialize_struct("testdata.dynamic_descriptors.Foo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for FooCopy {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.bar.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("testdata.dynamic_descriptors.FooCopy", len)?;
        if let Some(v) = self.bar.as_ref() {
            struct_ser.serialize_field("bar", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FooCopy {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "bar",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Bar,
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
                            "bar" => Ok(GeneratedField::Bar),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = FooCopy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct testdata.dynamic_descriptors.FooCopy")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<FooCopy, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut bar__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Bar => {
                            if bar__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bar"));
                            }
                            bar__ = map_.next_value()?;
                        }
                    }
                }
                Ok(FooCopy {
                    bar: bar__,
                })
            }
        }
        deserializer.deserialize_struct("testdata.dynamic_descriptors.FooCopy", FIELDS, GeneratedVisitor)
    }
}
