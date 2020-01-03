import 'dart:ffi' as ffidart;
import 'package:ffi/ffi.dart';
import 'dart:typed_data';
import 'dart:io';
import 'dart:async';
import 'dart:isolate';
import 'dart:typed_data';

import './native/types.dart';
import './native/signatures.dart';
import './native/constants.dart';

//probably a ffidart.Int32 Function(ffidart.Int32 num)
int myFunc(ffidart.Pointer<Frame> frame) {
	final mframe = frame.ref;
	print("Called MyFunc -> ${mframe.id};");
	free(frame);
	print("After free => MyFunc -> ${mframe.id};");
	return 0;
}

void spawnlistenerasync(SendPort sendPort) {
	final p2Fun = ffidart.Pointer.fromFunction<local_myFunc>(myFunc, 0);
	var path = LIBNAME; // Linux only
	final dylib = ffidart.DynamicLibrary.open(path);
	final listen_dart_func native_listen = dylib.lookup<ffidart.NativeFunction<listen_ffi_func>>('listen').asFunction();
	print("Pointer -> $p2Fun");
	print("Running listener");
	native_listen(p2Fun);
	print("Listener should be  done now!");
}


void calculate() async {
	// Open the dynamic library
	var path = LIBNAME; // Linux only
	final dylib = ffidart.DynamicLibrary.open(path);
	// Look up the Rust/C function
	final line_dart_func native_line_func = dylib.lookup<ffidart.NativeFunction<line_ffi_func>>(LINE_FUNC).asFunction();
	final dart_func native_func = dylib.lookup<ffidart.NativeFunction<ffi_func>>(OPEN_FILE_FUNC).asFunction();
	final can_dart_func native_can_func = dylib.lookup<ffidart.NativeFunction<can_ffi_func>>(CANFRAME_FUNC).asFunction();
	final exp_dart_func native_exp_func = dylib.lookup<ffidart.NativeFunction<exp_ffi_func>>(EXPLAIN_FUNC).asFunction();
	final invoke_dart_func native_invoke = dylib.lookup<ffidart.NativeFunction<invoke_ffi_func>>('invoke').asFunction();
	final connect_dart_func native_connect = dylib.lookup<ffidart.NativeFunction<connect_ffi_func>>(CONNECT_FUNC).asFunction();
	final say_dart_func native_say = dylib.lookup<ffidart.NativeFunction<say_ffi_func>>(SAY_FUNC).asFunction();
	final kill_dart_func native_kill = dylib.lookup<ffidart.NativeFunction<kill_ffi_func>>(TERMINATE_FUNC).asFunction();
	final silence_dart_func native_silence = dylib.lookup<ffidart.NativeFunction<silence_ffi_func>>(SILENCE_FUNC).asFunction();
	//final listen_dart_func native_listen = dylib.lookup<ffidart.NativeFunction<listen_ffi_func>>('listen').asFunction();

	final cmdP = Utf8.toUtf8("/run/media/harryk/Backup/OPIBUS/c-dashboard/docs/dumps/Zeva-running.log");
	final cmdb = Utf8.toUtf8("zeva");
	final iface = Utf8.toUtf8("vcan0");
	final ifaceb = Utf8.toUtf8("vcan1");


	final p2Fun = ffidart.Pointer.fromFunction<local_myFunc>(myFunc, 0);

	print("Pointer -> $p2Fun");
	final ret = native_connect(iface); //p2Fun
	var receivePort = new ReceivePort();
	var v = Isolate.spawn(spawnlistenerasync, receivePort.sendPort);
	
	
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

	//final lsn = native_listen(p2Fun);
	print("V is $v");

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
			//final b = native_invoke(explainerBc, finframe.data);
		    ffidart.Pointer<ffidart.Uint8> p = allocate();
			//ffidart.Pointer<ffidart.Uint8> u = allocate();
			final data = [99, 101, 102, 103, 104, 105, 106, 107];
			for (var i = 0, len = data.length; i < len; ++i) {
			  //print("Allocating $i with ${data[i]}");
			  p[i] = data[i];
			  //u[i] = data[i];
			}
			//This should return a pointer!
			final liteframe = allocate<LiteFrame>();
			liteframe.ref.id = 30;
			liteframe.ref.data = p;
			liteframe.ref.remote = 0;
			liteframe.ref.error = 0;
			// Rust now owns the data!
			final b = native_say(iface, liteframe);
			//NB: p is invalid from here after being passed to liteframe!
			//stderr.write('\tTold 30 ${p.asTypedList(8)} || ${finframe.data.asTypedList(8)}: $b\n');
		}


		//print("...\r");
		//sleep(const Duration(milliseconds:50));

		//stderr.write(' Bytes: ${bytes} -> ${decoded} ');
		stderr.write(' [Timestamp | Id] -> ${finframe.timestamp} ${finframe.id} \n');
		//stderr.write(' [Device] -> ${device} ');
		//stderr.write(' [Remote] -> ${finframe.remote} ');
		//stderr.write('\t [Data] -> ${finframe.data.asTypedList(8)} \r');
		//stderr.write('\t\t[Error | Extended] -> ${finframe.error} ${finframe.extended}\r\e[K');
		i++;
		free(last_line);
		free(frame);
	}
	print("Try connect!");
	// Deadlock happens here!
	final retb = native_connect(ifaceb); //p2Fun
	print("Connecting got [$ret, $retb]\n");
	//sleep(const Duration(milliseconds:4500));
	native_silence();
	native_kill(iface); //so now this is blocking!
	native_kill(ifaceb);
	final b = await v;
	//print("Should leave now! -> $b");
	//A Dart program terminates when all its isolates have terminated.
	//An isolate is terminated if there are no more events in the event loop and there are no open ReceivePorts anymore. 
	//To achieve that, the Dart VM tracks all ReceivePorts that an isolate created. 
	//An extreme example is void main() => ReceivePort(), a program which does not terminate.
	//By closing the port in the main isolate at the end, the program indeed terminates:
	print("Closing port!\n");
	receivePort.close(); // Required to close else dart itself wont terminate!!
	print("Closed port!\n");
	free(explainerA);
	free(explainerT);
	free(explainerBc);
	free(explainerBv);
	free(explainerF);
	free(cmdP);
	free(cmdb);
	free(iface);
}
