package main

import (
	"bufio"
	"encoding/json"
	"os"
)

func handler() {

	//preparing the message to be sent to the acl service
	message := map[string]string{"message": "{\"user\": \"Bob\", \"permission\": \"read\"}"}
	msg, err := json.Marshal(message)
	if err != nil {
		return
	}

	//invoke the acl service
	os.Stdout.Write(msg)

	//reading the result
	reader := bufio.NewReader(os.Stdin)
	result, _ := reader.ReadString('\n')
	//returning the result
	os.Stdout.WriteString("access control result: " + result)
}

func main() {
	handler()
}
