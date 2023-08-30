// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data exporter for [`BlobDataProvider`](crate::BlobDataProvider).
//!
//! This module can be used as a target for the `icu_datagen` crate.
//!
//! # Examples
//!
//! ```
//! use icu_datagen::prelude::*;
//! use icu_provider_blob::export::*;
//!
//! let mut blob: Vec<u8> = Vec::new();
//!
//! // Set up the exporter
//! let mut exporter = BlobExporter::new_with_sink(Box::new(&mut blob));
//!
//! // Export something
//! DatagenDriver::new()
//!     .with_keys([icu_provider::hello_world::HelloWorldV1Marker::KEY])
//!     .export(&DatagenProvider::latest_tested(), exporter)
//!     .unwrap();
//!
//! // communicate the blob to the client application (network, disk, etc.)
//! ```
//!
//! The resulting blob can now be used like this:
//!
//! ```
//! use icu_locid::langid;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use icu_provider_blob::BlobDataProvider;
//!
//! // obtain the data blob
//! # let blob = std::fs::read(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/hello_world.postcard")).unwrap();
//!
//! // Create a provider reading from the blob
//! let provider =
//!     BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
//!         .expect("Should successfully read from blob");
//!
//! // Use the provider as a `BufferProvider`
//! let formatter = HelloWorldFormatter::try_new_with_buffer_provider(&provider, &langid!("en").into()).unwrap();
//!
//! assert_eq!(formatter.format_to_string(), "Hello World");
//! ```

mod blob_exporter;

pub use blob_exporter::BlobExporter;
