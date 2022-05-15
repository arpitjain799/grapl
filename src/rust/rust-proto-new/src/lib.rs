use std::time::SystemTimeError;

use bytes::{
    Buf,
    Bytes,
};
use prost::{
    DecodeError,
    EncodeError,
};
use thiserror::Error;

pub mod protocol {
    pub mod healthcheck;
    pub mod status;
    pub mod tls;
}

pub(crate) mod server_internals;

// This module hierarchy contains all the stubs generated by the protocol buffer
// compiler. They are not to be exported by this crate's public API, instead
// they will remain internal to this crate.
pub(crate) mod protobufs {
    pub(crate) mod graplinc {
        pub(crate) mod common {
            pub(crate) mod v1beta1 {
                include!(concat!(env!("OUT_DIR"), "/graplinc.common.v1beta1.rs"));
            }
        }

        pub(crate) mod grapl {
            pub(crate) mod api {
                pub(crate) mod graph {
                    pub(crate) mod v1beta1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/graplinc.grapl.api.graph.v1beta1.rs"
                        ));
                    }
                }

                // #[cfg(feature = "graph-mutation")]
                pub(crate) mod graph_mutation {
                    pub(crate) mod v1beta1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/graplinc.grapl.api.graph_mutation.v1beta1.rs"
                        ));
                    }
                }

                pub(crate) mod pipeline_ingress {
                    pub(crate) mod v1beta1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/graplinc.grapl.api.pipeline_ingress.v1beta1.rs"
                        ));
                    }
                }

                pub(crate) mod plugin_bootstrap {
                    pub(crate) mod v1beta1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/graplinc.grapl.api.plugin_bootstrap.v1beta1.rs"
                        ));
                    }
                }

                pub(crate) mod plugin_registry {
                    pub(crate) mod v1beta1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/graplinc.grapl.api.plugin_registry.v1beta1.rs"
                        ));
                    }
                }

                pub(crate) mod plugin_sdk {
                    pub(crate) mod generators {
                        pub(crate) mod v1beta1 {
                            include!(concat!(
                                env!("OUT_DIR"),
                                "/graplinc.grapl.api.plugin_sdk.generators.v1beta1.rs"
                            ));
                        }
                    }
                }

                pub(crate) mod plugin_work_queue {
                    pub(crate) mod v1beta1 {
                        include!(concat!(
                            env!("OUT_DIR"),
                            "/graplinc.grapl.api.plugin_work_queue.v1beta1.rs"
                        ));
                    }
                }
            }

            pub(crate) mod pipeline {
                pub(crate) mod v1beta1 {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/graplinc.grapl.pipeline.v1beta1.rs"
                    ));
                }

                pub(crate) mod v1beta2 {
                    include!(concat!(
                        env!("OUT_DIR"),
                        "/graplinc.grapl.pipeline.v1beta2.rs"
                    ));
                }
            }
        }
    }
}

// This module hierarchy closely emulates the one above, but contains our rust
// language representations of the wire format. This module hierarchy must be
// kept synchronized with the one above. This module hierarchy defines this
// crate's public API.
pub mod graplinc {
    pub mod common {
        pub mod v1beta1;
    }

    pub mod grapl {
        pub mod api {
            pub mod graph {
                pub mod v1beta1;
            }

            // #[cfg(feature = "graph-mutation")]
            pub mod graph_mutation {
                pub mod v1beta1;
            }

            pub mod model_plugin_deployer {
                pub mod v1;
            }

            pub mod pipeline_ingress {
                pub mod v1beta1;
            }

            pub mod plugin_bootstrap {
                pub mod v1beta1;
            }

            pub mod plugin_registry {
                pub mod v1beta1;
                mod v1beta1_client;
                mod v1beta1_server;
            }

            pub mod plugin_sdk {
                pub mod generators {
                    pub mod v1beta1;
                }
            }

            pub mod plugin_work_queue {
                pub mod v1beta1;
                mod v1beta1_client;
                mod v1beta1_server;
            }
        }

        pub mod metrics {
            pub mod v1;
        }

        pub mod pipeline {
            pub mod v1beta1;
            pub mod v1beta2;
        }
    }
}

// This one's a little funky. See https://stackoverflow.com/a/53207767. The idea
// here is that we _do not want_ to expose the TYPE_URL as part of this crate's
// public API. However, in order to serialize a google.protobuf.Any message we
// require the TYPE_URL. Since prost does not support protocol buffer reflection
// features--descriptors, crucially--we are unable to determine the TYPE_URL for
// an arbitrary protocol buffer type at runtime. Therefore we resort to
// hard-coding them.
//
// The critical invariant here is that the TYPE_URL must be identically the same
// for any protocol buffer as that generated by the python implementation of
// google.protobuf.any_pb2.Any#Pack as used in python-proto. We maintain this
// invariant with round-trip regression tests which ensure every message can
// pass unmolested through all of our serialization and deserialization code.
//
// Should we find the need to work with protocol buffer stubs in a third
// language we'll add those stubs to the round-trip tests as well.
pub(crate) mod type_url {
    pub trait TypeUrl {
        const TYPE_URL: &'static str;
    }
}

#[non_exhaustive]
#[derive(Error, Debug)]
pub enum SerDeError {
    #[error("failed to serialize {0}")]
    EncodingFailed(#[from] EncodeError),

    #[error("failed to deserialize {0}")]
    DecodingFailed(#[from] DecodeError),

    #[error("bad timestamp {0}")]
    BadTimestamp(#[from] SystemTimeError),

    #[error("missing message field {0}")]
    MissingField(&'static str),

    #[error("field {field_name} failed assertion {assertion}")]
    InvalidField {
        field_name: &'static str,
        assertion: String,
    },

    #[error("Unknown enum variant {0}")]
    UnknownVariant(&'static str),
}

impl From<std::convert::Infallible> for SerDeError {
    fn from(_: std::convert::Infallible) -> Self {
        unimplemented!()
    }
}

pub trait SerDe: type_url::TypeUrl + Clone + std::fmt::Debug {
    fn serialize(self) -> Result<Bytes, SerDeError>;

    fn deserialize<B>(buf: B) -> Result<Self, SerDeError>
    where
        B: Buf,
        Self: Sized;
}

pub(crate) mod serde_impl {
    use bytes::{
        Bytes,
        BytesMut,
    };
    use prost::Message;

    use crate::{
        type_url,
        SerDe,
        SerDeError,
    };

    /// You can use this blanket implementation to automatically implement SerDe
    /// for a type YourType defined within this crate. YourType must implement
    /// either From<proto::YourType> or TryFrom<proto::YourType> and you must
    /// implement either From<YourType> or TryFrom<YourType> for
    /// proto::YourType. YourType must also implement type_url::TypeUrl. If
    /// YourType meets these conditions you can automatically implement SerDe by
    /// implementing the simpler trait ProtobufSerializable.
    ///
    /// Example usage:
    ///
    /// use crate::{
    ///     protobufs::graplinc::v1beta1 as proto,
    ///     serde_impl,
    ///     type_url,
    /// };
    ///
    /// pub struct YourType {...}
    ///
    /// impl type_url::TypeUrl for YourType {
    ///     const TYPE_URL: &'static str =
    ///         "graplsecurity.com/graplinc.v1beta1.YourType";
    /// }
    ///
    /// impl serde_impl::ProtobufSerializable for YourType {
    ///    type ProtobufMessage = proto::YourType;
    /// }
    pub(crate) trait ProtobufSerializable: Sized {
        type ProtobufMessage: TryFrom<Self> + TryInto<Self> + Message + Default;
    }

    impl<T> SerDe for T
    where
        T: ProtobufSerializable,
        T: type_url::TypeUrl + Clone + std::fmt::Debug,
        SerDeError: From<<<T as ProtobufSerializable>::ProtobufMessage as TryFrom<T>>::Error>,
        SerDeError: From<<<T as ProtobufSerializable>::ProtobufMessage as TryInto<T>>::Error>,
    {
        fn serialize(self: T) -> Result<Bytes, SerDeError> {
            let proto = T::ProtobufMessage::try_from(self)?;
            let mut buf = BytesMut::with_capacity(proto.encoded_len());
            proto.encode(&mut buf)?;
            Ok(buf.freeze())
        }

        fn deserialize<B>(buf: B) -> Result<T, SerDeError>
        where
            B: bytes::Buf,
            Self: Sized,
        {
            let proto: T::ProtobufMessage = Message::decode(buf)?;
            Ok(proto.try_into()?)
        }
    }
}
