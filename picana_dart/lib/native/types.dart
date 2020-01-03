import 'dart:ffi';
import 'package:ffi/ffi.dart' show Utf8, allocate, free;

/// SPN definitions bridge
class Defined extends Struct {
	@Int8()
	int available; // From the resource -> Whether a definition is available

	factory Defined.allocate(int available) =>
			allocate<Defined>().ref
			..available = available;
}

// Maintain order to be similar to the Struct!! -> (Not sure why but it worked?)
class Frame extends Struct {
	@Uint64()
	int timestamp;

	@Uint32()
	int id;

	//@ffidart.Pointer() -> Not needed(or even working!)
	Pointer<Utf8> device;

	Pointer<Uint8> data;

	@Int8()
	int remote;

	@Int8()
	int error;

	@Int8()
	int extended;

	@Uint32()
	int error_code;

	//factory Frame.allocate(int t_usec, int id, ffidart.Pointer<Utf8> device, ffidart.Pointer<int> data, int remote, int error, int extended) =>
	factory Frame.allocate(int t_usec, int id, Pointer<Utf8> device, Pointer<Uint8> data, int remote, int error, int extended, int error_code) =>
			allocate<Frame>().ref
			..timestamp = t_usec
			..id = id
			..device = device
			..data = data
			..remote = remote
			..error = error
			..extended = extended
			..error_code = error_code;

	// Are we responsible for this memory
	void dispose(){
		free(data);
		free(device);
	}
}

class LiteFrame extends Struct {

	@Uint32()
	int id;

	Pointer<Uint8> data;

	@Int8()
	int remote;

	@Int8()
	int error;

	factory LiteFrame.allocate(int id, Pointer<Uint8> data, bool remote, bool error) =>
			allocate<LiteFrame>().ref
			..id = id
			..data = data
			..remote = remote ? 1 : 0
			..error = error ? 1 : 0;

	// Are we responsible for this memory?
	void dispose(){
		free(data);
	}
}
