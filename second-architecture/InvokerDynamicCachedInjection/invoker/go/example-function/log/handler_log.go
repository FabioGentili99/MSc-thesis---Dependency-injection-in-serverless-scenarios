package main

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"log"
	"net/http"
	"os"
	"time"

	"github.com/sirupsen/logrus"
)

var logger = logrus.New()

// Custom CSV Formatter
type CSVFormatter struct{}

func (f *CSVFormatter) Format(entry *logrus.Entry) ([]byte, error) {
	// Convert timestamp to ISO 8601 format
	timestamp := entry.Time.UTC().Format("2006-01-02T15:04:05.000Z")
	// Format the log as CSV: timestamp,level,logger,message
	logMsg := fmt.Sprintf("%s,%s,%s\n",
		timestamp, entry.Level.String(), entry.Message)
	return []byte(logMsg), nil
}

func setup() {

	logger.SetFormatter(&logrus.JSONFormatter{TimestampFormat: "2006-01-02T15:04:05.000Z07:00"})
	file, err := os.OpenFile("./logs.txt", os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0666)
	if err == nil {
		logger.SetOutput(file)
		logger.SetLevel(logrus.InfoLevel)    // Log level
		logger.SetFormatter(&CSVFormatter{}) // Use custom CSV formatter
	} else {
		log.Fatal(err)
	}
}

func handler() {
	//retrieve injected service. It is stored in the environment variable SERVICE
	logService, exists := os.LookupEnv("SERVICE")
	if !exists {
		os.Stdout.WriteString("SERVICE environment variable not set")
	}

	//preparing the message to be sent to the acl service
	message := map[string]string{"message": "{\"timestamp\": \"2024-11-28T16:05:34Z\", \"message\": \"ciao\", \"severity\": \"info\"}"}

	//invoke the logging service
	result, err := invokeFunction(logService, message)

	if err != nil {
		log.Fatal(err)
	}
	os.Stdout.WriteString("logging function result: " + result)
}

func invokeFunction(address string, message map[string]string) (string, error) {

	msg, err := json.Marshal(message)
	if err != nil {
		return "error marshalling message", fmt.Errorf("error marshaling message: %w", err)
	}
	start := time.Now()

	req, err := http.NewRequest("POST", address, bytes.NewBuffer(msg))
	if err != nil {
		fmt.Println("Error creating request:", err)
		return "error creating http request", fmt.Errorf("error creating http request: %w", err)
	}

	req.Header.Set("Content-Type", "application/json")
	client := &http.Client{}
	response, err := client.Do(req)
	if err != nil {
		fmt.Println("Error sending request:", err)
		return "error sending http request", fmt.Errorf("error sending http request: %w", err)
	}
	defer response.Body.Close()

	end := time.Now()
	logger.Infof("logging function executed in %v ms", end.Sub(start).Milliseconds())

	result, err := io.ReadAll(response.Body)
	if err != nil {
		fmt.Println("Error reading response:", err)
		return "error reading response", fmt.Errorf("error reading response: %w", err)
	}

	return string(result), nil
}

func main() {
	setup()
	handler()
}
