import 'package:picana_dart/picana_dart.dart';
import 'dart:isolate';
import 'dart:async';
import 'dart:ffi'; // Needed for NativePort!

// Demonstrate command line usage!
void main(List<String> arguments) {
	print("ARGS IS => $arguments");
	final port = new ReceivePort();
	final sendport = port.sendPort;

	port.listen((item){
		print("[${DateTime.now().millisecondsSinceEpoch}] Handling -> $item");
		// This is where to close the port
		//TODO: Use message types to close the port!
		if(item is int && item > 0){
			port.close();
		}
	});

	final async_picana = AsyncPicana();

	async_picana.connect("vcan0", sendport.nativePort).then((value){
		print("Connected => $value");
	});

	//final isolate = async_picana.startConnectionListener(port);

	async_picana.connect("vcan1", port.sendPort.nativePort).then((value){
		print("Connected => $value");
	});

	Timer(
			Duration(seconds: 15), (){ 
				print("Timer finished"); 
				async_picana.picana.native_silence();
				port.close(); 
			});
}
