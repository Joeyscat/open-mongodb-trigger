// DO NOT EDIT
//
// entry.go
package main

import (
	"encoding/binary"
	"unsafe"

	"event-handler/model"

	jsoniter "github.com/json-iterator/tinygo"
)

func main() {

}

var json jsoniter.JsonAdapter

//export event_handler_entry
func EventHandlerEntry(p *uint32) uint32 {
	numbytes := *(*[4]byte)(unsafe.Pointer(p))
	num := binary.LittleEndian.Uint32(numbytes[:])

	dataPointer := unsafe.Add(unsafe.Pointer(p), 4)
	data := unsafe.Slice((*byte)(dataPointer), num)

	json = jsoniter.CreateJsonAdapter(model.EventResult_json{}, model.ChangeStreamEvent_json{})

	var event model.ChangeStreamEvent
	err := json.Unmarshal(data, &event)
	if err != nil {
		panic(err)
	}

	//
	err = handlerEvent(event)

	var result model.EventResult
	if err != nil {
		result = model.Error(err.Error())
	} else {
		result = model.Ok()
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
