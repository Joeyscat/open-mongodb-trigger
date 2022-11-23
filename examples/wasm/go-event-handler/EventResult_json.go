package main

import jsoniter "github.com/json-iterator/tinygo"

type EventResult_json struct {
}
func (json EventResult_json) Type() interface{} {
  var val EventResult
  return val
}
func (json EventResult_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  EventResult_json_unmarshal(iter, out.(*EventResult))
}
func (json EventResult_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  EventResult_json_marshal(stream, val.(EventResult))
}
func EventResult_json_unmarshal(iter *jsoniter.Iterator, out *EventResult) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !EventResult_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func EventResult_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *EventResult) bool {
  switch {
  case field == `ok`:
    iter.ReadBool(&(*out).Ok)
    return true
  case field == `msg`:
    iter.ReadString(&(*out).Msg)
    return true
  }
  return false
}
func EventResult_json_marshal(stream *jsoniter.Stream, val EventResult) {
    stream.WriteObjectHead()
    EventResult_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func EventResult_json_marshal_field(stream *jsoniter.Stream, val EventResult) {
    stream.WriteObjectField(`ok`)
    stream.WriteBool(val.Ok)
    stream.WriteMore()
    stream.WriteObjectField(`msg`)
    stream.WriteString(val.Msg)
    stream.WriteMore()
}
