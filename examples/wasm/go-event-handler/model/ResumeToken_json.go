package model

import jsoniter "github.com/json-iterator/tinygo"

type ResumeToken_json struct {
}
func (json ResumeToken_json) Type() interface{} {
  var val ResumeToken
  return val
}
func (json ResumeToken_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  ResumeToken_json_unmarshal(iter, out.(*ResumeToken))
}
func (json ResumeToken_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  ResumeToken_json_marshal(stream, val.(ResumeToken))
}
func ResumeToken_json_unmarshal(iter *jsoniter.Iterator, out *ResumeToken) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ResumeToken_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ResumeToken_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *ResumeToken) bool {
  switch {
  case field == `_data`:
    iter.ReadString(&(*out).Data)
    return true
  }
  return false
}
func ResumeToken_json_marshal(stream *jsoniter.Stream, val ResumeToken) {
    stream.WriteObjectHead()
    ResumeToken_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ResumeToken_json_marshal_field(stream *jsoniter.Stream, val ResumeToken) {
    stream.WriteObjectField(`_data`)
    stream.WriteString(val.Data)
    stream.WriteMore()
}
