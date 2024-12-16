package main

import (
	"encoding/json"
	"fmt"
	"function/injector"
	"log"
	"os"
	"time"

	"github.com/nats-io/nats.go"
	"github.com/sirupsen/logrus"
)

var logger = logrus.New()

func setup() {

	logger.SetFormatter(&logrus.JSONFormatter{TimestampFormat: "2006-01-02T15:04:05.000Z07:00"})
	file, err := os.OpenFile("./logs.txt", os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0666)
	if err == nil {
		logger.SetOutput(file)
	} else {
		log.Fatal(err)
	}
}

func handler() {
	i := injector.NewInjector()
	aclService := i.GetServiceById("log")

	topic := aclService.ServiceTopic
	message := map[string]string{"timestamp": "2024-11-28T16:05:34", "message": "ciao", "severity": "info"}
	result, err := invokeFunction(topic, message)
	if err != nil {
		log.Fatal(err)
	}
	os.Stdout.WriteString("logging function result: " + result)
}

func invokeFunction(topic string, message map[string]string) (string, error) {
	nc, err := nats.Connect(nats.DefaultURL)
	if err != nil {
		return "error connecting to NATS", fmt.Errorf("error connecting to NATS: %w", err)
	}
	defer nc.Close()

	start := time.Now()
	msg, err := json.Marshal(message)
	if err != nil {
		return "error marshalling message", fmt.Errorf("error marshaling message: %w", err)
	}

	response, err := nc.Request(topic, msg, nats.DefaultTimeout)
	if err != nil {
		return "error publishing message", fmt.Errorf("error publishing message: %w", err)
	}
	end := time.Now()
	logger.Infof("logging function executed in %v ms", end.Sub(start).Milliseconds())

	/*
		var result string
		if err := json.Unmarshal(response.Data, &result); err != nil {
			return "failed to decode result", fmt.Errorf("error unmarshaling response: %w", err)
		}
	*/
	return string(response.Data), nil
}

func main() {
	setup()
	handler()
}
