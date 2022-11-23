package main

import jsoniter "github.com/json-iterator/tinygo"

type Event_json struct {
}
func (json Event_json) Type() interface{} {
  var val Event
  return val
}
func (json Event_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  Event_json_unmarshal(iter, out.(*Event))
}
func (json Event_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  Event_json_marshal(stream, val.(Event))
}
func Event_json_unmarshal(iter *jsoniter.Iterator, out *Event) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !Event_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func Event_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *Event) bool {
  switch {
  case field == `id`:
    iter.ReadInt64(&(*out).Id)
    return true
  case field == `operationType`:
    iter.ReadString(&(*out).OperationType)
    return true
  }
  return false
}
func Event_json_marshal(stream *jsoniter.Stream, val Event) {
    stream.WriteObjectHead()
    Event_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func Event_json_marshal_field(stream *jsoniter.Stream, val Event) {
    stream.WriteObjectField(`id`)
    stream.WriteInt64(val.Id)
    stream.WriteMore()
    stream.WriteObjectField(`operationType`)
    stream.WriteString(val.OperationType)
    stream.WriteMore()
}
