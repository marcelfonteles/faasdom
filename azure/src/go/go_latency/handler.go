package main

import (
    "encoding/json"
    "log"
    "net/http"
    "os"
)

type payload struct {
    Test string `json:"test"`
}

type response struct {
    Success bool `json:"success"`
    Payload payload `json:"payload"`
}



func Go_latency(w http.ResponseWriter, r *http.Request) {
    data := response{ 
        Success: true,  
        Payload: payload{
            Test: "latency test",
        },
    }
    w.Header().Set("Content-Type", "application/json")
    w.WriteHeader(http.StatusCreated)
    json.NewEncoder(w).Encode(data)
}

func main() {
    listenAddr := ":8080"
    if val, ok := os.LookupEnv("FUNCTIONS_CUSTOMHANDLER_PORT"); ok {
        listenAddr = ":" + val
    }
    http.HandleFunc("/api/latency", Go_latency)
    log.Printf("About to listen on %s. Go to https://127.0.0.1%s/", listenAddr, listenAddr)
    log.Fatal(http.ListenAndServe(listenAddr, nil))
}