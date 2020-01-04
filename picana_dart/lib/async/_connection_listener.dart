import 'dart:async';
import 'dart:ffi';
import 'dart:isolate';

import '../native/types.dart';
import '../native/signatures.dart';
import '../native/picana.dart';

class ConnectionIsolate {

	//ReceivePort _receivePort;
	Picana _picana; //Current isolate picana!
	Isolate _connectionIsolate;

	static ConnectionIsolate _sConnectionIsolate = new ConnectionIsolate._internal();

	factory ConnectionIsolate(){
		return _sConnectionIsolate;
	}

	ConnectionIsolate._internal(){
		//_receivePort = ReceivePort();
		_picana = Picana();
		_connectionIsolate = null;
	}

	static int _connectionFrameHandler(Pointer<Frame> frame){
		final Picana picana = Picana();
		print("Sending frame! => $frame");
		//print("Received a frame => ${frame.ref.id} | ${frame.ref.timestamp}");
		//TODO: SEND maps! {key: value}
		picana.sender.send({"data": frame.ref.data.asTypedList(8)});
		free(frame);
		return 0;
	}

	//On current thread!
	void _connectionHandler(dynamic message){
		if(message is SendPort){
			print("SendPort Received! on Main Isolate");
		} else {
			print("Other thing received on Main isolate! => $message");
		}
	}

	Future<void> startConnection() async {
		var v = await Isolate.spawn(_startIsolate, _picana.receivePort.sendPort);
		//Our isolates picana!
		_picana.mReceivePort.listen(_connectionHandler);
	}

	static void _startIsolate(dynamic sendPort) async {
		//We are in a separate thread!
		final Picana picana = Picana();
		sendPort.send(picana.receivePort.sendPort);
		picana.sender = sendPort;
		final functionPointer = Pointer.fromFunction<local_myFunc>(_connectionFrameHandler, 0);
		//print("Natively => ${sendPort.nativePort}");
		picana.native_listen(functionPointer);
	}
}
