// lib.go
package main

import (
	"encoding/binary"
	"errors"
	"fmt"
	"unsafe"

	jsoniter "github.com/json-iterator/tinygo"
)

func main() {

}

//export event_handler_entry
func EventHandlerEntry(p *uint32) uint32 {
	numbytes := *(*[4]byte)(unsafe.Pointer(p))
	num := binary.LittleEndian.Uint32(numbytes[:])

	dataPointer := unsafe.Add(unsafe.Pointer(p), 4)
	data := unsafe.Slice((*byte)(dataPointer), num)

	json := jsoniter.CreateJsonAdapter(EventResult_json{}, Event_json{})

	var event Event
	err := json.Unmarshal(data, &event)
	if err != nil {
		panic(err)
	}

	//
	err = handlerEvent(event)

	var result EventResult
	if err != nil {
		result = Error(err.Error())
	} else {
		result = Ok()
	}

	data, err = json.Marshal(result)
	if err != nil {
		panic(err)
	}

	num = uint32(len(data))
	arr := make([]byte, num+4)
	binary.LittleEndian.PutUint32(arr, num)
	for i := 0; i < int(num); i++ {
		arr[i+4] = data[i]
	}

	resultPtr := *(*uint32)(unsafe.Pointer(&arr))

	return resultPtr
}

// implemented by the user
func handlerEvent(event Event) error {
	fmt.Printf("event: %#+v\n", event)

	if event.OperationType == "delete" {
		return errors.New("unsuppored op_type: Delete")
	}

	return nil
}
