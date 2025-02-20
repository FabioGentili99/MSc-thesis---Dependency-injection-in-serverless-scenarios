package main

import (
	"encoding/json"
	"fmt"
	"os"
	"strings"
	"time"
)

func logMessageToFile(timestamp, message, severity string) bool {
	validSeverities := []string{"info", "warn", "error"}

	// Validate severity level
	if !contains(validSeverities, severity) {
		fmt.Println("Invalid severity level. Valid options are: info, warn, error.")
		return false
	}

	// Validate timestamp
	_, err := time.Parse(time.RFC3339, timestamp)
	if err != nil {
		fmt.Println("Invalid timestamp format. Use ISO 8601 format (e.g., yyyy-mm-ddThh:mm:ssZ).")
		return false
	}

	logEntry := fmt.Sprintf("[%s] [%s]: %s\n", timestamp, strings.ToUpper(severity), message)

	// Append the log entry to the logs.txt file
	err = appendToFile("logs.txt", logEntry)
	if err != nil {
		fmt.Println("Logging operation failed:", err.Error())
		return false
	}
	return true // Logging succeeded
}

func contains(slice []string, item string) bool {
	for _, v := range slice {
		if v == item {
			return true
		}
	}
	return false
}

func appendToFile(filename, data string) error {
	f, err := os.OpenFile(filename, os.O_APPEND|os.O_CREATE|os.O_WRONLY, 0644)
	if err != nil {
		return err
	}
	defer f.Close()

	if _, err := f.WriteString(data); err != nil {
		return err
	}
	return nil
}

func main() {
	var params struct {
		Timestamp string `json:"timestamp"`
		Message   string `json:"message"`
		Severity  string `json:"severity"`
	}

	// Get input from the command line
	if len(os.Args) < 2 {
		fmt.Println("Invalid input. Please provide the correct number of arguments.")
		os.Exit(1)
	}

	err := json.Unmarshal([]byte(os.Args[1]), &params)
	if err != nil {
		fmt.Println("Invalid input. Please provide the correct number of arguments.")
		os.Exit(1)
	}

	// Perform the logging operation
	result := logMessageToFile(params.Timestamp, params.Message, params.Severity)
	// Indicate success or failure
	if result {
		os.Stdout.WriteString("logging operation succeeded")
	} else {
		os.Stdout.WriteString("logging operation failed.")
	}
}
