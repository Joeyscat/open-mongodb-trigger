package model

import jsoniter "github.com/json-iterator/tinygo"

type ChangeStreamEvent_json struct {
}
func (json ChangeStreamEvent_json) Type() interface{} {
  var val ChangeStreamEvent
  return val
}
func (json ChangeStreamEvent_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  ChangeStreamEvent_json_unmarshal(iter, out.(*ChangeStreamEvent))
}
func (json ChangeStreamEvent_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  ChangeStreamEvent_json_marshal(stream, val.(ChangeStreamEvent))
}
func ChangeStreamEvent_json_unmarshal(iter *jsoniter.Iterator, out *ChangeStreamEvent) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *ChangeStreamEvent) bool {
  switch {
  case field == `_id`:
    ResumeToken_json_unmarshal(iter, &(*out).Id)
    return true
  case field == `operationType`:
    iter.ReadString(&(*out).OperationType)
    return true
  case field == `ns`:
    ChangeNamespace_json_unmarshal(iter, &(*out).Ns)
    return true
  case field == `to`:
    ChangeNamespace_json_unmarshal(iter, &(*out).To)
    return true
  case field == `documentKey`:
    ChangeStreamEvent_struct1_json_unmarshal(iter, &(*out).DocumentKey)
    return true
  case field == `updateDescription`:
    ChangeStreamEvent_struct3_json_unmarshal(iter, &(*out).UpdateDescription)
    return true
  case field == `clusterTime`:
    ChangeStreamEvent_struct6_json_unmarshal(iter, &(*out).ClusterTime)
    return true
  case field == `fullDocument`:
    ChangeStreamEvent_struct8_json_unmarshal(iter, &(*out).FullDocument)
    return true
  }
  return false
}
func ChangeStreamEvent_struct2_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
	Oid string `json:"$oid"`
}) bool {
  switch {
  case field == `$oid`:
    iter.ReadString(&(*out).Oid)
    return true
  }
  return false
}
func ChangeStreamEvent_struct2_json_unmarshal (iter *jsoniter.Iterator, out *struct {
	Oid string `json:"$oid"`
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct2_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_struct1_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
	Id struct {
		Oid string `json:"$oid"`
	} `json:"_id"`
}) bool {
  switch {
  case field == `_id`:
    ChangeStreamEvent_struct2_json_unmarshal(iter, &(*out).Id)
    return true
  }
  return false
}
func ChangeStreamEvent_struct1_json_unmarshal (iter *jsoniter.Iterator, out *struct {
	Id struct {
		Oid string `json:"$oid"`
	} `json:"_id"`
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct1_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_struct4_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
}) bool {
  return false
}
func ChangeStreamEvent_struct4_json_unmarshal (iter *jsoniter.Iterator, out *struct {
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct4_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_array5_json_unmarshal (iter *jsoniter.Iterator, out *[]string) {
  i := 0
  val := *out
  more := iter.ReadArrayHead()
  for more {
    if i == len(val) {
      val = append(val, make([]string, 4)...)
    }
    iter.ReadString(&val[i])
    i++
    more = iter.ReadArrayMore()
  }
  if i == 0 {
    *out = []string{}
  } else {
    *out = val[:i]
  }
}
func ChangeStreamEvent_struct3_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
	UpdatedFields	struct {
	}	`json:"updatedFields"`
	RemovedFields	[]string	`json:"removedFields"`
}) bool {
  switch {
  case field == `updatedFields`:
    ChangeStreamEvent_struct4_json_unmarshal(iter, &(*out).UpdatedFields)
    return true
  case field == `removedFields`:
    ChangeStreamEvent_array5_json_unmarshal(iter, &(*out).RemovedFields)
    return true
  }
  return false
}
func ChangeStreamEvent_struct3_json_unmarshal (iter *jsoniter.Iterator, out *struct {
	UpdatedFields	struct {
	}	`json:"updatedFields"`
	RemovedFields	[]string	`json:"removedFields"`
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct3_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_struct7_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
	T	uint	`json:"t"`
	I	uint	`json:"i"`
}) bool {
  switch {
  case field == `t`:
    iter.ReadUint(&(*out).T)
    return true
  case field == `i`:
    iter.ReadUint(&(*out).I)
    return true
  }
  return false
}
func ChangeStreamEvent_struct7_json_unmarshal (iter *jsoniter.Iterator, out *struct {
	T	uint	`json:"t"`
	I	uint	`json:"i"`
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct7_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_struct6_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
	Timestamp struct {
		T	uint	`json:"t"`
		I	uint	`json:"i"`
	} `json:"$timestamp"`
}) bool {
  switch {
  case field == `$timestamp`:
    ChangeStreamEvent_struct7_json_unmarshal(iter, &(*out).Timestamp)
    return true
  }
  return false
}
func ChangeStreamEvent_struct6_json_unmarshal (iter *jsoniter.Iterator, out *struct {
	Timestamp struct {
		T	uint	`json:"t"`
		I	uint	`json:"i"`
	} `json:"$timestamp"`
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct6_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_struct8_json_unmarshal_field (iter *jsoniter.Iterator, field string, out *struct {
}) bool {
  return false
}
func ChangeStreamEvent_struct8_json_unmarshal (iter *jsoniter.Iterator, out *struct {
}) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !ChangeStreamEvent_struct8_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func ChangeStreamEvent_json_marshal(stream *jsoniter.Stream, val ChangeStreamEvent) {
    stream.WriteObjectHead()
    ChangeStreamEvent_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_json_marshal_field(stream *jsoniter.Stream, val ChangeStreamEvent) {
    stream.WriteObjectField(`_id`)
    ResumeToken_json_marshal(stream, val.Id)
    stream.WriteMore()
    stream.WriteObjectField(`operationType`)
    stream.WriteString(val.OperationType)
    stream.WriteMore()
    stream.WriteObjectField(`ns`)
    ChangeNamespace_json_marshal(stream, val.Ns)
    stream.WriteMore()
    stream.WriteObjectField(`to`)
    ChangeNamespace_json_marshal(stream, val.To)
    stream.WriteMore()
    stream.WriteObjectField(`documentKey`)
    ChangeStreamEvent_struct9_json_marshal(stream, val.DocumentKey)
    stream.WriteMore()
    stream.WriteObjectField(`updateDescription`)
    ChangeStreamEvent_struct11_json_marshal(stream, val.UpdateDescription)
    stream.WriteMore()
    stream.WriteObjectField(`clusterTime`)
    ChangeStreamEvent_struct14_json_marshal(stream, val.ClusterTime)
    stream.WriteMore()
    stream.WriteObjectField(`fullDocument`)
    ChangeStreamEvent_struct16_json_marshal(stream, val.FullDocument)
    stream.WriteMore()
}
func ChangeStreamEvent_struct10_json_marshal_field (stream *jsoniter.Stream, val struct {
	Oid string `json:"$oid"`
}) {
    stream.WriteObjectField(`$oid`)
    stream.WriteString(val.Oid)
    stream.WriteMore()
}
func ChangeStreamEvent_struct10_json_marshal (stream *jsoniter.Stream, val struct {
	Oid string `json:"$oid"`
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct10_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_struct9_json_marshal_field (stream *jsoniter.Stream, val struct {
	Id struct {
		Oid string `json:"$oid"`
	} `json:"_id"`
}) {
    stream.WriteObjectField(`_id`)
    ChangeStreamEvent_struct10_json_marshal(stream, val.Id)
    stream.WriteMore()
}
func ChangeStreamEvent_struct9_json_marshal (stream *jsoniter.Stream, val struct {
	Id struct {
		Oid string `json:"$oid"`
	} `json:"_id"`
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct9_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_struct12_json_marshal_field (stream *jsoniter.Stream, val struct {
}) {
}
func ChangeStreamEvent_struct12_json_marshal (stream *jsoniter.Stream, val struct {
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct12_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_array13_json_marshal (stream *jsoniter.Stream, val []string) {
  if len(val) == 0 {
    stream.WriteEmptyArray()
  } else {
    stream.WriteArrayHead()
    for i, elem := range val {
      if i != 0 { stream.WriteMore() }
    stream.WriteString(elem)
    }
    stream.WriteArrayTail()
  }
}
func ChangeStreamEvent_struct11_json_marshal_field (stream *jsoniter.Stream, val struct {
	UpdatedFields	struct {
	}	`json:"updatedFields"`
	RemovedFields	[]string	`json:"removedFields"`
}) {
    stream.WriteObjectField(`updatedFields`)
    ChangeStreamEvent_struct12_json_marshal(stream, val.UpdatedFields)
    stream.WriteMore()
    stream.WriteObjectField(`removedFields`)
    ChangeStreamEvent_array13_json_marshal(stream, val.RemovedFields)
    stream.WriteMore()
}
func ChangeStreamEvent_struct11_json_marshal (stream *jsoniter.Stream, val struct {
	UpdatedFields	struct {
	}	`json:"updatedFields"`
	RemovedFields	[]string	`json:"removedFields"`
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct11_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_struct15_json_marshal_field (stream *jsoniter.Stream, val struct {
	T	uint	`json:"t"`
	I	uint	`json:"i"`
}) {
    stream.WriteObjectField(`t`)
    stream.WriteUint(val.T)
    stream.WriteMore()
    stream.WriteObjectField(`i`)
    stream.WriteUint(val.I)
    stream.WriteMore()
}
func ChangeStreamEvent_struct15_json_marshal (stream *jsoniter.Stream, val struct {
	T	uint	`json:"t"`
	I	uint	`json:"i"`
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct15_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_struct14_json_marshal_field (stream *jsoniter.Stream, val struct {
	Timestamp struct {
		T	uint	`json:"t"`
		I	uint	`json:"i"`
	} `json:"$timestamp"`
}) {
    stream.WriteObjectField(`$timestamp`)
    ChangeStreamEvent_struct15_json_marshal(stream, val.Timestamp)
    stream.WriteMore()
}
func ChangeStreamEvent_struct14_json_marshal (stream *jsoniter.Stream, val struct {
	Timestamp struct {
		T	uint	`json:"t"`
		I	uint	`json:"i"`
	} `json:"$timestamp"`
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct14_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func ChangeStreamEvent_struct16_json_marshal_field (stream *jsoniter.Stream, val struct {
}) {
}
func ChangeStreamEvent_struct16_json_marshal (stream *jsoniter.Stream, val struct {
}) {
    stream.WriteObjectHead()
    ChangeStreamEvent_struct16_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
