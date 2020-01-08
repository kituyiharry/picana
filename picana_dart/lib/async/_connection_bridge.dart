import 'dart:isolate';
import 'dart:ffi';
import '../native/picana.dart';
import '../native/types.dart';
import '../native/signatures.dart';

int myFunc(Pointer<Frame> frame) {
	final mframe = frame.ref;
	print("Called MyFunc -> ${mframe.id};");
	//free(frame);
	//print("After free => MyFunc -> ${mframe.id};");
	return 0;
}

class ConnectionBridge {

	Picana _picana; //Library functions!
	SendPort _sendPort;

	static ConnectionBridge _connectionBridge = ConnectionBridge._internal();

	void sender (SendPort sender){
		_sendPort = sender;
	}

	factory ConnectionBridge(){
		return _connectionBridge;
	}

	ConnectionBridge._internal(){
		_picana = Picana();
		_sendPort = null;
	}
	

	static ConnectionBridge withSender(SendPort sender) {
		return _connectionBridge..sender(sender);
	}

	static void _spawnMessageHandler(SendPort _sendPort) {
		//if(message is local_myFunc){
		print("In isolate!");
		final p2Fun = Pointer.fromFunction<local_myFunc>(myFunc, 0);
		//}
		Picana _pick = Picana();
		_pick.native_primitive(_sendPort.nativePort);
		//Todo -> Listen Async
		_pick.native_listen(p2Fun);
	}

	Future<Isolate> spawn() async {
		return Isolate.spawn(_spawnMessageHandler, _sendPort);
	}

}
