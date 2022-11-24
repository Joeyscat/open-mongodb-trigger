package model

/*
  {
	"_id": {
	  "_data": "82637EF6BC000000032B022C0100296E5A10048A6B9C18252B44DB98AE98AFFA20168A46645F69640064637EF6B8AB3642F350795DD50004"
	},
	"operationType": "insert",
	"ns": {
	  "db": "db1",
	  "coll": "c1"
	},
	"to": null,
	"documentKey": {
	  "_id": {
		"$oid": "637ef6b8ab3642f350795dd5"
	  }
	},
	"updateDescription": null,
	"clusterTime": {
	  "$timestamp": {
		"t": 1669265084,
		"i": 3
	  }
	},
	"wallTime": null,
	"fullDocument": {
	  "_id": {
		"$oid": "637ef6b8ab3642f350795dd5"
	  },
	  "name": 1
	},
	"fullDocumentBeforeChange": null
  }

    "updateDescription": {
    "updatedFields": {
      "name": 3
    },
    "removedFields": [],
    "truncatedArrays": null
  },
*/

//go:generate go run github.com/json-iterator/tinygo/gen
type ChangeStreamEvent struct {
	Id            ResumeToken     `json:"_id"`
	OperationType string          `json:"operationType"`
	Ns            ChangeNamespace `json:"ns"`
	To            ChangeNamespace `json:"to"`
	DocumentKey   struct {
		Id struct {
			Oid string `json:"$oid"`
		} `json:"_id"`
	} `json:"documentKey"`
	UpdateDescription struct {
		UpdatedFields struct {
		} `json:"updatedFields"`
		RemovedFields []string `json:"removedFields"`
	} `json:"updateDescription"`
	ClusterTime struct {
		Timestamp struct {
			T uint `json:"t"`
			I uint `json:"i"`
		} `json:"$timestamp"`
	} `json:"clusterTime"`
	FullDocument struct {
	} `json:"fullDocument"`
}

//go:generate go run github.com/json-iterator/tinygo/gen
type ResumeToken struct {
	Data string `json:"_data"`
}

//go:generate go run github.com/json-iterator/tinygo/gen
type ChangeNamespace struct {
	DB   string `json:"db"`
	Coll string `json:"coll"`
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
