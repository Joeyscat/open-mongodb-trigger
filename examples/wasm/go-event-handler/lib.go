// lib.go
package main

import (
	"errors"
	"fmt"

	"event-handler/model"
)

// implemented by the user
func handlerEvent(event model.ChangeStreamEvent) error {
	// fmt.Printf("event: %#+v\n", event)
	data, err := json.MarshalIndent(event, "", "\t")
	if err != nil {
		return err
	}
	fmt.Printf("event:\n%s\n", string(data))

	if event.OperationType == "delete" {
		return errors.New("unsuppored op_type: Delete")
	}

	return nil
}
