module main
import vweb

const (
	port = 8082
)

struct App {
	vweb.Context
mut:
	state shared State
}

struct State {
mut:
	cnt int
}

fn main() {
	println('web server')
	vweb.run(&App{}, port)
}

pub fn (mut app App) index() vweb.Result {
	lock app.state {
		app.state.cnt++
	}
	return $vweb.html()
}


// func miio_led_control(led int, value int) {
// 	/*
// 		echo 1 > /sys/class/gpio/gpio36/value
// 		echo 0 > /sys/class/gpio/gpio36/value
// 		echo 1 > /sys/class/gpio/gpio78/value
// 		echo 0 > /sys/class/gpio/gpio78/value
// 	*/

// 	var value_string = strconv.Itoa(value)
// 	var data = []byte(value_string)

// 	if led == 1 {
// 		err := ioutil.WriteFile("/sys/class/gpio/gpio36/value", data, 0644)
// 		check(err)
// 	} else {
// 		err := ioutil.WriteFile("/sys/class/gpio/gpio78/value", data, 0644)
// 		check(err)
// 	}
// }