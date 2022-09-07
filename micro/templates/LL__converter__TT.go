package LL__go_package__TT

import (
	"context"

	// -
	"google.golang.org/protobuf/runtime/protoimpl"
)

// - DEFINE
type LL__struct_orig_name__TT struct {
	LL__go_name__TT string `gorm:"column:id;type:uuid;primaryKey" json:"id"`
}

type LL__struct_name__TT struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	LL__proto_go_name__TT string `protobuf:"bytes,2,opt,name=id,proto3" json:"id,omitempty"` // required
}

// - DEFINE

// + {{#structs}}
func (m *LL__struct_name__TT) ToORM(ctx context.Context) (LL__struct_orig_name__TT, error) {
	to := LL__struct_orig_name__TT{}
	var err error
	return to, err
}

// + {{/structs}}
