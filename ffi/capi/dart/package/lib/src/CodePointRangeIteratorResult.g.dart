// generated by diplomat-tool

// https://github.com/dart-lang/sdk/issues/53946
// ignore_for_file: non_native_function_type_argument_to_pointer

part of 'lib.g.dart';

/// Result of a single iteration of [`CodePointRangeIterator`].
/// Logically can be considered to be an `Option<RangeInclusive<u32>>`,
///
/// `start` and `end` represent an inclusive range of code points [start, end],
/// and `done` will be true if the iterator has already finished. The last contentful
/// iteration will NOT produce a range done=true, in other words `start` and `end` are useful
/// values if and only if `done=false`.
final class _CodePointRangeIteratorResultFfi extends ffi.Struct {
  @ffi.Uint32()
  external int start;
  @ffi.Uint32()
  external int end;
  @ffi.Bool()
  external bool done;
}

final class CodePointRangeIteratorResult {
  final _CodePointRangeIteratorResultFfi _underlying;

  CodePointRangeIteratorResult._(this._underlying);

  factory CodePointRangeIteratorResult() {
    final pointer = ffi2.calloc<_CodePointRangeIteratorResultFfi>();
    final result = CodePointRangeIteratorResult._(pointer.ref);
    _callocFree.attach(result, pointer.cast());
    return result;
  }

  int get start => _underlying.start;
  set start(int start) {
    _underlying.start = start;
  }

  int get end => _underlying.end;
  set end(int end) {
    _underlying.end = end;
  }

  bool get done => _underlying.done;
  set done(bool done) {
    _underlying.done = done;
  }

  @override
  bool operator ==(Object other) =>
      other is CodePointRangeIteratorResult &&
      other._underlying.start == _underlying.start &&
      other._underlying.end == _underlying.end &&
      other._underlying.done == _underlying.done;

  @override
  int get hashCode => Object.hashAll([
        _underlying.start,
        _underlying.end,
        _underlying.done,
      ]);
}
