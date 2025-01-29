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
var (
	natsURL = getEnv("NATSSERVER", "nats://192.168.17.118:4222")
)

func getEnv(key, defaultValue string) string {
	if value, exists := os.LookupEnv(key); exists {
		return value
	}
	return defaultValue
}

func setup() {
	logger.SetFormatter(&logrus.JSONFormatter{TimestampFormat: "2006-01-02T15:04:05.000Z07:00"})
	file, err := os.OpenFile("logs.txt", os.O_CREATE|os.O_WRONLY|os.O_APPEND, 0666)
	if err == nil {
		logger.SetOutput(file)
	} else {
		log.Fatal(err)
	}

}

func handler() {
	i := injector.NewInjector()
	aclService := i.GetServiceById("acl")

	topic := aclService.ServiceTopic
	message := map[string]string{"user": "Bob", "permission": "read"}
	result, err := invokeFunction(topic, message)
	if err != nil {
		log.Fatal(err)
	}
	os.Stdout.WriteString("access control result: " + result)
}

func invokeFunction(topic string, message map[string]string) (string, error) {

	nc, err := nats.Connect(natsURL)
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
	logger.Infof("access control function executed in %v ms", end.Sub(start).Milliseconds())

	var result bool
	if err := json.Unmarshal(response.Data, &result); err != nil {
		return "failed to decode result", fmt.Errorf("error unmarshaling response: %w", err)
	}
	return "access granted", nil
}

func main() {
	setup()
	handler()
}
