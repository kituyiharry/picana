import 'dart:ffi';

import './_constants.dart';
import './signatures.dart';
//Proxy the Picana native library
class Picana {

	static final _sPicanaProxy = new Picana._internal();
	static final DynamicLibrary _dyLib = DynamicLibrary.open(LIBNAME);

	///Is there a better way to do this?
	final line_dart_func native_line_func = _dyLib.lookup<NativeFunction<line_ffi_func>>(LINE_FUNC).asFunction();
	final dart_func native_func = _dyLib.lookup<NativeFunction<ffi_func>>(OPEN_FILE_FUNC).asFunction();
	final can_dart_func native_can_func = _dyLib.lookup<NativeFunction<can_ffi_func>>(CANFRAME_FUNC).asFunction();
	final exp_dart_func native_exp_func = _dyLib.lookup<NativeFunction<exp_ffi_func>>(EXPLAIN_FUNC).asFunction();
	final invoke_dart_func native_invoke = _dyLib.lookup<NativeFunction<invoke_ffi_func>>(INVOKE_FUNC).asFunction();
	final connect_dart_func native_connect = _dyLib.lookup<NativeFunction<connect_ffi_func>>(CONNECT_FUNC).asFunction();
	final say_dart_func native_say = _dyLib.lookup<NativeFunction<say_ffi_func>>(SAY_FUNC).asFunction();
	final kill_dart_func native_kill = _dyLib.lookup<NativeFunction<kill_ffi_func>>(TERMINATE_FUNC).asFunction();
	final listen_dart_func native_listen = _dyLib.lookup<NativeFunction<listen_ffi_func>>(LISTEN_FUNC).asFunction();
	final silence_dart_func native_silence = _dyLib.lookup<NativeFunction<silence_ffi_func>>(SILENCE_FUNC).asFunction();

	factory Picana(){
		return _sPicanaProxy;
	}

	// Lookup all required functions!
	Picana._internal(){}

}
