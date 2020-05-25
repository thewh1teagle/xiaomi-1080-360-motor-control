package main

import (
	"fmt"
	"io/ioutil"
	"log"
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

func main() {
	router := mux.NewRouter().StrictSlash(true)
	router.HandleFunc("/motor_move/{motor}/{direction}/{steps}", motor_move_route).Methods("GET")
	fmt.Println("Server started at port 8080")
	log.Fatal(http.ListenAndServe(":8080", router))
}
