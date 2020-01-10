import 'dart:async';
import 'dart:isolate';
import 'package:ffi/ffi.dart' show Utf8;

import '../native/picana.dart';
//import './_connection_listener.dart';
import './_connection_bridge.dart';


//Async patterns for accessing Picana utilities
class AsyncPicana {

	static AsyncPicana _sAsyncPicana = new AsyncPicana._internal();

	Picana mPicana;
	//ConnectionIsolate mIsolate;

	get picana => mPicana;


	factory AsyncPicana(){
		return _sAsyncPicana;
	}
	
	AsyncPicana._internal() {
		mPicana = Picana();
		//mIsolate = ConnectionIsolate();
	}

	//Returns the number of bytes!
	Future<int> loadCanDump(String fileName, String fileKey) async {
		final utfFilename = Utf8.toUtf8(fileName);
		final utfFileKey = Utf8.toUtf8(fileKey);
		return Future.value(mPicana.native_func(utfFilename, utfFileKey));
	}

	// Load a dbc file!
	Future<int> loadDBC(String fileName, String fileKey) async {
		final utfFilename = Utf8.toUtf8(fileName);
		final utfFileKey = Utf8.toUtf8(fileKey);
		return Future.value(mPicana.native_dbc(utfFilename, utfFileKey));
	}

	//Connect to an interface e.g can0, vcan1
	Future<int> connect(String interface, int port) async {
		//Timer(Duration(seconds: 1), () => print("you should see me second"));
		print("Connecting to $interface");
		final utfInterface = Utf8.toUtf8(interface);
		final conn = mPicana.native_connect(utfInterface, port);
		return Future.value(conn);
	}

	Future<Isolate> startConnectionListener(SendPort port) async {
		//print("Starting Connection Listener");
		//mIsolate.startConnection().then((nullable){
			//print("[THEN] Starting Connection!");
		//});
		return await ConnectionBridge.withSender(port).spawn();
	}

}
