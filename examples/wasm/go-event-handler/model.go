package main

//go:generate go run github.com/json-iterator/tinygo/gen
type Event struct {
	Id            int64  `json:"id"`
	OperationType string `json:"operationType"`
}

//go:generate go run github.com/json-iterator/tinygo/gen
type EventResult struct {
	Ok  bool   `json:"ok"`
	Msg string `json:"msg"`
}

func Ok() EventResult {
	return EventResult{
		Ok:  true,
		Msg: "",
	}
}

func Error(msg string) EventResult {
	return EventResult{
		Ok:  false,
		Msg: msg,
	}
}
