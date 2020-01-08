library picana;
import 'dart:ffi';
import 'dart:isolate';
import 'package:ffi/ffi.dart';

import './_constants.dart';
import './signatures.dart';
import './types.dart';
//Proxy the Picana native library


//import 'dart-ext:picana'; -- not supported in flutter!

class Picana {

	//ReceivePort mReceiver;
	//SendPort mSender;

	//get receiver => mReceiver;
	//get sender => mSender;

	///TODO: Copying can be reduced by using native Dart port...this is way too slow!!
	// TODO: Use nativeports to pass messages across threads. Think of threading logic here!
	//void set sender (SendPort sender){
		////final ret = native_primitive(sender.nativePort);
		////print("Sender ret= $ret");
		//mSender = sender;
	//}

	static final _sPicanaProxy = new Picana._internal();
	static final DynamicLibrary _dyLib = DynamicLibrary.open(LIBNAME);

	///Is there a better way to do this?
	final line_dart_func native_line_func = _dyLib.lookup<NativeFunction<line_ffi_func>>(LINE_FUNC).asFunction();
	final dart_func native_func = _dyLib.lookup<NativeFunction<ffi_func>>(OPEN_FILE_FUNC).asFunction();
	final dbc_dart_func native_dbc = _dyLib.lookup<NativeFunction<dbc_ffi_func>>(OPEN_DBC_FUNC).asFunction();
	final can_dart_func native_can_func = _dyLib.lookup<NativeFunction<can_ffi_func>>(CANFRAME_FUNC).asFunction();
	final exp_dart_func native_exp_func = _dyLib.lookup<NativeFunction<exp_ffi_func>>(EXPLAIN_FUNC).asFunction();
	final invoke_dart_func native_invoke = _dyLib.lookup<NativeFunction<invoke_ffi_func>>(INVOKE_FUNC).asFunction();
	final connect_dart_func native_connect = _dyLib.lookup<NativeFunction<connect_ffi_func>>(CONNECT_FUNC).asFunction();
	final say_dart_func native_say = _dyLib.lookup<NativeFunction<say_ffi_func>>(SAY_FUNC).asFunction();
	final kill_dart_func native_kill = _dyLib.lookup<NativeFunction<kill_ffi_func>>(TERMINATE_FUNC).asFunction();
	final listen_dart_func native_listen = _dyLib.lookup<NativeFunction<listen_ffi_func>>(LISTEN_FUNC).asFunction();
	final silence_dart_func native_silence = _dyLib.lookup<NativeFunction<silence_ffi_func>>(SILENCE_FUNC).asFunction();
	final primitive_dart_func native_primitive = _dyLib.lookup<NativeFunction<primitive_ffi_func>>('primitive').asFunction();

	factory Picana(){
		return _sPicanaProxy;
	}

	// Lookup all required functions!
	Picana._internal(){
		print("Creating a picana");
		//mReceiver = ReceivePort();
		//sender = null;
	}

	Pointer<LiteFrame> createFrame(int id, List<int> data, [bool remote = false, bool error = false]) {
		Pointer<Uint8> p = allocate();
		//final data = [99, 101, 102, 103, 104, 105, 106, 107];
		for (var i = 0, len = data.length; i < len; ++i) {
			p[i] = data[i];
		}
		final liteframe = allocate<LiteFrame>();
		liteframe.ref.id = id;
		liteframe.ref.data = p;
		liteframe.ref.remote = remote ? 1 : 0;
		liteframe.ref.error = error ? 1 : 0;
		return liteframe;
	}

	void dispose(){
		//mReceiver.close();
	}
}
