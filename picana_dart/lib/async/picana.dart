import 'dart:async';
import 'package:ffi/ffi.dart' show Utf8;

import '../native/picana.dart';


//Async patterns for accessing Picana utilities
class AsyncPicana {

	static AsyncPicana _sAsyncPicana = new AsyncPicana._internal();

	Picana mPicana;

	factory AsyncPicana(){
		return _sAsyncPicana;
	}
	
	AsyncPicana._internal() {
		mPicana = Picana();
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
	Future<int> startConnection(String interface) async {
		final utfInterface = Utf8.toUtf8(interface);
		await mPicana.native_connect(utfInterface);
	}


	static void startConnectionListener(Function(dynamic message) listener){
	}
}
