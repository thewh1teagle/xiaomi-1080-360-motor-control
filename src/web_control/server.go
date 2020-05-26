package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"net"
	"net/http"
	"os"
	"strconv"

	"github.com/gorilla/mux"
)

const FORWARD = 1
const REVERSE = 0
const PAN = 1
const TILT = 0

func miio_motor_move(motor string, direction string, steps string) {
	f, err := os.Create("event")
	if err != nil {
		fmt.Println(err)
		return
	}
	l, err := f.WriteString(motor + " " + direction + " " + steps)
	fmt.Println(l, "bytes written successfully")
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
	dat, err := ioutil.ReadFile("status")
	if err != nil {
		panic(err)
	}
	s := string(dat[0])
	i, err := strconv.Atoi(s)
	fmt.Printf("status: %v \n", s)
	fmt.Printf("status: %d \n", i)
	return (i)
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
		fmt.Fprintf(w, "max")
	}
}

func get_local_ip() string {
	tt, err := net.Interfaces()
	if err != nil {
		panic(err)
	}
	for _, t := range tt {
		aa, err := t.Addrs()
		if err != nil {
			panic(err)
		}
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

func main() {
	router := mux.NewRouter().StrictSlash(true)
	router.HandleFunc("/motor_move/{motor}/{direction}/{steps}", motor_move_route).Methods("GET")
	fmt.Printf("Server started at http://%s:8080 \n", get_local_ip())
	log.Fatal(http.ListenAndServe(":8080", router))
}
