// Code generated by protoc-gen-gogo. DO NOT EDIT.
// source: tokenbridge/coin_meta_rollback_protection.proto

package types

import (
	fmt "fmt"
	io "io"
	math "math"
	math_bits "math/bits"

	proto "github.com/gogo/protobuf/proto"
)

// Reference imports to suppress errors if they are not otherwise used.
var _ = proto.Marshal
var _ = fmt.Errorf
var _ = math.Inf

// This is a compile-time assertion to ensure that this generated file
// is compatible with the proto package it is being compiled against.
// A compilation error at this line likely means your copy of the
// proto package needs to be updated.
const _ = proto.GoGoProtoPackageIsVersion3 // please upgrade the proto package

type CoinMetaRollbackProtection struct {
	Index              string `protobuf:"bytes,1,opt,name=index,proto3" json:"index,omitempty"`
	LastUpdateSequence uint64 `protobuf:"varint,2,opt,name=lastUpdateSequence,proto3" json:"lastUpdateSequence,omitempty"`
}

func (m *CoinMetaRollbackProtection) Reset()         { *m = CoinMetaRollbackProtection{} }
func (m *CoinMetaRollbackProtection) String() string { return proto.CompactTextString(m) }
func (*CoinMetaRollbackProtection) ProtoMessage()    {}
func (*CoinMetaRollbackProtection) Descriptor() ([]byte, []int) {
	return fileDescriptor_23ec5ccab8f2b4ca, []int{0}
}
func (m *CoinMetaRollbackProtection) XXX_Unmarshal(b []byte) error {
	return m.Unmarshal(b)
}
func (m *CoinMetaRollbackProtection) XXX_Marshal(b []byte, deterministic bool) ([]byte, error) {
	if deterministic {
		return xxx_messageInfo_CoinMetaRollbackProtection.Marshal(b, m, deterministic)
	} else {
		b = b[:cap(b)]
		n, err := m.MarshalToSizedBuffer(b)
		if err != nil {
			return nil, err
		}
		return b[:n], nil
	}
}
func (m *CoinMetaRollbackProtection) XXX_Merge(src proto.Message) {
	xxx_messageInfo_CoinMetaRollbackProtection.Merge(m, src)
}
func (m *CoinMetaRollbackProtection) XXX_Size() int {
	return m.Size()
}
func (m *CoinMetaRollbackProtection) XXX_DiscardUnknown() {
	xxx_messageInfo_CoinMetaRollbackProtection.DiscardUnknown(m)
}

var xxx_messageInfo_CoinMetaRollbackProtection proto.InternalMessageInfo

func (m *CoinMetaRollbackProtection) GetIndex() string {
	if m != nil {
		return m.Index
	}
	return ""
}

func (m *CoinMetaRollbackProtection) GetLastUpdateSequence() uint64 {
	if m != nil {
		return m.LastUpdateSequence
	}
	return 0
}

func init() {
	proto.RegisterType((*CoinMetaRollbackProtection)(nil), "certusone.wormholechain.tokenbridge.CoinMetaRollbackProtection")
}

func init() {
	proto.RegisterFile("tokenbridge/coin_meta_rollback_protection.proto", fileDescriptor_23ec5ccab8f2b4ca)
}

var fileDescriptor_23ec5ccab8f2b4ca = []byte{
	// 229 bytes of a gzipped FileDescriptorProto
	0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0xff, 0x6c, 0x8f, 0xb1, 0x4a, 0xc4, 0x40,
	0x10, 0x86, 0xb3, 0xa2, 0x82, 0x29, 0x17, 0x8b, 0xc3, 0x62, 0x39, 0xb4, 0xb9, 0xc6, 0xdd, 0xc2,
	0xc2, 0x5e, 0x6b, 0x41, 0x23, 0x36, 0x36, 0x61, 0x77, 0x33, 0x5c, 0x96, 0xdb, 0xcc, 0xc4, 0xcd,
	0x04, 0xcf, 0xb7, 0xf0, 0xb1, 0x2c, 0xaf, 0xb4, 0x94, 0xe4, 0x45, 0xc4, 0x53, 0x83, 0xc2, 0x75,
	0x33, 0xfc, 0xf0, 0xf1, 0x7d, 0xb9, 0x61, 0x5a, 0x01, 0xba, 0x14, 0xaa, 0x25, 0x18, 0x4f, 0x01,
	0xcb, 0x06, 0xd8, 0x96, 0x89, 0x62, 0x74, 0xd6, 0xaf, 0xca, 0x36, 0x11, 0x83, 0xe7, 0x40, 0xa8,
	0xbf, 0x4e, 0x92, 0x67, 0x1e, 0x12, 0xf7, 0x1d, 0x21, 0xe8, 0x67, 0x4a, 0x4d, 0x4d, 0x11, 0x7c,
	0x6d, 0x03, 0xea, 0x3f, 0xa0, 0x53, 0x97, 0x9f, 0x5c, 0x53, 0xc0, 0x1b, 0x60, 0x5b, 0xfc, 0x90,
	0x6e, 0x27, 0x90, 0x3c, 0xce, 0x0f, 0x02, 0x56, 0xb0, 0x9e, 0x89, 0xb9, 0x58, 0x1c, 0x15, 0xdf,
	0x8f, 0xd4, 0xb9, 0x8c, 0xb6, 0xe3, 0x87, 0xb6, 0xb2, 0x0c, 0xf7, 0xf0, 0xd4, 0x03, 0x7a, 0x98,
	0xed, 0xcd, 0xc5, 0x62, 0xbf, 0xd8, 0xb1, 0x5c, 0xdd, 0xbd, 0x0d, 0x4a, 0x6c, 0x06, 0x25, 0x3e,
	0x06, 0x25, 0x5e, 0x47, 0x95, 0x6d, 0x46, 0x95, 0xbd, 0x8f, 0x2a, 0x7b, 0xbc, 0x5c, 0x06, 0xae,
	0x7b, 0xa7, 0x3d, 0x35, 0x66, 0xb2, 0x35, 0xbf, 0xb6, 0xe7, 0x5b, 0x5d, 0xb3, 0xfe, 0x57, 0xce,
	0x2f, 0x2d, 0x74, 0xee, 0x70, 0x9b, 0x78, 0xf1, 0x19, 0x00, 0x00, 0xff, 0xff, 0x05, 0xde, 0xcf,
	0xef, 0x15, 0x01, 0x00, 0x00,
}

func (m *CoinMetaRollbackProtection) Marshal() (dAtA []byte, err error) {
	size := m.Size()
	dAtA = make([]byte, size)
	n, err := m.MarshalToSizedBuffer(dAtA[:size])
	if err != nil {
		return nil, err
	}
	return dAtA[:n], nil
}

func (m *CoinMetaRollbackProtection) MarshalTo(dAtA []byte) (int, error) {
	size := m.Size()
	return m.MarshalToSizedBuffer(dAtA[:size])
}

func (m *CoinMetaRollbackProtection) MarshalToSizedBuffer(dAtA []byte) (int, error) {
	i := len(dAtA)
	_ = i
	var l int
	_ = l
	if m.LastUpdateSequence != 0 {
		i = encodeVarintCoinMetaRollbackProtection(dAtA, i, uint64(m.LastUpdateSequence))
		i--
		dAtA[i] = 0x10
	}
	if len(m.Index) > 0 {
		i -= len(m.Index)
		copy(dAtA[i:], m.Index)
		i = encodeVarintCoinMetaRollbackProtection(dAtA, i, uint64(len(m.Index)))
		i--
		dAtA[i] = 0xa
	}
	return len(dAtA) - i, nil
}

func encodeVarintCoinMetaRollbackProtection(dAtA []byte, offset int, v uint64) int {
	offset -= sovCoinMetaRollbackProtection(v)
	base := offset
	for v >= 1<<7 {
		dAtA[offset] = uint8(v&0x7f | 0x80)
		v >>= 7
		offset++
	}
	dAtA[offset] = uint8(v)
	return base
}
func (m *CoinMetaRollbackProtection) Size() (n int) {
	if m == nil {
		return 0
	}
	var l int
	_ = l
	l = len(m.Index)
	if l > 0 {
		n += 1 + l + sovCoinMetaRollbackProtection(uint64(l))
	}
	if m.LastUpdateSequence != 0 {
		n += 1 + sovCoinMetaRollbackProtection(uint64(m.LastUpdateSequence))
	}
	return n
}

func sovCoinMetaRollbackProtection(x uint64) (n int) {
	return (math_bits.Len64(x|1) + 6) / 7
}
func sozCoinMetaRollbackProtection(x uint64) (n int) {
	return sovCoinMetaRollbackProtection(uint64((x << 1) ^ uint64((int64(x) >> 63))))
}
func (m *CoinMetaRollbackProtection) Unmarshal(dAtA []byte) error {
	l := len(dAtA)
	iNdEx := 0
	for iNdEx < l {
		preIndex := iNdEx
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return ErrIntOverflowCoinMetaRollbackProtection
			}
			if iNdEx >= l {
				return io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= uint64(b&0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		fieldNum := int32(wire >> 3)
		wireType := int(wire & 0x7)
		if wireType == 4 {
			return fmt.Errorf("proto: CoinMetaRollbackProtection: wiretype end group for non-group")
		}
		if fieldNum <= 0 {
			return fmt.Errorf("proto: CoinMetaRollbackProtection: illegal tag %d (wire type %d)", fieldNum, wire)
		}
		switch fieldNum {
		case 1:
			if wireType != 2 {
				return fmt.Errorf("proto: wrong wireType = %d for field Index", wireType)
			}
			var stringLen uint64
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinMetaRollbackProtection
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				stringLen |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			intStringLen := int(stringLen)
			if intStringLen < 0 {
				return ErrInvalidLengthCoinMetaRollbackProtection
			}
			postIndex := iNdEx + intStringLen
			if postIndex < 0 {
				return ErrInvalidLengthCoinMetaRollbackProtection
			}
			if postIndex > l {
				return io.ErrUnexpectedEOF
			}
			m.Index = string(dAtA[iNdEx:postIndex])
			iNdEx = postIndex
		case 2:
			if wireType != 0 {
				return fmt.Errorf("proto: wrong wireType = %d for field LastUpdateSequence", wireType)
			}
			m.LastUpdateSequence = 0
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return ErrIntOverflowCoinMetaRollbackProtection
				}
				if iNdEx >= l {
					return io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				m.LastUpdateSequence |= uint64(b&0x7F) << shift
				if b < 0x80 {
					break
				}
			}
		default:
			iNdEx = preIndex
			skippy, err := skipCoinMetaRollbackProtection(dAtA[iNdEx:])
			if err != nil {
				return err
			}
			if (skippy < 0) || (iNdEx+skippy) < 0 {
				return ErrInvalidLengthCoinMetaRollbackProtection
			}
			if (iNdEx + skippy) > l {
				return io.ErrUnexpectedEOF
			}
			iNdEx += skippy
		}
	}

	if iNdEx > l {
		return io.ErrUnexpectedEOF
	}
	return nil
}
func skipCoinMetaRollbackProtection(dAtA []byte) (n int, err error) {
	l := len(dAtA)
	iNdEx := 0
	depth := 0
	for iNdEx < l {
		var wire uint64
		for shift := uint(0); ; shift += 7 {
			if shift >= 64 {
				return 0, ErrIntOverflowCoinMetaRollbackProtection
			}
			if iNdEx >= l {
				return 0, io.ErrUnexpectedEOF
			}
			b := dAtA[iNdEx]
			iNdEx++
			wire |= (uint64(b) & 0x7F) << shift
			if b < 0x80 {
				break
			}
		}
		wireType := int(wire & 0x7)
		switch wireType {
		case 0:
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowCoinMetaRollbackProtection
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				iNdEx++
				if dAtA[iNdEx-1] < 0x80 {
					break
				}
			}
		case 1:
			iNdEx += 8
		case 2:
			var length int
			for shift := uint(0); ; shift += 7 {
				if shift >= 64 {
					return 0, ErrIntOverflowCoinMetaRollbackProtection
				}
				if iNdEx >= l {
					return 0, io.ErrUnexpectedEOF
				}
				b := dAtA[iNdEx]
				iNdEx++
				length |= (int(b) & 0x7F) << shift
				if b < 0x80 {
					break
				}
			}
			if length < 0 {
				return 0, ErrInvalidLengthCoinMetaRollbackProtection
			}
			iNdEx += length
		case 3:
			depth++
		case 4:
			if depth == 0 {
				return 0, ErrUnexpectedEndOfGroupCoinMetaRollbackProtection
			}
			depth--
		case 5:
			iNdEx += 4
		default:
			return 0, fmt.Errorf("proto: illegal wireType %d", wireType)
		}
		if iNdEx < 0 {
			return 0, ErrInvalidLengthCoinMetaRollbackProtection
		}
		if depth == 0 {
			return iNdEx, nil
		}
	}
	return 0, io.ErrUnexpectedEOF
}

var (
	ErrInvalidLengthCoinMetaRollbackProtection        = fmt.Errorf("proto: negative length found during unmarshaling")
	ErrIntOverflowCoinMetaRollbackProtection          = fmt.Errorf("proto: integer overflow")
	ErrUnexpectedEndOfGroupCoinMetaRollbackProtection = fmt.Errorf("proto: unexpected end of group")
)
