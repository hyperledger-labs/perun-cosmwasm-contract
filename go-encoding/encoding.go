package encoding


import (
	"fmt"
	"github.com/novifinancial/serde-reflection/serde-generate/runtime/golang/serde"
	"github.com/novifinancial/serde-reflection/serde-generate/runtime/golang/bcs"
)


type Addr string

func (obj *Addr) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serializer.SerializeStr(((string)(*obj))); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *Addr) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeAddr(deserializer serde.Deserializer) (Addr, error) {
	var obj string
	if err := deserializer.IncreaseContainerDepth(); err != nil { return (Addr)(obj), err }
	if val, err := deserializer.DeserializeStr(); err == nil { obj = val } else { return ((Addr)(obj)), err }
	deserializer.DecreaseContainerDepth()
	return (Addr)(obj), nil
}

func BcsDeserializeAddr(input []byte) (Addr, error) {
	if input == nil {
		var obj Addr
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeAddr(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}

type EncodedBalance map[string]serde.Uint128

func (obj *EncodedBalance) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serialize_map_str_to_u128(((map[string]serde.Uint128)(*obj)), serializer); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *EncodedBalance) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeEncodedBalance(deserializer serde.Deserializer) (EncodedBalance, error) {
	var obj map[string]serde.Uint128
	if err := deserializer.IncreaseContainerDepth(); err != nil { return (EncodedBalance)(obj), err }
	if val, err := deserialize_map_str_to_u128(deserializer); err == nil { obj = val } else { return ((EncodedBalance)(obj)), err }
	deserializer.DecreaseContainerDepth()
	return (EncodedBalance)(obj), nil
}

func BcsDeserializeEncodedBalance(input []byte) (EncodedBalance, error) {
	if input == nil {
		var obj EncodedBalance
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeEncodedBalance(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}

type Funding struct {
	Channel []uint8
	Part OffIdentity
}

func (obj *Funding) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serialize_vector_u8(obj.Channel, serializer); err != nil { return err }
	if err := obj.Part.Serialize(serializer); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *Funding) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeFunding(deserializer serde.Deserializer) (Funding, error) {
	var obj Funding
	if err := deserializer.IncreaseContainerDepth(); err != nil { return obj, err }
	if val, err := deserialize_vector_u8(deserializer); err == nil { obj.Channel = val } else { return obj, err }
	if val, err := DeserializeOffIdentity(deserializer); err == nil { obj.Part = val } else { return obj, err }
	deserializer.DecreaseContainerDepth()
	return obj, nil
}

func BcsDeserializeFunding(input []byte) (Funding, error) {
	if input == nil {
		var obj Funding
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeFunding(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}

type OffIdentity []uint8

func (obj *OffIdentity) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serialize_vector_u8((([]uint8)(*obj)), serializer); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *OffIdentity) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeOffIdentity(deserializer serde.Deserializer) (OffIdentity, error) {
	var obj []uint8
	if err := deserializer.IncreaseContainerDepth(); err != nil { return (OffIdentity)(obj), err }
	if val, err := deserialize_vector_u8(deserializer); err == nil { obj = val } else { return ((OffIdentity)(obj)), err }
	deserializer.DecreaseContainerDepth()
	return (OffIdentity)(obj), nil
}

func BcsDeserializeOffIdentity(input []byte) (OffIdentity, error) {
	if input == nil {
		var obj OffIdentity
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeOffIdentity(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}

type Params struct {
	Nonce []uint8
	Participants []OffIdentity
	DisputeDuration uint64
}

func (obj *Params) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serialize_vector_u8(obj.Nonce, serializer); err != nil { return err }
	if err := serialize_vector_OffIdentity(obj.Participants, serializer); err != nil { return err }
	if err := serializer.SerializeU64(obj.DisputeDuration); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *Params) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeParams(deserializer serde.Deserializer) (Params, error) {
	var obj Params
	if err := deserializer.IncreaseContainerDepth(); err != nil { return obj, err }
	if val, err := deserialize_vector_u8(deserializer); err == nil { obj.Nonce = val } else { return obj, err }
	if val, err := deserialize_vector_OffIdentity(deserializer); err == nil { obj.Participants = val } else { return obj, err }
	if val, err := deserializer.DeserializeU64(); err == nil { obj.DisputeDuration = val } else { return obj, err }
	deserializer.DecreaseContainerDepth()
	return obj, nil
}

func BcsDeserializeParams(input []byte) (Params, error) {
	if input == nil {
		var obj Params
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeParams(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}

type State struct {
	ChannelId []uint8
	Version uint64
	Balances []EncodedBalance
	Finalized bool
}

func (obj *State) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serialize_vector_u8(obj.ChannelId, serializer); err != nil { return err }
	if err := serializer.SerializeU64(obj.Version); err != nil { return err }
	if err := serialize_vector_EncodedBalance(obj.Balances, serializer); err != nil { return err }
	if err := serializer.SerializeBool(obj.Finalized); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *State) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeState(deserializer serde.Deserializer) (State, error) {
	var obj State
	if err := deserializer.IncreaseContainerDepth(); err != nil { return obj, err }
	if val, err := deserialize_vector_u8(deserializer); err == nil { obj.ChannelId = val } else { return obj, err }
	if val, err := deserializer.DeserializeU64(); err == nil { obj.Version = val } else { return obj, err }
	if val, err := deserialize_vector_EncodedBalance(deserializer); err == nil { obj.Balances = val } else { return obj, err }
	if val, err := deserializer.DeserializeBool(); err == nil { obj.Finalized = val } else { return obj, err }
	deserializer.DecreaseContainerDepth()
	return obj, nil
}

func BcsDeserializeState(input []byte) (State, error) {
	if input == nil {
		var obj State
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeState(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}

type Withdrawal struct {
	ChannelId []uint8
	Part OffIdentity
	Receiver Addr
}

func (obj *Withdrawal) Serialize(serializer serde.Serializer) error {
	if err := serializer.IncreaseContainerDepth(); err != nil { return err }
	if err := serialize_vector_u8(obj.ChannelId, serializer); err != nil { return err }
	if err := obj.Part.Serialize(serializer); err != nil { return err }
	if err := obj.Receiver.Serialize(serializer); err != nil { return err }
	serializer.DecreaseContainerDepth()
	return nil
}

func (obj *Withdrawal) BcsSerialize() ([]byte, error) {
	if obj == nil {
		return nil, fmt.Errorf("Cannot serialize null object")
	}
	serializer := bcs.NewSerializer();
	if err := obj.Serialize(serializer); err != nil { return nil, err }
	return serializer.GetBytes(), nil
}

func DeserializeWithdrawal(deserializer serde.Deserializer) (Withdrawal, error) {
	var obj Withdrawal
	if err := deserializer.IncreaseContainerDepth(); err != nil { return obj, err }
	if val, err := deserialize_vector_u8(deserializer); err == nil { obj.ChannelId = val } else { return obj, err }
	if val, err := DeserializeOffIdentity(deserializer); err == nil { obj.Part = val } else { return obj, err }
	if val, err := DeserializeAddr(deserializer); err == nil { obj.Receiver = val } else { return obj, err }
	deserializer.DecreaseContainerDepth()
	return obj, nil
}

func BcsDeserializeWithdrawal(input []byte) (Withdrawal, error) {
	if input == nil {
		var obj Withdrawal
		return obj, fmt.Errorf("Cannot deserialize null array")
	}
	deserializer := bcs.NewDeserializer(input);
	obj, err := DeserializeWithdrawal(deserializer)
	if err == nil && deserializer.GetBufferOffset() < uint64(len(input)) {
		return obj, fmt.Errorf("Some input bytes were not read")
	}
	return obj, err
}
func serialize_map_str_to_u128(value map[string]serde.Uint128, serializer serde.Serializer) error {
	if err := serializer.SerializeLen(uint64(len(value))); err != nil { return err }
	offsets := make([]uint64, len(value))
	count := 0
	for k, v := range(value) {
		offsets[count] = serializer.GetBufferOffset()
		count += 1
		if err := serializer.SerializeStr(k); err != nil { return err }
		if err := serializer.SerializeU128(v); err != nil { return err }
	}
	serializer.SortMapEntries(offsets);
	return nil
}

func deserialize_map_str_to_u128(deserializer serde.Deserializer) (map[string]serde.Uint128, error) {
	length, err := deserializer.DeserializeLen()
	if err != nil { return nil, err }
	obj := make(map[string]serde.Uint128)
	previous_slice := serde.Slice { 0, 0 }
	for i := 0; i < int(length); i++ {
		var slice serde.Slice
		slice.Start = deserializer.GetBufferOffset()
		var key string
		if val, err := deserializer.DeserializeStr(); err == nil { key = val } else { return nil, err }
		slice.End = deserializer.GetBufferOffset()
		if i > 0 {
			err := deserializer.CheckThatKeySlicesAreIncreasing(previous_slice, slice)
			if err != nil { return nil, err }
		}
		previous_slice = slice
		if val, err := deserializer.DeserializeU128(); err == nil { obj[key] = val } else { return nil, err }
	}
	return obj, nil
}

func serialize_vector_EncodedBalance(value []EncodedBalance, serializer serde.Serializer) error {
	if err := serializer.SerializeLen(uint64(len(value))); err != nil { return err }
	for _, item := range(value) {
		if err := item.Serialize(serializer); err != nil { return err }
	}
	return nil
}

func deserialize_vector_EncodedBalance(deserializer serde.Deserializer) ([]EncodedBalance, error) {
	length, err := deserializer.DeserializeLen()
	if err != nil { return nil, err }
	obj := make([]EncodedBalance, length)
	for i := range(obj) {
		if val, err := DeserializeEncodedBalance(deserializer); err == nil { obj[i] = val } else { return nil, err }
	}
	return obj, nil
}

func serialize_vector_OffIdentity(value []OffIdentity, serializer serde.Serializer) error {
	if err := serializer.SerializeLen(uint64(len(value))); err != nil { return err }
	for _, item := range(value) {
		if err := item.Serialize(serializer); err != nil { return err }
	}
	return nil
}

func deserialize_vector_OffIdentity(deserializer serde.Deserializer) ([]OffIdentity, error) {
	length, err := deserializer.DeserializeLen()
	if err != nil { return nil, err }
	obj := make([]OffIdentity, length)
	for i := range(obj) {
		if val, err := DeserializeOffIdentity(deserializer); err == nil { obj[i] = val } else { return nil, err }
	}
	return obj, nil
}

func serialize_vector_u8(value []uint8, serializer serde.Serializer) error {
	if err := serializer.SerializeLen(uint64(len(value))); err != nil { return err }
	for _, item := range(value) {
		if err := serializer.SerializeU8(item); err != nil { return err }
	}
	return nil
}

func deserialize_vector_u8(deserializer serde.Deserializer) ([]uint8, error) {
	length, err := deserializer.DeserializeLen()
	if err != nil { return nil, err }
	obj := make([]uint8, length)
	for i := range(obj) {
		if val, err := deserializer.DeserializeU8(); err == nil { obj[i] = val } else { return nil, err }
	}
	return obj, nil
}

