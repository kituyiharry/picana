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
typedef exp_ffi_func = ffidart.Pointer<Defined> Function(ffidart.Pointer<Utf8> key, ffidart.Pointer<Utf8> parameter);  //pub extern fn rust_fn(x: i32) -> i32
typedef invoke_ffi_func = ffidart.Float Function(ffidart.Pointer<Defined> defined, ffidart.Pointer<ffidart.Uint8> data);  //pub extern fn rust_fn(x: i32) -> i32
// Dart type definition for calling the C foreign function
typedef line_dart_func = ffidart.Pointer<Utf8> Function(ffidart.Pointer<Utf8> x, int y);
typedef can_dart_func =  ffidart.Pointer<Frame> Function(ffidart.Pointer<Utf8> x, int y);
typedef exp_dart_func =  ffidart.Pointer<Defined> Function(ffidart.Pointer<Utf8> x, ffidart.Pointer<Utf8> z);

// Weird how this actually works!?
typedef invoke_dart_func = double Function(ffidart.Pointer<Defined> defined, ffidart.Pointer<ffidart.Uint8> data);  //pub extern fn rust_fn(x: i32) -> i32


void calculate() {
	// Open the dynamic library
	var path = './libpicana.so'; // Linux only
	final dylib = ffidart.DynamicLibrary.open(path);
	// Look up the Rust/C function
	final line_dart_func native_line_func = dylib.lookup<ffidart.NativeFunction<line_ffi_func>>('line').asFunction();
	final dart_func native_func = dylib.lookup<ffidart.NativeFunction<ffi_func>>('openfile').asFunction();
	final can_dart_func native_can_func = dylib.lookup<ffidart.NativeFunction<can_ffi_func>>('canframedata').asFunction();
	final exp_dart_func native_exp_func = dylib.lookup<ffidart.NativeFunction<exp_ffi_func>>('explainer').asFunction();
	final invoke_dart_func native_invoke = dylib.lookup<ffidart.NativeFunction<invoke_ffi_func>>('invoke').asFunction();

	final cmdP = Utf8.toUtf8("/run/media/harryk/Backup/OPIBUS/c-dashboard/docs/dumps/Zeva-running.log");
	final cmdb = Utf8.toUtf8("zeva");

	final bytes = native_func(cmdP, cmdb);
	int i = 0;

	final explainTemp  = Utf8.toUtf8("Temperature");
	final explainAux = Utf8.toUtf8("AuxVoltage");
	final explainBatV = Utf8.toUtf8("BatVolts");
	final explainBatC = Utf8.toUtf8("BatCurrent");
	final explainFalse = Utf8.toUtf8("False!"); // always returns 0

	final explainerT  = native_exp_func(cmdb, explainTemp);
	final explainerA  = native_exp_func(cmdb, explainAux);
	final explainerBv = native_exp_func(cmdb, explainBatV);
	final explainerBc = native_exp_func(cmdb, explainBatC);
	final explainerF  = native_exp_func(cmdb, explainFalse); // Should be unavailable!

	print("Explainers Available? -> [${explainerBc.ref.available}, ${explainerBv.ref.available}, ${explainerF.ref.available}] \n");

	while (i < bytes) {

		final last_line = native_line_func(cmdb, i);
		final ffidart.Pointer<Frame> frame = native_can_func(cmdb, i);
		final finframe = frame.ref;
		final decoded = Utf8.fromUtf8(last_line);
		final device = Utf8.fromUtf8(finframe.device);


		if(finframe.id == 30){
			final t = native_invoke(explainerT, finframe.data);
			final a = native_invoke(explainerA, finframe.data).toStringAsFixed(3);
			final b = native_invoke(explainerBv, finframe.data);
			final f = native_invoke(explainerF, finframe.data); //Should be 0 always!
			stderr.write('${finframe.id} : Temp -> $t  \tAux -> $a\t Bat -> $b\t F -> $f\n');
		} 

		if(finframe.id == 40){
			final b = native_invoke(explainerBc, finframe.data);
			stderr.write('${finframe.id} : Bat -> $b \n');
		}


		//sleep(const Duration(milliseconds:200));

		//stderr.write(' Bytes: ${bytes} -> ${decoded} ');
		//stderr.write(' [Timestamp | Id] -> ${finframe.timestamp} ${finframe.id} ');
		//stderr.write(' [Device] -> ${device} ');
		//stderr.write(' [Remote] -> ${finframe.remote} ');
		//stderr.write(' [Data] -> ${finframe.data.asTypedList(8)} ');
		//stderr.write(' [Error | Extended] -> ${finframe.error} ${finframe.extended}\r');
		i++;
		free(last_line);
		free(frame);
	}
	free(explainerA);
	free(explainerT);
	free(explainerBc);
	free(explainerBv);
	free(explainerF);
	free(cmdP);
	free(cmdb);
}

class Defined extends ffidart.Struct {
	@ffidart.Int8()
	int available; // From the resource -> Whether a definition is available

	factory Defined.allocate(int available) =>
			allocate<Defined>().ref
			..available = available;
}

// Maintain order to be similar to the Struct!! -> (Not sure why but it worked?)
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
