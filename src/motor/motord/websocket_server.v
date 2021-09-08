module main

import net.websocket
import term

fn start_server(port int, message_handler fn(string)) ? {
	mut s := websocket.new_server(.ip6, port, '')
	// Make that in execution test time give time to execute at least one time
	s.ping_interval = 100
	s.on_connect(fn (mut s websocket.ServerClient) ?bool {
		// Here you can look att the client info and accept or not accept
		// just returning a true/false
		if s.resource_name != '/' {
			return false
		}
		return true
	}) ?

	// on_message_ref, broadcast all incoming messages to all clients except the one sent it
	s.on_message_ref(fn (mut ws websocket.Client, msg &websocket.Message, mut m websocket.Server) ? {
		message_handler( msg.payload.bytestr() )
		// for _, cli in m.clients {
		// for i, _ in m.clients {
		// 	mut c := m.clients[i]
		// 	if c.client.state == .open && c.client.id != ws.id {
		// 		c.client.write(msg.payload, websocket.OPCode.text_frame) or { panic(err) }
		// 	}
		// }
	}, s)

	s.on_close(fn (mut ws websocket.Client, code int, reason string) ? {
		println(term.green('client ($ws.id) closed connection'))
	})
	s.listen() or { println(term.red('error on server listen: $err')) }
	unsafe {
		s.free()
	}
}
