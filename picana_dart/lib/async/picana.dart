import 'dart:async';
import 'dart:isolate';
import 'package:ffi/ffi.dart' show Utf8;

import '../native/picana.dart';
import './_connection_listener.dart';


//Async patterns for accessing Picana utilities
class AsyncPicana {

	static AsyncPicana _sAsyncPicana = new AsyncPicana._internal();

	Picana mPicana;
	ConnectionIsolate mIsolate;


	factory AsyncPicana(){
		return _sAsyncPicana;
	}
	
	AsyncPicana._internal() {
		mPicana = Picana();
		mIsolate = ConnectionIsolate();
	}

	//Returns the number of bytes!
	Future<int> loadCanDump(String fileName, String fileKey) async {
		final utfFilename = Utf8.toUtf8(fileName);
		final utfFileKey = Utf8.toUtf8(fileKey);
		await mPicana.native_func(utfFilename, utfFileKey);
	}

	// Load a dbc file!
	Future<int> loadDBC(String fileName, String fileKey) async {
		final utfFilename = Utf8.toUtf8(fileName);
		final utfFileKey = Utf8.toUtf8(fileKey);
		await mPicana.native_dbc(utfFilename, utfFileKey);
	}

	//Connect to an interface e.g can0, vcan1
	Future<int> connect(String interface) async {
		final utfInterface = Utf8.toUtf8(interface);
		final conn = mPicana.native_connect(utfInterface);
		return Future.value(conn);
	}

	void startConnectionListener(){
		print("Starting Connection Listener");
		mIsolate.startConnection().then((nullable){
			print("Starting Connection!");
		});
	}

}
