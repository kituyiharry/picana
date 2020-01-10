import 'dart:ffi' as ffidart;
import 'package:ffi/ffi.dart';
import 'dart:typed_data';
import 'dart:io';
import 'dart:async';
import 'dart:isolate';
import 'dart:typed_data';

import './native/types.dart';
import './native/signatures.dart';
import './native/picana.dart';
import './async/picana.dart';

//probably a ffidart.Int32 Function(ffidart.Int32 num)
int myFunc(ffidart.Pointer<Frame> frame) {
	final mframe = frame.ref;
	print("Called MyFunc -> ${mframe.id};");
	free(frame);
	print("After free => MyFunc -> ${mframe.id};");
	return 0;
}

void spawnlistenerasync(SendPort sendPort) {
	//final picana = Picana();
	//final apicana = AsyncPicana();
	//final p2Fun = ffidart.Pointer.fromFunction<local_myFunc>(myFunc, 0);
	//print("Pointer -> $p2Fun");
	print("Running listener");
	//picana.native_listen(p2Fun);
	print("Listener should be  done now!");
}


Future<Isolate> calculate(SendPort port) async {
	// Open the dynamic library
	final async_picana = AsyncPicana();

	//print("Opening VCAN0");

	await async_picana.connect("vcan0", port.nativePort).then((value){
		print("Connected => $value");
	});

	final isolate = async_picana.startConnectionListener(port);

	async_picana.picana.native_silence();

	return isolate;

	//print("Pushing!!");
	//print("Launched Connection Listener($back)! -- waiting 30 sec");
	//rport.close();

	/*final picana = Picana();
	final cmdP = Utf8.toUtf8("/run/media/harryk/Backup/OPIBUS/c-dashboard/docs/dumps/Zeva-running.log");
	final cmdc = Utf8.toUtf8("./zeva_30.dbc");
	final cmdb = Utf8.toUtf8("zeva");
	final iface = Utf8.toUtf8("vcan0");
	final ifaceb = Utf8.toUtf8("vcan1");


	final p2Fun = ffidart.Pointer.fromFunction<local_myFunc>(myFunc, 0);

	print("Pointer -> $p2Fun");
	final ret = picana.native_connect(iface); //p2Fun
	var receivePort = new ReceivePort();
	//var v = await Isolate.spawn(spawnlistenerasync, receivePort.sendPort);


	final dbc = picana.native_dbc(cmdc, cmdb);
	final bytes = picana.native_func(cmdP, cmdb);
	int i = 0;

	print("Bytes => $bytes, DBC => $dbc");

	final explainTemp  = Utf8.toUtf8("Temperature");
	final explainAux = Utf8.toUtf8("AuxVoltage");
	final explainBatV = Utf8.toUtf8("BatVolts");
	final explainBatC = Utf8.toUtf8("BatCurrent");
	final explainFalse = Utf8.toUtf8("False!"); // always returns 0

	final explainerT  = picana.native_exp_func(cmdb, explainTemp);
	final explainerA  = picana.native_exp_func(cmdb, explainAux);
	final explainerBv = picana.native_exp_func(cmdb, explainBatV);
	final explainerBc = picana.native_exp_func(cmdb, explainBatC);
	final explainerF  = picana.native_exp_func(cmdb, explainFalse); // Should be unavailable!

	print("Explainers Available? -> [${explainerA.ref.available}, ${explainerT.ref.available}, ${explainerF.ref.available}] \n");

	//final lsn = native_listen(p2Fun);
	//print("V is $v");
	//final b = await v;

	/*while (i < bytes) {

	  final last_line = picana.native_line_func(cmdb, i);
	  final ffidart.Pointer<Frame> frame = picana.native_can_func(cmdb, i);
	  final finframe = frame.ref;
	  final decoded = Utf8.fromUtf8(last_line);
	  final device = Utf8.fromUtf8(finframe.device);


	  if(finframe.id == 30){
	  print("ID 30 Found!");
	  final t = picana.native_invoke(explainerT, finframe.data);
	  final a = picana.native_invoke(explainerA, finframe.data).toStringAsFixed(3);
	  final b = picana.native_invoke(explainerBv, finframe.data);
	  final f = picana.native_invoke(explainerF, finframe.data); //Should be 0 always!
	  stderr.write('${finframe.id} : Temp -> $t  \tAux -> $a\t Bat -> $b\t F -> $f\n');
	  } 

	  if(finframe.id == 40){
	  print("ID 40 Found!");
	//final b = native_invoke(explainerBc, finframe.data);
	//ffidart.Pointer<ffidart.Uint8> p = allocate();
	//ffidart.Pointer<ffidart.Uint8> u = allocate();

	final data = [99, 101, 102, 103, 104, 105, 106, 107];
	final liteframe = picana.createFrame(30, data);

	// Rust now owns the data!
	final b = picana.native_say(iface, liteframe);
	//print("Frame is ${liteframe.ref.id} - ${b}");
	//NB: p is invalid from here after being passed to liteframe!
	//stderr.write('\tTold 30 ${p.asTypedList(8)} || ${finframe.data.asTypedList(8)}: $b\n');
	}


	//print("...\r");
	sleep(const Duration(milliseconds:50));

	//stderr.write(' Bytes: ${bytes} -> ${decoded} ');
	//stderr.write(' [Timestamp | Id] -> ${finframe.timestamp} ${finframe.id} \n');
	//stderr.write(' [Device] -> ${device} ');
	//stderr.write(' [Remote] -> ${finframe.remote} ');
	//stderr.write('\t [Data] -> ${finframe.data.asTypedList(8)} \r');
	//stderr.write('\t\t[Error | Extended] -> ${finframe.error} ${finframe.extended}\r\e[K');
	i++;
	free(last_line);
	free(frame);
	}*/
	print("Try connect!");
	final retb = picana.native_connect(ifaceb); //p2Fun
	print("Connecting got [$ret, $retb]\n");
	//sleep(const Duration(milliseconds:4500));
	picana.native_silence();
	picana.native_kill(iface); //so now this is blocking!
	picana.native_kill(ifaceb);

	picana.dispose();
	//print("Should leave now! -> $b");
	//A Dart program terminates when all its isolates have terminated.
	//An isolate is terminated if there are no more events in the event loop and there are no open ReceivePorts anymore. 
	//To achieve that, the Dart VM tracks all ReceivePorts that an isolate created. 
	//An extreme example is void main() => ReceivePort(), a program which does not terminate.
	//By closing the port in the main isolate at the end, the program indeed terminates:
	print("Closing port!\n");
	receivePort.close(); // Required to close else dart itself wont terminate!!
	rport.close();
	print("Closed port!\n");
	free(explainerA);
	free(explainerT);
	free(explainerBc);
	free(explainerBv);
	free(explainerF);
	free(cmdP);
	free(cmdb);
	free(iface);*/
}
