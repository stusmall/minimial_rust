package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"math/rand"
	"net/http"
	"os"
	"slices"
	"time"
)

func main() {
	fmt.Println("Starting integration test...")
	hostname, present := os.LookupEnv("HOSTNAME")
	if present == false {
		print("HOSTNAME is not set")
		os.Exit(1)
	}

	const characterSet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
	rng := rand.New(rand.NewSource(time.Now().UnixNano()))
	b := make([]byte, 10)
	for i := range b {
		b[i] = characterSet[rng.Intn(len(characterSet))]
	}
	message := string(b)

	fmt.Println("Adding message...")
	values := map[string]string{"message": message}
	jsonValue, _ := json.Marshal(values)

	url := fmt.Sprintf("http://%s:8080/api/v1/message", hostname)
	resp, err := http.Post(url, "application/json", bytes.NewBuffer(jsonValue))
	if err != nil || resp.StatusCode != http.StatusOK {
		fmt.Println("Failed to add new message: ", err, resp.StatusCode)
		os.Exit(1)
	}

	fmt.Println("Reading message back...")
	resp, err = http.Get(url)
	if err != nil || resp.StatusCode != http.StatusOK {
		fmt.Println("Failed to get message: ", err, resp.StatusCode)
		os.Exit(1)
	}

	var data map[string][]string
	err = json.NewDecoder(resp.Body).Decode(&data)

	if err != nil {
		fmt.Println("Failed to deserialize: ", err)
		os.Exit(1)
	}
	if slices.Contains(data["messages"], message) {
		fmt.Println("Test passed!")
	} else {
		fmt.Println("Our value wasn't returned!")
		os.Exit(1)
	}
}
