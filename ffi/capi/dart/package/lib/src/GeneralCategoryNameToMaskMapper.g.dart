// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// A type capable of looking up General Category mask values from a string name.
///
/// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.get_name_to_enum_mapper) for more information.
///
/// See the [Rust documentation for `PropertyValueNameToEnumMapper`](https://docs.rs/icu/latest/icu/properties/names/struct.PropertyValueNameToEnumMapper.html) for more information.
final class GeneralCategoryNameToMaskMapper implements ffi.Finalizable {
  final ffi.Pointer<ffi.Opaque> _underlying;

  GeneralCategoryNameToMaskMapper._(this._underlying) {
    _finalizer.attach(this, _underlying.cast());
  }

  static final _finalizer = ffi.NativeFinalizer(_capi('ICU4XGeneralCategoryNameToMaskMapper_destroy'));

  /// Get the mask value matching the given name, using strict matching
  ///
  /// Returns 0 if the name is unknown for this property
  int getStrict(String name) {
    final temp = ffi2.Arena();
    final nameView = name.utf8View;
    final result = _ICU4XGeneralCategoryNameToMaskMapper_get_strict(_underlying, nameView.pointer(temp), nameView.length);
    temp.releaseAll();
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XGeneralCategoryNameToMaskMapper_get_strict =
    _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XGeneralCategoryNameToMaskMapper_get_strict')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// Get the mask value matching the given name, using loose matching
  ///
  /// Returns 0 if the name is unknown for this property
  int getLoose(String name) {
    final temp = ffi2.Arena();
    final nameView = name.utf8View;
    final result = _ICU4XGeneralCategoryNameToMaskMapper_get_loose(_underlying, nameView.pointer(temp), nameView.length);
    temp.releaseAll();
    return result;
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XGeneralCategoryNameToMaskMapper_get_loose =
    _capi<ffi.NativeFunction<ffi.Uint32 Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, ffi.Size)>>('ICU4XGeneralCategoryNameToMaskMapper_get_loose')
      .asFunction<int Function(ffi.Pointer<ffi.Opaque>, ffi.Pointer<ffi.Uint8>, int)>(isLeaf: true);

  /// See the [Rust documentation for `get_name_to_enum_mapper`](https://docs.rs/icu/latest/icu/properties/struct.GeneralCategoryGroup.html#method.get_name_to_enum_mapper) for more information.
  ///
  /// Throws [Error] on failure.
  factory GeneralCategoryNameToMaskMapper.load(DataProvider provider) {
    final result = _ICU4XGeneralCategoryNameToMaskMapper_load(provider._underlying);
    if (!result.isOk) {
      throw Error.values.firstWhere((v) => v._underlying == result.union.err);
    }
    return GeneralCategoryNameToMaskMapper._(result.union.ok);
  }

  // ignore: non_constant_identifier_names
  static final _ICU4XGeneralCategoryNameToMaskMapper_load =
    _capi<ffi.NativeFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>>('ICU4XGeneralCategoryNameToMaskMapper_load')
      .asFunction<_ResultOpaqueInt32 Function(ffi.Pointer<ffi.Opaque>)>(isLeaf: true);
}
