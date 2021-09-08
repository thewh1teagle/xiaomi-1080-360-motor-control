module main

fn message_handler(message string) {
}

fn main() {
	println('press ctrl-c to quit...')
	motor_api := load_motor_api() ?
	// start_server(3000, message_handler) ?
	motor_api.pan(2.0, Direction.forward)
}