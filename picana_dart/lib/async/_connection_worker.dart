import 'dart:isolate';

class ConnectionWorker extends Isolate {
	Worker() : super.heavy();

	main() {
		this.port.receive(
				void _(var message, SendPort replyTo) {
					print ("Worker receives: ${message}");
					replyTo.send("Pong");
					this.port.close();
				}
		);
	}
}
