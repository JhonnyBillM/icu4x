// @generated
include!("macros.rs");
/// Implement [`DataProvider<M>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_data_provider {
    ($ provider : path) => {
        impl_fallback_likelysubtags_v1!($provider);
        impl_fallback_parents_v1!($provider);
        impl_fallback_supplement_co_v1!($provider);
    };
}
#[doc(inline)]
pub use __impl_data_provider as impl_data_provider;
/// Implement [`AnyProvider`](icu_provider::AnyProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                const FALLBACK_LIKELYSUBTAGS_V1: icu_provider::DataKeyHash = <icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const FALLBACK_PARENTS_V1: icu_provider::DataKeyHash = <icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const FALLBACK_SUPPLEMENT_CO_V1: icu_provider::DataKeyHash = <icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    FALLBACK_LIKELYSUBTAGS_V1 => icu_provider::DataProvider::<icu_provider_adapters::fallback::provider::LocaleFallbackLikelySubtagsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    FALLBACK_PARENTS_V1 => icu_provider::DataProvider::<icu_provider_adapters::fallback::provider::LocaleFallbackParentsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    FALLBACK_SUPPLEMENT_CO_V1 => icu_provider::DataProvider::<icu_provider_adapters::fallback::provider::CollationFallbackSupplementV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.61"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);