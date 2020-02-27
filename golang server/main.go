package main

import (
	"fmt"
	"log"
	"net/http"

	"github.com/gorilla/mux"
)

func homeLink(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "motor status: ")
}

func left(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "left")
}

func right(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "right")
}

func up(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "up")
}

func down(w http.ResponseWriter, r *http.Request) {
	fmt.Fprintf(w, "down")
}

func main() {
	router := mux.NewRouter().StrictSlash(true)
	router.HandleFunc("/", homeLink)
	router.HandleFunc("/left", left)
	router.HandleFunc("/right", right)
	router.HandleFunc("/up", up)
	router.HandleFunc("/down", down)
	log.Fatal(http.ListenAndServe(":8080", router))
}
