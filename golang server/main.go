package main

import (
	"fmt"
	"log"
	"net/http"
	"os/exec"

	"github.com/gorilla/mux"
)

func homeLink(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "motor control")
}

func motor_move(w http.ResponseWriter, r *http.Request) {
	params := mux.Vars(r)
	var direction = params["direction"]
	var steps = params["steps"]

	out, err := exec.Command("./motor", direction, steps).Output()
	if err != nil {
		log.Fatal(err)
	}
	fmt.Printf("%s", out)
	fmt.Fprintf(w, "Direction: %v \nSteps: %v", direction, steps)
}

func main() {
	router := mux.NewRouter().StrictSlash(true)
	router.HandleFunc("/", homeLink).Methods("GET")
	router.HandleFunc("/motor_move/{direction}/{steps}", motor_move).Methods("GET")
	log.Fatal(http.ListenAndServe(":8080", router))
}
