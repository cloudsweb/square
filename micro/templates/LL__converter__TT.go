package LL__go_package__TT

import (
	"context"

	// -
	"google.golang.org/protobuf/runtime/protoimpl"
	// + {{#go_imports}}
	// + {{{ raku }}}
	// + {{/go_imports}}
)

// - DEFINE
type LL__go_orm_name__TT struct {
	LL__go_orm_name__TT string `gorm:"column:id;type:uuid;primaryKey" json:"id"`
}

type LL__go_proto_name__TT struct {
	state         protoimpl.MessageState
	sizeCache     protoimpl.SizeCache
	unknownFields protoimpl.UnknownFields

	LL__go_proto_name__TT string `protobuf:"bytes,2,opt,name=id,proto3" json:"id,omitempty"` // required
}

// - DEFINE

// + {{#structs}}
func (m *LL__go_proto_name__TT) ToORM(ctx context.Context) (LL__go_orm_name__TT, error) {
	to := LL__go_orm_name__TT{}
	var err error
	// + {{#fields}}

	// -
	to.LL__go_orm_name__TT = m.LL__go_proto_name__TT // {{{ types.go_orm }}} <- {{{ types.go_proto }}}
	// + {{{ go_convert_to_orm }}}
	// + {{/fields}}

	return to, err
}

func (m *LL__go_orm_name__TT) ToPB(ctx context.Context) (LL__go_proto_name__TT, error) {
	to := LL__go_proto_name__TT{}
	var err error
	// + {{#fields}}

	// -
	to.LL__go_proto_name__TT = m.LL__go_orm_name__TT // {{{ types.go_proto }}} <- {{{ types.go_orm }}}
	// + {{{ go_convert_to_pb }}}
	// + {{/fields}}

	return to, err
}

// + {{/structs}}
