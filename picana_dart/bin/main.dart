import 'package:picana_dart/picana_dart.dart' as picana_dart;
import 'dart:isolate';
import 'dart:async';
 
// Demonstrate command line usage!
void main(List<String> arguments) async {
  final port = new ReceivePort();

  port.listen((item){
	  print("Handling -> $item");
	  // This is where to close the port
	  //TODO: Use message types to close the port!
	  if(item is int && item > 0){
		  port.close();
	  }
  });

  await picana_dart.calculate(port.sendPort);

  //Timer(Duration(seconds: 3), (){ print("Timer finished"); port.close(); });

}
