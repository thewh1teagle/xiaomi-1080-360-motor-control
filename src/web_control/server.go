package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net"
	"net/http"
	"os"
	"strconv"
	"path/filepath"
	"github.com/gorilla/mux"
)

const FORWARD = 1
const REVERSE = 0
const PAN = 1
const TILT = 0
var MOTORD_FOLDER = ""
var EVENT_FILE = "event"

// spaHandler implements the http.Handler interface, so we can use it
// to respond to HTTP requests. The path to the static directory and
// path to the index file within that static directory are used to
// serve the SPA in the given static directory.
type spaHandler struct {
	staticPath string
	indexPath  string
}

// ServeHTTP inspects the URL path to locate a file within the static dir
// on the SPA handler. If a file is found, it will be served. If not, the
// file located at the index path on the SPA handler will be served. This
// is suitable behavior for serving an SPA (single page application).
func (h spaHandler) ServeHTTP(w http.ResponseWriter, r *http.Request) {
    // get the absolute path to prevent directory traversal
	path, err := filepath.Abs(r.URL.Path)
	if err != nil {
        // if we failed to get the absolute path respond with a 400 bad request
        // and stop
		http.Error(w, err.Error(), http.StatusBadRequest)
		return
	}

    // prepend the path with the path to the static directory
	path = filepath.Join(h.staticPath, path)

    // check whether a file exists at the given path
	_, err = os.Stat(path)
	if os.IsNotExist(err) {
		// file does not exist, serve index.html
		http.ServeFile(w, r, filepath.Join(h.staticPath, h.indexPath))
		return
	} else if err != nil {
        // if we got an error (that wasn't that the file doesn't exist) stating the
        // file, return a 500 internal server error and stop
		http.Error(w, err.Error(), http.StatusInternalServerError)
		return
	}

    // otherwise, use http.FileServer to serve the static dir
	http.FileServer(http.Dir(h.staticPath)).ServeHTTP(w, r)
}

func miio_motor_move(motor string, direction string, steps string) {
	f, err := os.Create(MOTORD_FOLDER + "/" + EVENT_FILE)
	if err != nil {
		fmt.Println(err)
		return
	}
	_, err = f.WriteString(motor + " " + direction + " " + steps)
	if err != nil {
		fmt.Println(err)
		f.Close()
		return
	}
	err = f.Close()
	if err != nil {
		fmt.Println(err)
		return
	}

}


func read_position_status() int {
	dat, err := ioutil.ReadFile(MOTORD_FOLDER + "/" + "status")
	check(err)
	s := string(dat[0])
	i, err := strconv.Atoi(s)
	return (i)
}

func check(e error) {
    if e != nil {
        panic(e)
    }
}

func miio_led_control(led int, value int) {
	/*
	echo 1 > /sys/class/gpio/gpio36/value
	echo 0 > /sys/class/gpio/gpio36/value
	echo 1 > /sys/class/gpio/gpio78/value
	echo 0 > /sys/class/gpio/gpio78/value
	*/

	var value_string = strconv.Itoa(value)
	var data = []byte(value_string)

	if led == 1 {
		err := ioutil.WriteFile("/sys/class/gpio/gpio36/value", data, 0644)
		check(err)
	} else {
		err := ioutil.WriteFile("/sys/class/gpio/gpio78/value", data, 0644)
		check(err)
	}
}


func led_control_route(w http.ResponseWriter, r *http.Request) { 
	params := mux.Vars(r)
	var led_num = 0
	var color = params["color"]
	var value = params["value"]

	
	if "yellow" == color {
		led_num = 1
	}

	if value == "on" {
		miio_led_control(led_num ,1)
	} else {
		miio_led_control(led_num ,0)
	}
	
}

func motor_move_route(w http.ResponseWriter, r *http.Request) {
	params := mux.Vars(r)
	var motor = params["motor"]
	var direction = params["direction"]
	var steps = params["steps"]
	miio_motor_move(motor, direction, steps)
	var status = read_position_status()
	if status == 0 {
		fmt.Fprintf(w, "ok")
	} else {
		fmt.Fprintf(w, "overflow")
	}
}

func get_local_ip() string {
	tt, err := net.Interfaces()
	check(err)
	for _, t := range tt {
		aa, err := t.Addrs()
		check(err)
		for _, a := range aa {
			ipnet, ok := a.(*net.IPNet)
			if !ok {
				continue
			}
			v4 := ipnet.IP.To4()
			if v4 == nil || v4[0] == 127 { // loopback address
				continue
			}
			return v4.String()
		}
	}
	return ""
}


func print_usage() {
	programName := os.Args[0]
	fmt.Println("Usage: ")
	fmt.Printf("%v --event_file <path> \n", programName)
}	

func validate_args() {
	argsWithoutProg := os.Args[1:]

	for index, value := range argsWithoutProg {
        if value == "--help" {
			print_usage()
			os.Exit(0)
		} else if value == "--motord_folder" {
			MOTORD_FOLDER = argsWithoutProg[index + 1]
		}
    }
}


func main() {

	validate_args()

	if _, err := os.Stat(MOTORD_FOLDER + "/" + EVENT_FILE); os.IsNotExist(err) {
		fmt.Printf("%v file not found! \n", EVENT_FILE)
		panic("event file not found!")
	}

	router := mux.NewRouter().StrictSlash(true)

	
	router.HandleFunc("/motor_move/{motor}/{direction}/{steps}", motor_move_route).Methods("GET")
	router.HandleFunc("/led_control/{color}/{value}", led_control_route).Methods("GET")

	spa := spaHandler{staticPath: "static", indexPath: "index.html"}
	router.PathPrefix("/").Handler(spa)

	fmt.Printf("Server started at http://%s:8080 \n", get_local_ip())
	log.Fatal(http.ListenAndServe(":8080", router))
}
