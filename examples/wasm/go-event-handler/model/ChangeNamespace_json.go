package model

import jsoniter "github.com/json-iterator/tinygo"

type ChangeNamespace_json struct {
}
func (json ChangeNamespace_json) Type() interface{} {
  var val ChangeNamespace
  return val
}
func (json ChangeNamespace_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  ChangeNamespace_json_unmarshal(iter, out.(*ChangeNamespace))
}
func (json ChangeNamespace_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  ChangeNamespace_json_marshal(stream, val.(ChangeNamespace))
}
func ChangeNamespace_json_unmarshal(iter *jsoniter.Iterator, out *ChangeNamespace) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeNamespace_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeNamespace_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *ChangeNamespace) bool {
  switch {
  case field == `db`:
    iter.ReadString(&(*out).DB)
    return true
  case field == `coll`:
    iter.ReadString(&(*out).Coll)
    return true
  }
  return false
}
func ChangeNamespace_json_marshal(stream *jsoniter.Stream, val ChangeNamespace) {
    stream.WriteObjectHead()
    ChangeNamespace_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeNamespace_json_marshal_field(stream *jsoniter.Stream, val ChangeNamespace) {
    stream.WriteObjectField(`db`)
    stream.WriteString(val.DB)
    stream.WriteMore()
    stream.WriteObjectField(`coll`)
    stream.WriteString(val.Coll)
    stream.WriteMore()
}
