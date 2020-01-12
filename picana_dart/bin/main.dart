import 'package:picana_dart/picana_dart.dart' as picana_dart;
import 'dart:isolate';
import 'dart:async';
 
// Demonstrate command line usage!
void main(List<String> arguments) {
  final port = new ReceivePort();

  port.listen((item){
	  print("Handling -> $item");
	  // This is where to close the port
	  //TODO: Use message types to close the port!
	  if(item is int && item > 0){
		  port.close();
	  }
  });

  picana_dart.calculate(port.sendPort).then((val){
	  //picana_dart.picana.native_silence();
	  print("The Val! => $val");
  });

  Timer(
		  Duration(seconds: 21), (){ print("Timer finished"); 
	  port.close(); });

}
