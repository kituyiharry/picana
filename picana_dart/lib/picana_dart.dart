import 'dart:ffi' as ffidart;
import 'package:ffi/ffi.dart';
import 'dart:typed_data';
import 'dart:io';

// FFI signature of the hello_world C function
typedef ffi_func = ffidart.Int32 Function(ffidart.Pointer<Utf8> x, ffidart.Pointer<Utf8> y);  //pub extern fn rust_fn(x: i32) -> i32
// Dart type definition for calling the C foreign function
typedef dart_func = int Function(ffidart.Pointer<Utf8> x, ffidart.Pointer<Utf8> y);

typedef line_ffi_func = ffidart.Pointer<Utf8> Function(ffidart.Pointer<Utf8> key, ffidart.Int32 y);  //pub extern fn rust_fn(x: i32) -> i32
typedef can_ffi_func = ffidart.Pointer<Frame> Function(ffidart.Pointer<Utf8> key, ffidart.Int32 y);  //pub extern fn rust_fn(x: i32) -> i32
// Dart type definition for calling the C foreign function
typedef line_dart_func = ffidart.Pointer<Utf8> Function(ffidart.Pointer<Utf8> x, int y);
typedef can_dart_func =  ffidart.Pointer<Frame> Function(ffidart.Pointer<Utf8> x, int y);

void calculate() {
	// Open the dynamic library
	var path = './libpicana.so';
	final dylib = ffidart.DynamicLibrary.open(path);
	// Look up the Rust/C function
	final line_dart_func native_line_func = dylib.lookup<ffidart.NativeFunction<line_ffi_func>>('line').asFunction();
	final dart_func native_func = dylib.lookup<ffidart.NativeFunction<ffi_func>>('openfile').asFunction();
	final can_dart_func native_can_func = dylib.lookup<ffidart.NativeFunction<can_ffi_func>>('canframedata').asFunction();

	final cmdP = Utf8.toUtf8("/run/media/harryk/Backup/OPIBUS/c-dashboard/docs/dumps/RinehartCtr.log");
	final cmdb = Utf8.toUtf8("zeva");

	final bytes = native_func(cmdP, cmdb);
	int i = 0;

	while (i <  bytes) {

		final last_line = native_line_func(cmdb, i);
		final ffidart.Pointer<Frame> frame = native_can_func(cmdb, i);
		final finframe = frame.ref;
		final decoded = Utf8.fromUtf8(last_line);
		final device = Utf8.fromUtf8(finframe.device);

		//sleep(const Duration(milliseconds:200));

	    stderr.write(' Bytes: ${bytes} -> ${decoded} ');
		stderr.write(' [Timestamp | Id] -> ${finframe.timestamp} ${finframe.id} ');
		stderr.write(' [Device] -> ${device} ');
		stderr.write(' [Remote] -> ${finframe.remote} ');
		stderr.write(' [Data] -> ${finframe.data.asTypedList(8)} ');
		stderr.write(' [Error | Extended] -> ${finframe.error} ${finframe.extended}\r');
		i++;
		free(last_line);
		free(frame);
	}
	free(cmdP);
	free(cmdb);
}

class Frame extends ffidart.Struct {
	@ffidart.Uint64()
	int timestamp;

	@ffidart.Uint32()
	int id;

	//@ffidart.Pointer() -> Not needed(or even working!)
	ffidart.Pointer<Utf8> device;

	ffidart.Pointer<ffidart.Uint8> data;

	@ffidart.Int8()
	int remote;

	@ffidart.Uint32()
	int error;

	@ffidart.Int8()
	int extended;

	//factory Frame.allocate(int t_usec, int id, ffidart.Pointer<Utf8> device, ffidart.Pointer<int> data, int remote, int error, int extended) =>
	factory Frame.allocate(int t_usec, int id, ffidart.Pointer<Utf8> device, ffidart.Pointer<ffidart.Uint8> data, int remote, int error, int extended) =>
			allocate<Frame>().ref
			..timestamp = t_usec
			..id = id
			..device = device
			..data = data
			..remote = remote
			..error = error
			..extended = extended;

	// Are we responsible for this memory
	void dispose(){
		//free(data);
		//free(device);
	}
}
