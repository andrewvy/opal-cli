// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct PInputTransaction {
    // message fields
    pub transaction: ::std::vec::Vec<u8>,
    pub txout_index: i32,
    pub signature: ::std::vec::Vec<u8>,
    pub public_key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PInputTransaction {}

impl PInputTransaction {
    pub fn new() -> PInputTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PInputTransaction {
        static mut instance: ::protobuf::lazy::Lazy<PInputTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PInputTransaction,
        };
        unsafe {
            instance.get(PInputTransaction::new)
        }
    }

    // bytes transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: ::std::vec::Vec<u8>) {
        self.transaction = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction
    }

    // Take field
    pub fn take_transaction(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transaction, ::std::vec::Vec::new())
    }

    pub fn get_transaction(&self) -> &[u8] {
        &self.transaction
    }

    fn get_transaction_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction
    }

    // int32 txout_index = 2;

    pub fn clear_txout_index(&mut self) {
        self.txout_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_txout_index(&mut self, v: i32) {
        self.txout_index = v;
    }

    pub fn get_txout_index(&self) -> i32 {
        self.txout_index
    }

    fn get_txout_index_for_reflect(&self) -> &i32 {
        &self.txout_index
    }

    fn mut_txout_index_for_reflect(&mut self) -> &mut i32 {
        &mut self.txout_index
    }

    // bytes signature = 3;

    pub fn clear_signature(&mut self) {
        self.signature.clear();
    }

    // Param is passed by value, moved
    pub fn set_signature(&mut self, v: ::std::vec::Vec<u8>) {
        self.signature = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_signature(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // Take field
    pub fn take_signature(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.signature, ::std::vec::Vec::new())
    }

    pub fn get_signature(&self) -> &[u8] {
        &self.signature
    }

    fn get_signature_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.signature
    }

    fn mut_signature_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.signature
    }

    // bytes public_key = 4;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public_key, ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn get_public_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }
}

impl ::protobuf::Message for PInputTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transaction)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.txout_index = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.signature)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.transaction.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.transaction);
        }
        if self.txout_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.txout_index, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.signature.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.signature);
        }
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.public_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.transaction.is_empty() {
            os.write_bytes(1, &self.transaction)?;
        }
        if self.txout_index != 0 {
            os.write_int32(2, self.txout_index)?;
        }
        if !self.signature.is_empty() {
            os.write_bytes(3, &self.signature)?;
        }
        if !self.public_key.is_empty() {
            os.write_bytes(4, &self.public_key)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PInputTransaction {
    fn new() -> PInputTransaction {
        PInputTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<PInputTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transaction",
                    PInputTransaction::get_transaction_for_reflect,
                    PInputTransaction::mut_transaction_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "txout_index",
                    PInputTransaction::get_txout_index_for_reflect,
                    PInputTransaction::mut_txout_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "signature",
                    PInputTransaction::get_signature_for_reflect,
                    PInputTransaction::mut_signature_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    PInputTransaction::get_public_key_for_reflect,
                    PInputTransaction::mut_public_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PInputTransaction>(
                    "PInputTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PInputTransaction {
    fn clear(&mut self) {
        self.clear_transaction();
        self.clear_txout_index();
        self.clear_signature();
        self.clear_public_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PInputTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PInputTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct POutputTransaction {
    // message fields
    pub amount: i32,
    pub address: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for POutputTransaction {}

impl POutputTransaction {
    pub fn new() -> POutputTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static POutputTransaction {
        static mut instance: ::protobuf::lazy::Lazy<POutputTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const POutputTransaction,
        };
        unsafe {
            instance.get(POutputTransaction::new)
        }
    }

    // int32 amount = 1;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &i32 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut i32 {
        &mut self.amount
    }

    // bytes address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }
}

impl ::protobuf::Message for POutputTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.amount = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(1, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.address);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.amount != 0 {
            os.write_int32(1, self.amount)?;
        }
        if !self.address.is_empty() {
            os.write_bytes(2, &self.address)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for POutputTransaction {
    fn new() -> POutputTransaction {
        POutputTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<POutputTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "amount",
                    POutputTransaction::get_amount_for_reflect,
                    POutputTransaction::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    POutputTransaction::get_address_for_reflect,
                    POutputTransaction::mut_address_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<POutputTransaction>(
                    "POutputTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for POutputTransaction {
    fn clear(&mut self) {
        self.clear_amount();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for POutputTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for POutputTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PTransaction {
    // message fields
    pub id: ::std::vec::Vec<u8>,
    pub txins: ::protobuf::RepeatedField<PInputTransaction>,
    pub txouts: ::protobuf::RepeatedField<POutputTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PTransaction {}

impl PTransaction {
    pub fn new() -> PTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PTransaction {
        static mut instance: ::protobuf::lazy::Lazy<PTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PTransaction,
        };
        unsafe {
            instance.get(PTransaction::new)
        }
    }

    // bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.id, ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // repeated .PInputTransaction txins = 2;

    pub fn clear_txins(&mut self) {
        self.txins.clear();
    }

    // Param is passed by value, moved
    pub fn set_txins(&mut self, v: ::protobuf::RepeatedField<PInputTransaction>) {
        self.txins = v;
    }

    // Mutable pointer to the field.
    pub fn mut_txins(&mut self) -> &mut ::protobuf::RepeatedField<PInputTransaction> {
        &mut self.txins
    }

    // Take field
    pub fn take_txins(&mut self) -> ::protobuf::RepeatedField<PInputTransaction> {
        ::std::mem::replace(&mut self.txins, ::protobuf::RepeatedField::new())
    }

    pub fn get_txins(&self) -> &[PInputTransaction] {
        &self.txins
    }

    fn get_txins_for_reflect(&self) -> &::protobuf::RepeatedField<PInputTransaction> {
        &self.txins
    }

    fn mut_txins_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PInputTransaction> {
        &mut self.txins
    }

    // repeated .POutputTransaction txouts = 3;

    pub fn clear_txouts(&mut self) {
        self.txouts.clear();
    }

    // Param is passed by value, moved
    pub fn set_txouts(&mut self, v: ::protobuf::RepeatedField<POutputTransaction>) {
        self.txouts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_txouts(&mut self) -> &mut ::protobuf::RepeatedField<POutputTransaction> {
        &mut self.txouts
    }

    // Take field
    pub fn take_txouts(&mut self) -> ::protobuf::RepeatedField<POutputTransaction> {
        ::std::mem::replace(&mut self.txouts, ::protobuf::RepeatedField::new())
    }

    pub fn get_txouts(&self) -> &[POutputTransaction] {
        &self.txouts
    }

    fn get_txouts_for_reflect(&self) -> &::protobuf::RepeatedField<POutputTransaction> {
        &self.txouts
    }

    fn mut_txouts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<POutputTransaction> {
        &mut self.txouts
    }
}

impl ::protobuf::Message for PTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.txins {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.txouts {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.txins)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.txouts)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.id);
        }
        for value in &self.txins {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.txouts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_bytes(1, &self.id)?;
        }
        for v in &self.txins {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.txouts {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PTransaction {
    fn new() -> PTransaction {
        PTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<PTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    PTransaction::get_id_for_reflect,
                    PTransaction::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PInputTransaction>>(
                    "txins",
                    PTransaction::get_txins_for_reflect,
                    PTransaction::mut_txins_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<POutputTransaction>>(
                    "txouts",
                    PTransaction::get_txouts_for_reflect,
                    PTransaction::mut_txouts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PTransaction>(
                    "PTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PTransaction {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_txins();
        self.clear_txouts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PUnspentOutputTransaction {
    // message fields
    pub amount: i32,
    pub address: ::std::vec::Vec<u8>,
    pub spent: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PUnspentOutputTransaction {}

impl PUnspentOutputTransaction {
    pub fn new() -> PUnspentOutputTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PUnspentOutputTransaction {
        static mut instance: ::protobuf::lazy::Lazy<PUnspentOutputTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PUnspentOutputTransaction,
        };
        unsafe {
            instance.get(PUnspentOutputTransaction::new)
        }
    }

    // int32 amount = 1;

    pub fn clear_amount(&mut self) {
        self.amount = 0;
    }

    // Param is passed by value, moved
    pub fn set_amount(&mut self, v: i32) {
        self.amount = v;
    }

    pub fn get_amount(&self) -> i32 {
        self.amount
    }

    fn get_amount_for_reflect(&self) -> &i32 {
        &self.amount
    }

    fn mut_amount_for_reflect(&mut self) -> &mut i32 {
        &mut self.amount
    }

    // bytes address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // bool spent = 3;

    pub fn clear_spent(&mut self) {
        self.spent = false;
    }

    // Param is passed by value, moved
    pub fn set_spent(&mut self, v: bool) {
        self.spent = v;
    }

    pub fn get_spent(&self) -> bool {
        self.spent
    }

    fn get_spent_for_reflect(&self) -> &bool {
        &self.spent
    }

    fn mut_spent_for_reflect(&mut self) -> &mut bool {
        &mut self.spent
    }
}

impl ::protobuf::Message for PUnspentOutputTransaction {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.amount = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.spent = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.amount != 0 {
            my_size += ::protobuf::rt::value_size(1, self.amount, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.address);
        }
        if self.spent != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.amount != 0 {
            os.write_int32(1, self.amount)?;
        }
        if !self.address.is_empty() {
            os.write_bytes(2, &self.address)?;
        }
        if self.spent != false {
            os.write_bool(3, self.spent)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PUnspentOutputTransaction {
    fn new() -> PUnspentOutputTransaction {
        PUnspentOutputTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<PUnspentOutputTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "amount",
                    PUnspentOutputTransaction::get_amount_for_reflect,
                    PUnspentOutputTransaction::mut_amount_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    PUnspentOutputTransaction::get_address_for_reflect,
                    PUnspentOutputTransaction::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "spent",
                    PUnspentOutputTransaction::get_spent_for_reflect,
                    PUnspentOutputTransaction::mut_spent_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PUnspentOutputTransaction>(
                    "PUnspentOutputTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PUnspentOutputTransaction {
    fn clear(&mut self) {
        self.clear_amount();
        self.clear_address();
        self.clear_spent();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PUnspentOutputTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PUnspentOutputTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PUnspentTransaction {
    // message fields
    pub id: ::std::vec::Vec<u8>,
    pub coinbase: bool,
    pub unspent_txouts: ::protobuf::RepeatedField<PUnspentOutputTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PUnspentTransaction {}

impl PUnspentTransaction {
    pub fn new() -> PUnspentTransaction {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PUnspentTransaction {
        static mut instance: ::protobuf::lazy::Lazy<PUnspentTransaction> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PUnspentTransaction,
        };
        unsafe {
            instance.get(PUnspentTransaction::new)
        }
    }

    // bytes id = 1;

    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.id, ::std::vec::Vec::new())
    }

    pub fn get_id(&self) -> &[u8] {
        &self.id
    }

    fn get_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.id
    }

    fn mut_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.id
    }

    // bool coinbase = 2;

    pub fn clear_coinbase(&mut self) {
        self.coinbase = false;
    }

    // Param is passed by value, moved
    pub fn set_coinbase(&mut self, v: bool) {
        self.coinbase = v;
    }

    pub fn get_coinbase(&self) -> bool {
        self.coinbase
    }

    fn get_coinbase_for_reflect(&self) -> &bool {
        &self.coinbase
    }

    fn mut_coinbase_for_reflect(&mut self) -> &mut bool {
        &mut self.coinbase
    }

    // repeated .PUnspentOutputTransaction unspent_txouts = 3;

    pub fn clear_unspent_txouts(&mut self) {
        self.unspent_txouts.clear();
    }

    // Param is passed by value, moved
    pub fn set_unspent_txouts(&mut self, v: ::protobuf::RepeatedField<PUnspentOutputTransaction>) {
        self.unspent_txouts = v;
    }

    // Mutable pointer to the field.
    pub fn mut_unspent_txouts(&mut self) -> &mut ::protobuf::RepeatedField<PUnspentOutputTransaction> {
        &mut self.unspent_txouts
    }

    // Take field
    pub fn take_unspent_txouts(&mut self) -> ::protobuf::RepeatedField<PUnspentOutputTransaction> {
        ::std::mem::replace(&mut self.unspent_txouts, ::protobuf::RepeatedField::new())
    }

    pub fn get_unspent_txouts(&self) -> &[PUnspentOutputTransaction] {
        &self.unspent_txouts
    }

    fn get_unspent_txouts_for_reflect(&self) -> &::protobuf::RepeatedField<PUnspentOutputTransaction> {
        &self.unspent_txouts
    }

    fn mut_unspent_txouts_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PUnspentOutputTransaction> {
        &mut self.unspent_txouts
    }
}

impl ::protobuf::Message for PUnspentTransaction {
    fn is_initialized(&self) -> bool {
        for v in &self.unspent_txouts {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.coinbase = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.unspent_txouts)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.id);
        }
        if self.coinbase != false {
            my_size += 2;
        }
        for value in &self.unspent_txouts {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_bytes(1, &self.id)?;
        }
        if self.coinbase != false {
            os.write_bool(2, self.coinbase)?;
        }
        for v in &self.unspent_txouts {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PUnspentTransaction {
    fn new() -> PUnspentTransaction {
        PUnspentTransaction::new()
    }

    fn descriptor_static(_: ::std::option::Option<PUnspentTransaction>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "id",
                    PUnspentTransaction::get_id_for_reflect,
                    PUnspentTransaction::mut_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "coinbase",
                    PUnspentTransaction::get_coinbase_for_reflect,
                    PUnspentTransaction::mut_coinbase_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PUnspentOutputTransaction>>(
                    "unspent_txouts",
                    PUnspentTransaction::get_unspent_txouts_for_reflect,
                    PUnspentTransaction::mut_unspent_txouts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PUnspentTransaction>(
                    "PUnspentTransaction",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PUnspentTransaction {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_coinbase();
        self.clear_unspent_txouts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PUnspentTransaction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PUnspentTransaction {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PBlock {
    // message fields
    pub version: i32,
    pub bits: i32,
    pub previous_hash: ::std::vec::Vec<u8>,
    pub hash: ::std::vec::Vec<u8>,
    pub timestamp: i32,
    pub nonce: i32,
    pub merkle_root: ::std::vec::Vec<u8>,
    pub transactions: ::protobuf::RepeatedField<PTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PBlock {}

impl PBlock {
    pub fn new() -> PBlock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PBlock {
        static mut instance: ::protobuf::lazy::Lazy<PBlock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PBlock,
        };
        unsafe {
            instance.get(PBlock::new)
        }
    }

    // int32 version = 1;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: i32) {
        self.version = v;
    }

    pub fn get_version(&self) -> i32 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &i32 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut i32 {
        &mut self.version
    }

    // int32 bits = 2;

    pub fn clear_bits(&mut self) {
        self.bits = 0;
    }

    // Param is passed by value, moved
    pub fn set_bits(&mut self, v: i32) {
        self.bits = v;
    }

    pub fn get_bits(&self) -> i32 {
        self.bits
    }

    fn get_bits_for_reflect(&self) -> &i32 {
        &self.bits
    }

    fn mut_bits_for_reflect(&mut self) -> &mut i32 {
        &mut self.bits
    }

    // bytes previous_hash = 3;

    pub fn clear_previous_hash(&mut self) {
        self.previous_hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_previous_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.previous_hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_previous_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.previous_hash
    }

    // Take field
    pub fn take_previous_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.previous_hash, ::std::vec::Vec::new())
    }

    pub fn get_previous_hash(&self) -> &[u8] {
        &self.previous_hash
    }

    fn get_previous_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.previous_hash
    }

    fn mut_previous_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.previous_hash
    }

    // bytes hash = 4;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }

    fn get_hash_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.hash
    }

    // int32 timestamp = 5;

    pub fn clear_timestamp(&mut self) {
        self.timestamp = 0;
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: i32) {
        self.timestamp = v;
    }

    pub fn get_timestamp(&self) -> i32 {
        self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &i32 {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut i32 {
        &mut self.timestamp
    }

    // int32 nonce = 6;

    pub fn clear_nonce(&mut self) {
        self.nonce = 0;
    }

    // Param is passed by value, moved
    pub fn set_nonce(&mut self, v: i32) {
        self.nonce = v;
    }

    pub fn get_nonce(&self) -> i32 {
        self.nonce
    }

    fn get_nonce_for_reflect(&self) -> &i32 {
        &self.nonce
    }

    fn mut_nonce_for_reflect(&mut self) -> &mut i32 {
        &mut self.nonce
    }

    // bytes merkle_root = 7;

    pub fn clear_merkle_root(&mut self) {
        self.merkle_root.clear();
    }

    // Param is passed by value, moved
    pub fn set_merkle_root(&mut self, v: ::std::vec::Vec<u8>) {
        self.merkle_root = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_merkle_root(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkle_root
    }

    // Take field
    pub fn take_merkle_root(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.merkle_root, ::std::vec::Vec::new())
    }

    pub fn get_merkle_root(&self) -> &[u8] {
        &self.merkle_root
    }

    fn get_merkle_root_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.merkle_root
    }

    fn mut_merkle_root_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.merkle_root
    }

    // repeated .PTransaction transactions = 9;

    pub fn clear_transactions(&mut self) {
        self.transactions.clear();
    }

    // Param is passed by value, moved
    pub fn set_transactions(&mut self, v: ::protobuf::RepeatedField<PTransaction>) {
        self.transactions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_transactions(&mut self) -> &mut ::protobuf::RepeatedField<PTransaction> {
        &mut self.transactions
    }

    // Take field
    pub fn take_transactions(&mut self) -> ::protobuf::RepeatedField<PTransaction> {
        ::std::mem::replace(&mut self.transactions, ::protobuf::RepeatedField::new())
    }

    pub fn get_transactions(&self) -> &[PTransaction] {
        &self.transactions
    }

    fn get_transactions_for_reflect(&self) -> &::protobuf::RepeatedField<PTransaction> {
        &self.transactions
    }

    fn mut_transactions_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<PTransaction> {
        &mut self.transactions
    }
}

impl ::protobuf::Message for PBlock {
    fn is_initialized(&self) -> bool {
        for v in &self.transactions {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.version = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.bits = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.previous_hash)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.hash)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.timestamp = tmp;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.nonce = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.merkle_root)?;
                },
                9 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.transactions)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(1, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.bits != 0 {
            my_size += ::protobuf::rt::value_size(2, self.bits, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.previous_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.previous_hash);
        }
        if !self.hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.hash);
        }
        if self.timestamp != 0 {
            my_size += ::protobuf::rt::value_size(5, self.timestamp, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.nonce != 0 {
            my_size += ::protobuf::rt::value_size(6, self.nonce, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.merkle_root.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.merkle_root);
        }
        for value in &self.transactions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.version != 0 {
            os.write_int32(1, self.version)?;
        }
        if self.bits != 0 {
            os.write_int32(2, self.bits)?;
        }
        if !self.previous_hash.is_empty() {
            os.write_bytes(3, &self.previous_hash)?;
        }
        if !self.hash.is_empty() {
            os.write_bytes(4, &self.hash)?;
        }
        if self.timestamp != 0 {
            os.write_int32(5, self.timestamp)?;
        }
        if self.nonce != 0 {
            os.write_int32(6, self.nonce)?;
        }
        if !self.merkle_root.is_empty() {
            os.write_bytes(7, &self.merkle_root)?;
        }
        for v in &self.transactions {
            os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PBlock {
    fn new() -> PBlock {
        PBlock::new()
    }

    fn descriptor_static(_: ::std::option::Option<PBlock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "version",
                    PBlock::get_version_for_reflect,
                    PBlock::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "bits",
                    PBlock::get_bits_for_reflect,
                    PBlock::mut_bits_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "previous_hash",
                    PBlock::get_previous_hash_for_reflect,
                    PBlock::mut_previous_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    PBlock::get_hash_for_reflect,
                    PBlock::mut_hash_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "timestamp",
                    PBlock::get_timestamp_for_reflect,
                    PBlock::mut_timestamp_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "nonce",
                    PBlock::get_nonce_for_reflect,
                    PBlock::mut_nonce_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "merkle_root",
                    PBlock::get_merkle_root_for_reflect,
                    PBlock::mut_merkle_root_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PTransaction>>(
                    "transactions",
                    PBlock::get_transactions_for_reflect,
                    PBlock::mut_transactions_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PBlock>(
                    "PBlock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PBlock {
    fn clear(&mut self) {
        self.clear_version();
        self.clear_bits();
        self.clear_previous_hash();
        self.clear_hash();
        self.clear_timestamp();
        self.clear_nonce();
        self.clear_merkle_root();
        self.clear_transactions();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PBlock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PBlock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PWallet {
    // message fields
    pub secret_key: ::std::vec::Vec<u8>,
    pub public_key: ::std::vec::Vec<u8>,
    pub address: ::std::vec::Vec<u8>,
    pub balance: i32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PWallet {}

impl PWallet {
    pub fn new() -> PWallet {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PWallet {
        static mut instance: ::protobuf::lazy::Lazy<PWallet> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PWallet,
        };
        unsafe {
            instance.get(PWallet::new)
        }
    }

    // bytes secret_key = 1;

    pub fn clear_secret_key(&mut self) {
        self.secret_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_secret_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.secret_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_secret_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret_key
    }

    // Take field
    pub fn take_secret_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.secret_key, ::std::vec::Vec::new())
    }

    pub fn get_secret_key(&self) -> &[u8] {
        &self.secret_key
    }

    fn get_secret_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.secret_key
    }

    fn mut_secret_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.secret_key
    }

    // bytes public_key = 2;

    pub fn clear_public_key(&mut self) {
        self.public_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_public_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.public_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_public_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // Take field
    pub fn take_public_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.public_key, ::std::vec::Vec::new())
    }

    pub fn get_public_key(&self) -> &[u8] {
        &self.public_key
    }

    fn get_public_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.public_key
    }

    fn mut_public_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.public_key
    }

    // bytes address = 3;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::vec::Vec<u8>) {
        self.address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.address, ::std::vec::Vec::new())
    }

    pub fn get_address(&self) -> &[u8] {
        &self.address
    }

    fn get_address_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.address
    }

    fn mut_address_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.address
    }

    // int32 balance = 4;

    pub fn clear_balance(&mut self) {
        self.balance = 0;
    }

    // Param is passed by value, moved
    pub fn set_balance(&mut self, v: i32) {
        self.balance = v;
    }

    pub fn get_balance(&self) -> i32 {
        self.balance
    }

    fn get_balance_for_reflect(&self) -> &i32 {
        &self.balance
    }

    fn mut_balance_for_reflect(&mut self) -> &mut i32 {
        &mut self.balance
    }
}

impl ::protobuf::Message for PWallet {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.secret_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.public_key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.address)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.balance = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.secret_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.secret_key);
        }
        if !self.public_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.public_key);
        }
        if !self.address.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.address);
        }
        if self.balance != 0 {
            my_size += ::protobuf::rt::value_size(4, self.balance, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.secret_key.is_empty() {
            os.write_bytes(1, &self.secret_key)?;
        }
        if !self.public_key.is_empty() {
            os.write_bytes(2, &self.public_key)?;
        }
        if !self.address.is_empty() {
            os.write_bytes(3, &self.address)?;
        }
        if self.balance != 0 {
            os.write_int32(4, self.balance)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PWallet {
    fn new() -> PWallet {
        PWallet::new()
    }

    fn descriptor_static(_: ::std::option::Option<PWallet>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "secret_key",
                    PWallet::get_secret_key_for_reflect,
                    PWallet::mut_secret_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "public_key",
                    PWallet::get_public_key_for_reflect,
                    PWallet::mut_public_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "address",
                    PWallet::get_address_for_reflect,
                    PWallet::mut_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                    "balance",
                    PWallet::get_balance_for_reflect,
                    PWallet::mut_balance_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PWallet>(
                    "PWallet",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PWallet {
    fn clear(&mut self) {
        self.clear_secret_key();
        self.clear_public_key();
        self.clear_address();
        self.clear_balance();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PWallet {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PWallet {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PEmpty {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PEmpty {}

impl PEmpty {
    pub fn new() -> PEmpty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PEmpty {
        static mut instance: ::protobuf::lazy::Lazy<PEmpty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PEmpty,
        };
        unsafe {
            instance.get(PEmpty::new)
        }
    }
}

impl ::protobuf::Message for PEmpty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PEmpty {
    fn new() -> PEmpty {
        PEmpty::new()
    }

    fn descriptor_static(_: ::std::option::Option<PEmpty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PEmpty>(
                    "PEmpty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PEmpty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PEmpty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PEmpty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PSendTransactionRequest {
    // message fields
    pub transaction: ::protobuf::SingularPtrField<PTransaction>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PSendTransactionRequest {}

impl PSendTransactionRequest {
    pub fn new() -> PSendTransactionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PSendTransactionRequest {
        static mut instance: ::protobuf::lazy::Lazy<PSendTransactionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PSendTransactionRequest,
        };
        unsafe {
            instance.get(PSendTransactionRequest::new)
        }
    }

    // .PTransaction transaction = 1;

    pub fn clear_transaction(&mut self) {
        self.transaction.clear();
    }

    pub fn has_transaction(&self) -> bool {
        self.transaction.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transaction(&mut self, v: PTransaction) {
        self.transaction = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction(&mut self) -> &mut PTransaction {
        if self.transaction.is_none() {
            self.transaction.set_default();
        }
        self.transaction.as_mut().unwrap()
    }

    // Take field
    pub fn take_transaction(&mut self) -> PTransaction {
        self.transaction.take().unwrap_or_else(|| PTransaction::new())
    }

    pub fn get_transaction(&self) -> &PTransaction {
        self.transaction.as_ref().unwrap_or_else(|| PTransaction::default_instance())
    }

    fn get_transaction_for_reflect(&self) -> &::protobuf::SingularPtrField<PTransaction> {
        &self.transaction
    }

    fn mut_transaction_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PTransaction> {
        &mut self.transaction
    }
}

impl ::protobuf::Message for PSendTransactionRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.transaction {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transaction)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.transaction.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.transaction.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PSendTransactionRequest {
    fn new() -> PSendTransactionRequest {
        PSendTransactionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PSendTransactionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PTransaction>>(
                    "transaction",
                    PSendTransactionRequest::get_transaction_for_reflect,
                    PSendTransactionRequest::mut_transaction_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PSendTransactionRequest>(
                    "PSendTransactionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PSendTransactionRequest {
    fn clear(&mut self) {
        self.clear_transaction();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PSendTransactionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PSendTransactionRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PSendTransactionResponse {
    // message fields
    pub transaction_id: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PSendTransactionResponse {}

impl PSendTransactionResponse {
    pub fn new() -> PSendTransactionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PSendTransactionResponse {
        static mut instance: ::protobuf::lazy::Lazy<PSendTransactionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PSendTransactionResponse,
        };
        unsafe {
            instance.get(PSendTransactionResponse::new)
        }
    }

    // bytes transaction_id = 1;

    pub fn clear_transaction_id(&mut self) {
        self.transaction_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_transaction_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.transaction_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transaction_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_id
    }

    // Take field
    pub fn take_transaction_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.transaction_id, ::std::vec::Vec::new())
    }

    pub fn get_transaction_id(&self) -> &[u8] {
        &self.transaction_id
    }

    fn get_transaction_id_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.transaction_id
    }

    fn mut_transaction_id_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.transaction_id
    }
}

impl ::protobuf::Message for PSendTransactionResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.transaction_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.transaction_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.transaction_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.transaction_id.is_empty() {
            os.write_bytes(1, &self.transaction_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PSendTransactionResponse {
    fn new() -> PSendTransactionResponse {
        PSendTransactionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PSendTransactionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "transaction_id",
                    PSendTransactionResponse::get_transaction_id_for_reflect,
                    PSendTransactionResponse::mut_transaction_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PSendTransactionResponse>(
                    "PSendTransactionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PSendTransactionResponse {
    fn clear(&mut self) {
        self.clear_transaction_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PSendTransactionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PSendTransactionResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\nopal.proto\"\x93\x01\n\x11PInputTransaction\x12\x20\n\x0btransaction\
    \x18\x01\x20\x01(\x0cR\x0btransaction\x12\x1f\n\x0btxout_index\x18\x02\
    \x20\x01(\x05R\ntxoutIndex\x12\x1c\n\tsignature\x18\x03\x20\x01(\x0cR\ts\
    ignature\x12\x1d\n\npublic_key\x18\x04\x20\x01(\x0cR\tpublicKey\"F\n\x12\
    POutputTransaction\x12\x16\n\x06amount\x18\x01\x20\x01(\x05R\x06amount\
    \x12\x18\n\x07address\x18\x02\x20\x01(\x0cR\x07address\"u\n\x0cPTransact\
    ion\x12\x0e\n\x02id\x18\x01\x20\x01(\x0cR\x02id\x12(\n\x05txins\x18\x02\
    \x20\x03(\x0b2\x12.PInputTransactionR\x05txins\x12+\n\x06txouts\x18\x03\
    \x20\x03(\x0b2\x13.POutputTransactionR\x06txouts\"c\n\x19PUnspentOutputT\
    ransaction\x12\x16\n\x06amount\x18\x01\x20\x01(\x05R\x06amount\x12\x18\n\
    \x07address\x18\x02\x20\x01(\x0cR\x07address\x12\x14\n\x05spent\x18\x03\
    \x20\x01(\x08R\x05spent\"\x84\x01\n\x13PUnspentTransaction\x12\x0e\n\x02\
    id\x18\x01\x20\x01(\x0cR\x02id\x12\x1a\n\x08coinbase\x18\x02\x20\x01(\
    \x08R\x08coinbase\x12A\n\x0eunspent_txouts\x18\x03\x20\x03(\x0b2\x1a.PUn\
    spentOutputTransactionR\runspentTxouts\"\xf7\x01\n\x06PBlock\x12\x18\n\
    \x07version\x18\x01\x20\x01(\x05R\x07version\x12\x12\n\x04bits\x18\x02\
    \x20\x01(\x05R\x04bits\x12#\n\rprevious_hash\x18\x03\x20\x01(\x0cR\x0cpr\
    eviousHash\x12\x12\n\x04hash\x18\x04\x20\x01(\x0cR\x04hash\x12\x1c\n\tti\
    mestamp\x18\x05\x20\x01(\x05R\ttimestamp\x12\x14\n\x05nonce\x18\x06\x20\
    \x01(\x05R\x05nonce\x12\x1f\n\x0bmerkle_root\x18\x07\x20\x01(\x0cR\nmerk\
    leRoot\x121\n\x0ctransactions\x18\t\x20\x03(\x0b2\r.PTransactionR\x0ctra\
    nsactions\"{\n\x07PWallet\x12\x1d\n\nsecret_key\x18\x01\x20\x01(\x0cR\ts\
    ecretKey\x12\x1d\n\npublic_key\x18\x02\x20\x01(\x0cR\tpublicKey\x12\x18\
    \n\x07address\x18\x03\x20\x01(\x0cR\x07address\x12\x18\n\x07balance\x18\
    \x04\x20\x01(\x05R\x07balance\"\x08\n\x06PEmpty\"J\n\x17PSendTransaction\
    Request\x12/\n\x0btransaction\x18\x01\x20\x01(\x0b2\r.PTransactionR\x0bt\
    ransaction\"A\n\x18PSendTransactionResponse\x12%\n\x0etransaction_id\x18\
    \x01\x20\x01(\x0cR\rtransactionId2s\n\tPInternal\x12\x1e\n\tGetWallet\
    \x12\x07.PEmpty\x1a\x08.PWallet\x12F\n\x0fSendTransaction\x12\x18.PSendT\
    ransactionRequest\x1a\x19.PSendTransactionResponseJ\xf2\x12\n\x06\x12\
    \x04\0\0D\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\
    \x02\0\x07\x01\n\n\n\x03\x04\0\x01\x12\x03\x02\x08\x19\n\x0b\n\x04\x04\0\
    \x02\0\x12\x03\x03\x02\x18\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x03\x02\x02\
    \x1b\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x02\x07\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x03\x08\x13\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\
    \x16\x17\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x02\x18\n\r\n\x05\x04\0\
    \x02\x01\x04\x12\x04\x04\x02\x03\x18\n\x0c\n\x05\x04\0\x02\x01\x05\x12\
    \x03\x04\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\x08\x13\n\x0c\
    \n\x05\x04\0\x02\x01\x03\x12\x03\x04\x16\x17\n\x0b\n\x04\x04\0\x02\x02\
    \x12\x03\x05\x02\x16\n\r\n\x05\x04\0\x02\x02\x04\x12\x04\x05\x02\x04\x18\
    \n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x02\x07\n\x0c\n\x05\x04\0\x02\
    \x02\x01\x12\x03\x05\x08\x11\n\x0c\n\x05\x04\0\x02\x02\x03\x12\x03\x05\
    \x14\x15\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x02\x17\n\r\n\x05\x04\0\
    \x02\x03\x04\x12\x04\x06\x02\x05\x16\n\x0c\n\x05\x04\0\x02\x03\x05\x12\
    \x03\x06\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\x12\x03\x06\x08\x12\n\x0c\
    \n\x05\x04\0\x02\x03\x03\x12\x03\x06\x15\x16\n\n\n\x02\x04\x01\x12\x04\t\
    \0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\x1a\n\x0b\n\x04\x04\x01\
    \x02\0\x12\x03\n\x02\x13\n\r\n\x05\x04\x01\x02\0\x04\x12\x04\n\x02\t\x1c\
    \n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\n\x08\x0e\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\x11\x12\
    \n\x0b\n\x04\x04\x01\x02\x01\x12\x03\x0b\x02\x14\n\r\n\x05\x04\x01\x02\
    \x01\x04\x12\x04\x0b\x02\n\x13\n\x0c\n\x05\x04\x01\x02\x01\x05\x12\x03\
    \x0b\x02\x07\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0b\x08\x0f\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x03\x0b\x12\x13\n\n\n\x02\x04\x02\x12\x04\
    \x0e\0\x12\x01\n\n\n\x03\x04\x02\x01\x12\x03\x0e\x08\x14\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x03\x0f\x02\x0f\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x0f\
    \x02\x0e\x16\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x0f\x02\x07\n\x0c\n\
    \x05\x04\x02\x02\0\x01\x12\x03\x0f\x08\n\n\x0c\n\x05\x04\x02\x02\0\x03\
    \x12\x03\x0f\r\x0e\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x10\x02'\n\x0c\n\
    \x05\x04\x02\x02\x01\x04\x12\x03\x10\x02\n\n\x0c\n\x05\x04\x02\x02\x01\
    \x06\x12\x03\x10\x0b\x1c\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x10\x1d\
    \"\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x10%&\n\x0b\n\x04\x04\x02\x02\
    \x02\x12\x03\x11\x02)\n\x0c\n\x05\x04\x02\x02\x02\x04\x12\x03\x11\x02\n\
    \n\x0c\n\x05\x04\x02\x02\x02\x06\x12\x03\x11\x0b\x1d\n\x0c\n\x05\x04\x02\
    \x02\x02\x01\x12\x03\x11\x1e$\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\
    \x11'(\n\n\n\x02\x04\x03\x12\x04\x14\0\x18\x01\n\n\n\x03\x04\x03\x01\x12\
    \x03\x14\x08!\n\x0b\n\x04\x04\x03\x02\0\x12\x03\x15\x02\x13\n\r\n\x05\
    \x04\x03\x02\0\x04\x12\x04\x15\x02\x14#\n\x0c\n\x05\x04\x03\x02\0\x05\
    \x12\x03\x15\x02\x07\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03\x15\x08\x0e\n\
    \x0c\n\x05\x04\x03\x02\0\x03\x12\x03\x15\x11\x12\n\x0b\n\x04\x04\x03\x02\
    \x01\x12\x03\x16\x02\x14\n\r\n\x05\x04\x03\x02\x01\x04\x12\x04\x16\x02\
    \x15\x13\n\x0c\n\x05\x04\x03\x02\x01\x05\x12\x03\x16\x02\x07\n\x0c\n\x05\
    \x04\x03\x02\x01\x01\x12\x03\x16\x08\x0f\n\x0c\n\x05\x04\x03\x02\x01\x03\
    \x12\x03\x16\x12\x13\n\x0b\n\x04\x04\x03\x02\x02\x12\x03\x17\x02\x11\n\r\
    \n\x05\x04\x03\x02\x02\x04\x12\x04\x17\x02\x16\x14\n\x0c\n\x05\x04\x03\
    \x02\x02\x05\x12\x03\x17\x02\x06\n\x0c\n\x05\x04\x03\x02\x02\x01\x12\x03\
    \x17\x07\x0c\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x03\x17\x0f\x10\n\n\n\
    \x02\x04\x04\x12\x04\x1a\0\x1e\x01\n\n\n\x03\x04\x04\x01\x12\x03\x1a\x08\
    \x1b\n\x0b\n\x04\x04\x04\x02\0\x12\x03\x1b\x02\x0f\n\r\n\x05\x04\x04\x02\
    \0\x04\x12\x04\x1b\x02\x1a\x1d\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03\x1b\
    \x02\x07\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x1b\x08\n\n\x0c\n\x05\x04\
    \x04\x02\0\x03\x12\x03\x1b\r\x0e\n\x0b\n\x04\x04\x04\x02\x01\x12\x03\x1c\
    \x02\x14\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04\x1c\x02\x1b\x0f\n\x0c\n\
    \x05\x04\x04\x02\x01\x05\x12\x03\x1c\x02\x06\n\x0c\n\x05\x04\x04\x02\x01\
    \x01\x12\x03\x1c\x07\x0f\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03\x1c\x12\
    \x13\n\x0b\n\x04\x04\x04\x02\x02\x12\x03\x1d\x028\n\x0c\n\x05\x04\x04\
    \x02\x02\x04\x12\x03\x1d\x02\n\n\x0c\n\x05\x04\x04\x02\x02\x06\x12\x03\
    \x1d\x0b$\n\x0c\n\x05\x04\x04\x02\x02\x01\x12\x03\x1d%3\n\x0c\n\x05\x04\
    \x04\x02\x02\x03\x12\x03\x1d67\n\n\n\x02\x04\x05\x12\x04\x20\0-\x01\n\n\
    \n\x03\x04\x05\x01\x12\x03\x20\x08\x0e\n\x0b\n\x04\x04\x05\x02\0\x12\x03\
    !\x02\x14\n\r\n\x05\x04\x05\x02\0\x04\x12\x04!\x02\x20\x10\n\x0c\n\x05\
    \x04\x05\x02\0\x05\x12\x03!\x02\x07\n\x0c\n\x05\x04\x05\x02\0\x01\x12\
    \x03!\x08\x0f\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03!\x12\x13\n\x0b\n\x04\
    \x04\x05\x02\x01\x12\x03\"\x02\x11\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04\
    \"\x02!\x14\n\x0c\n\x05\x04\x05\x02\x01\x05\x12\x03\"\x02\x07\n\x0c\n\
    \x05\x04\x05\x02\x01\x01\x12\x03\"\x08\x0c\n\x0c\n\x05\x04\x05\x02\x01\
    \x03\x12\x03\"\x0f\x10\n\x0b\n\x04\x04\x05\x02\x02\x12\x03$\x02\x1a\n\r\
    \n\x05\x04\x05\x02\x02\x04\x12\x04$\x02\"\x11\n\x0c\n\x05\x04\x05\x02\
    \x02\x05\x12\x03$\x02\x07\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03$\x08\
    \x15\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03$\x18\x19\n\x0b\n\x04\x04\
    \x05\x02\x03\x12\x03%\x02\x11\n\r\n\x05\x04\x05\x02\x03\x04\x12\x04%\x02\
    $\x1a\n\x0c\n\x05\x04\x05\x02\x03\x05\x12\x03%\x02\x07\n\x0c\n\x05\x04\
    \x05\x02\x03\x01\x12\x03%\x08\x0c\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\
    \x03%\x0f\x10\n\x0b\n\x04\x04\x05\x02\x04\x12\x03'\x02\x16\n\r\n\x05\x04\
    \x05\x02\x04\x04\x12\x04'\x02%\x11\n\x0c\n\x05\x04\x05\x02\x04\x05\x12\
    \x03'\x02\x07\n\x0c\n\x05\x04\x05\x02\x04\x01\x12\x03'\x08\x11\n\x0c\n\
    \x05\x04\x05\x02\x04\x03\x12\x03'\x14\x15\n\x0b\n\x04\x04\x05\x02\x05\
    \x12\x03(\x02\x12\n\r\n\x05\x04\x05\x02\x05\x04\x12\x04(\x02'\x16\n\x0c\
    \n\x05\x04\x05\x02\x05\x05\x12\x03(\x02\x07\n\x0c\n\x05\x04\x05\x02\x05\
    \x01\x12\x03(\x08\r\n\x0c\n\x05\x04\x05\x02\x05\x03\x12\x03(\x10\x11\n\
    \x0b\n\x04\x04\x05\x02\x06\x12\x03*\x02\x18\n\r\n\x05\x04\x05\x02\x06\
    \x04\x12\x04*\x02(\x12\n\x0c\n\x05\x04\x05\x02\x06\x05\x12\x03*\x02\x07\
    \n\x0c\n\x05\x04\x05\x02\x06\x01\x12\x03*\x08\x13\n\x0c\n\x05\x04\x05\
    \x02\x06\x03\x12\x03*\x16\x17\n\x0b\n\x04\x04\x05\x02\x07\x12\x03,\x02)\
    \n\x0c\n\x05\x04\x05\x02\x07\x04\x12\x03,\x02\n\n\x0c\n\x05\x04\x05\x02\
    \x07\x06\x12\x03,\x0b\x17\n\x0c\n\x05\x04\x05\x02\x07\x01\x12\x03,\x18$\
    \n\x0c\n\x05\x04\x05\x02\x07\x03\x12\x03,'(\n\n\n\x02\x04\x06\x12\x04/\0\
    4\x01\n\n\n\x03\x04\x06\x01\x12\x03/\x08\x0f\n\x0b\n\x04\x04\x06\x02\0\
    \x12\x030\x02\x17\n\r\n\x05\x04\x06\x02\0\x04\x12\x040\x02/\x11\n\x0c\n\
    \x05\x04\x06\x02\0\x05\x12\x030\x02\x07\n\x0c\n\x05\x04\x06\x02\0\x01\
    \x12\x030\x08\x12\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x030\x15\x16\n\x0b\n\
    \x04\x04\x06\x02\x01\x12\x031\x02\x17\n\r\n\x05\x04\x06\x02\x01\x04\x12\
    \x041\x020\x17\n\x0c\n\x05\x04\x06\x02\x01\x05\x12\x031\x02\x07\n\x0c\n\
    \x05\x04\x06\x02\x01\x01\x12\x031\x08\x12\n\x0c\n\x05\x04\x06\x02\x01\
    \x03\x12\x031\x15\x16\n\x0b\n\x04\x04\x06\x02\x02\x12\x032\x02\x14\n\r\n\
    \x05\x04\x06\x02\x02\x04\x12\x042\x021\x17\n\x0c\n\x05\x04\x06\x02\x02\
    \x05\x12\x032\x02\x07\n\x0c\n\x05\x04\x06\x02\x02\x01\x12\x032\x08\x0f\n\
    \x0c\n\x05\x04\x06\x02\x02\x03\x12\x032\x12\x13\n\x0b\n\x04\x04\x06\x02\
    \x03\x12\x033\x02\x14\n\r\n\x05\x04\x06\x02\x03\x04\x12\x043\x022\x14\n\
    \x0c\n\x05\x04\x06\x02\x03\x05\x12\x033\x02\x07\n\x0c\n\x05\x04\x06\x02\
    \x03\x01\x12\x033\x08\x0f\n\x0c\n\x05\x04\x06\x02\x03\x03\x12\x033\x12\
    \x13\n\n\n\x02\x04\x07\x12\x046\07\x01\n\n\n\x03\x04\x07\x01\x12\x036\
    \x08\x0e\n\n\n\x02\x04\x08\x12\x049\0;\x01\n\n\n\x03\x04\x08\x01\x12\x03\
    9\x08\x1f\n\x0b\n\x04\x04\x08\x02\0\x12\x03:\x02\x1f\n\r\n\x05\x04\x08\
    \x02\0\x04\x12\x04:\x029!\n\x0c\n\x05\x04\x08\x02\0\x06\x12\x03:\x02\x0e\
    \n\x0c\n\x05\x04\x08\x02\0\x01\x12\x03:\x0f\x1a\n\x0c\n\x05\x04\x08\x02\
    \0\x03\x12\x03:\x1d\x1e\n\n\n\x02\x04\t\x12\x04=\0?\x01\n\n\n\x03\x04\t\
    \x01\x12\x03=\x08\x20\n\x0b\n\x04\x04\t\x02\0\x12\x03>\x02\x1b\n\r\n\x05\
    \x04\t\x02\0\x04\x12\x04>\x02=\"\n\x0c\n\x05\x04\t\x02\0\x05\x12\x03>\
    \x02\x07\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03>\x08\x16\n\x0c\n\x05\x04\t\
    \x02\0\x03\x12\x03>\x19\x1a\n\n\n\x02\x06\0\x12\x04A\0D\x01\n\n\n\x03\
    \x06\0\x01\x12\x03A\x08\x11\n\x0b\n\x04\x06\0\x02\0\x12\x03B\x02+\n\x0c\
    \n\x05\x06\0\x02\0\x01\x12\x03B\x06\x0f\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03B\x11\x17\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03B\")\n\x0b\n\x04\x06\0\
    \x02\x01\x12\x03C\x02S\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03C\x06\x15\n\
    \x0c\n\x05\x06\0\x02\x01\x02\x12\x03C\x17.\n\x0c\n\x05\x06\0\x02\x01\x03\
    \x12\x03C9Qb\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
