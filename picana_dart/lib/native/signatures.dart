import 'dart:ffi';
import 'package:ffi/ffi.dart' show Utf8;
import './types.dart';

typedef ffi_func = Int32 Function(Pointer<Utf8> , Pointer<Utf8> );
typedef dart_func = int Function(Pointer<Utf8> , Pointer<Utf8> );

typedef line_ffi_func = Pointer<Utf8> Function(Pointer<Utf8> , Int32 );
typedef line_dart_func = Pointer<Utf8> Function(Pointer<Utf8> , int );

typedef can_ffi_func = Pointer<Frame> Function(Pointer<Utf8> , Int32);
typedef can_dart_func =  Pointer<Frame> Function(Pointer<Utf8> , int );

typedef exp_ffi_func = Pointer<Defined> Function(Pointer<Utf8> , Pointer<Utf8> );
typedef exp_dart_func =  Pointer<Defined> Function(Pointer<Utf8> , Pointer<Utf8> );

typedef invoke_ffi_func = Float Function(Pointer<Defined> , Pointer<Uint8> );
typedef invoke_dart_func = double Function(Pointer<Defined> , Pointer<Uint8> );

typedef local_myFunc = Int32 Function(Pointer<Frame>);

typedef connect_ffi_func = Int32 Function(Pointer<Utf8> );
typedef connect_dart_func = int Function(Pointer<Utf8> );

typedef listen_ffi_func = Int32 Function(Pointer<NativeFunction<local_myFunc>> );
typedef listen_dart_func = int Function(Pointer<NativeFunction<local_myFunc>> );

typedef say_ffi_func = Int32 Function(Pointer<Utf8>, Pointer<LiteFrame>);
typedef say_dart_func = int Function(Pointer<Utf8>, Pointer<LiteFrame>);

typedef kill_ffi_func = Int32 Function(Pointer<Utf8>);
typedef kill_dart_func = int Function(Pointer<Utf8>);

typedef silence_ffi_func = Int32 Function();
typedef silence_dart_func = int Function();
