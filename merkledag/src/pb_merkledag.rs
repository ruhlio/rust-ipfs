// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct PBLink {
    // message fields
    Hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    Name: ::protobuf::SingularField<::std::string::String>,
    Tsize: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PBLink {}

impl PBLink {
    pub fn new() -> PBLink {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PBLink {
        static mut instance: ::protobuf::lazy::Lazy<PBLink> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PBLink,
        };
        unsafe {
            instance.get(|| {
                PBLink {
                    Hash: ::protobuf::SingularField::none(),
                    Name: ::protobuf::SingularField::none(),
                    Tsize: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes Hash = 1;

    pub fn clear_Hash(&mut self) {
        self.Hash.clear();
    }

    pub fn has_Hash(&self) -> bool {
        self.Hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.Hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Hash.is_none() {
            self.Hash.set_default();
        };
        self.Hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_Hash(&mut self) -> ::std::vec::Vec<u8> {
        self.Hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Hash(&self) -> &[u8] {
        match self.Hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string Name = 2;

    pub fn clear_Name(&mut self) {
        self.Name.clear();
    }

    pub fn has_Name(&self) -> bool {
        self.Name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Name(&mut self, v: ::std::string::String) {
        self.Name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Name(&mut self) -> &mut ::std::string::String {
        if self.Name.is_none() {
            self.Name.set_default();
        };
        self.Name.as_mut().unwrap()
    }

    // Take field
    pub fn take_Name(&mut self) -> ::std::string::String {
        self.Name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_Name(&self) -> &str {
        match self.Name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 Tsize = 3;

    pub fn clear_Tsize(&mut self) {
        self.Tsize = ::std::option::Option::None;
    }

    pub fn has_Tsize(&self) -> bool {
        self.Tsize.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Tsize(&mut self, v: u64) {
        self.Tsize = ::std::option::Option::Some(v);
    }

    pub fn get_Tsize(&self) -> u64 {
        self.Tsize.unwrap_or(0)
    }
}

impl ::protobuf::Message for PBLink {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Hash));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.Name));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.Tsize = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.Hash {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.Name {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.Tsize {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.Hash.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.Name.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.Tsize {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PBLink>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PBLink {
    fn new() -> PBLink {
        PBLink::new()
    }

    fn descriptor_static(_: ::std::option::Option<PBLink>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "Hash",
                    PBLink::has_Hash,
                    PBLink::get_Hash,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "Name",
                    PBLink::has_Name,
                    PBLink::get_Name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "Tsize",
                    PBLink::has_Tsize,
                    PBLink::get_Tsize,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PBLink>(
                    "PBLink",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PBLink {
    fn clear(&mut self) {
        self.clear_Hash();
        self.clear_Name();
        self.clear_Tsize();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PBLink {
    fn eq(&self, other: &PBLink) -> bool {
        self.Hash == other.Hash &&
        self.Name == other.Name &&
        self.Tsize == other.Tsize &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PBLink {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PBNode {
    // message fields
    Links: ::protobuf::RepeatedField<PBLink>,
    Data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PBNode {}

impl PBNode {
    pub fn new() -> PBNode {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PBNode {
        static mut instance: ::protobuf::lazy::Lazy<PBNode> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PBNode,
        };
        unsafe {
            instance.get(|| {
                PBNode {
                    Links: ::protobuf::RepeatedField::new(),
                    Data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .PBLink Links = 2;

    pub fn clear_Links(&mut self) {
        self.Links.clear();
    }

    // Param is passed by value, moved
    pub fn set_Links(&mut self, v: ::protobuf::RepeatedField<PBLink>) {
        self.Links = v;
    }

    // Mutable pointer to the field.
    pub fn mut_Links(&mut self) -> &mut ::protobuf::RepeatedField<PBLink> {
        &mut self.Links
    }

    // Take field
    pub fn take_Links(&mut self) -> ::protobuf::RepeatedField<PBLink> {
        ::std::mem::replace(&mut self.Links, ::protobuf::RepeatedField::new())
    }

    pub fn get_Links(&self) -> &[PBLink] {
        &self.Links
    }

    // optional bytes Data = 1;

    pub fn clear_Data(&mut self) {
        self.Data.clear();
    }

    pub fn has_Data(&self) -> bool {
        self.Data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_Data(&mut self, v: ::std::vec::Vec<u8>) {
        self.Data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_Data(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.Data.is_none() {
            self.Data.set_default();
        };
        self.Data.as_mut().unwrap()
    }

    // Take field
    pub fn take_Data(&mut self) -> ::std::vec::Vec<u8> {
        self.Data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_Data(&self) -> &[u8] {
        match self.Data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for PBNode {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                2 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.Links));
                },
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.Data));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.Links {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.Data {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.Links {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.Data.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PBNode>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PBNode {
    fn new() -> PBNode {
        PBNode::new()
    }

    fn descriptor_static(_: ::std::option::Option<PBNode>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "Links",
                    PBNode::get_Links,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "Data",
                    PBNode::has_Data,
                    PBNode::get_Data,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PBNode>(
                    "PBNode",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PBNode {
    fn clear(&mut self) {
        self.clear_Links();
        self.clear_Data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PBNode {
    fn eq(&self, other: &PBNode) -> bool {
        self.Links == other.Links &&
        self.Data == other.Data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PBNode {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0f, 0x6d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x64, 0x61, 0x67, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0x46, 0x0a, 0x06, 0x50, 0x42, 0x4c, 0x69, 0x6e, 0x6b, 0x12, 0x12, 0x0a, 0x04, 0x48,
    0x61, 0x73, 0x68, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x68, 0x61, 0x73, 0x68, 0x12,
    0x12, 0x0a, 0x04, 0x4e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e,
    0x61, 0x6d, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x54, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x05, 0x74, 0x73, 0x69, 0x7a, 0x65, 0x22, 0x3b, 0x0a, 0x06, 0x50, 0x42, 0x4e,
    0x6f, 0x64, 0x65, 0x12, 0x1d, 0x0a, 0x05, 0x4c, 0x69, 0x6e, 0x6b, 0x73, 0x18, 0x02, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x07, 0x2e, 0x50, 0x42, 0x4c, 0x69, 0x6e, 0x6b, 0x52, 0x05, 0x6c, 0x69, 0x6e,
    0x6b, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x44, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x4a, 0xe8, 0x04, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x15,
    0x01, 0x0a, 0x24, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x01, 0x00, 0x0b, 0x01, 0x1a, 0x18, 0x20,
    0x41, 0x6e, 0x20, 0x49, 0x50, 0x46, 0x53, 0x20, 0x4d, 0x65, 0x72, 0x6b, 0x6c, 0x65, 0x44, 0x41,
    0x47, 0x20, 0x4c, 0x69, 0x6e, 0x6b, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x01, 0x08, 0x0e, 0x0a, 0x2d, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x02, 0x1a,
    0x1a, 0x20, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x68, 0x61, 0x73, 0x68, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63,
    0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x04, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x0b, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x18, 0x19, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x01, 0x12, 0x03, 0x07, 0x02, 0x1b, 0x1a, 0x2e, 0x20, 0x75, 0x74, 0x66, 0x20, 0x73, 0x74,
    0x72, 0x69, 0x6e, 0x67, 0x20, 0x6e, 0x61, 0x6d, 0x65, 0x2e, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c,
    0x64, 0x20, 0x62, 0x65, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x20, 0x70, 0x65, 0x72, 0x20,
    0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x07, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x12,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x19, 0x1a, 0x0a,
    0x2f, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x02, 0x1c, 0x1a, 0x22, 0x20, 0x63,
    0x75, 0x6d, 0x75, 0x6c, 0x61, 0x74, 0x69, 0x76, 0x65, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x61, 0x72, 0x67, 0x65, 0x74, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0a, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x1a, 0x1b, 0x0a, 0x24, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04,
    0x0e, 0x00, 0x15, 0x01, 0x1a, 0x18, 0x20, 0x41, 0x6e, 0x20, 0x49, 0x50, 0x46, 0x53, 0x20, 0x4d,
    0x65, 0x72, 0x6b, 0x6c, 0x65, 0x44, 0x41, 0x47, 0x20, 0x4e, 0x6f, 0x64, 0x65, 0x0a, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x08, 0x0e, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x11, 0x02, 0x1c, 0x1a, 0x17, 0x20, 0x72, 0x65, 0x66, 0x73, 0x20, 0x74,
    0x6f, 0x20, 0x6f, 0x74, 0x68, 0x65, 0x72, 0x20, 0x6f, 0x62, 0x6a, 0x65, 0x63, 0x74, 0x73, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x11, 0x02, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x11, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11, 0x12, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x1a, 0x1b, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x14, 0x02, 0x1a, 0x1a, 0x12, 0x20, 0x6f, 0x70, 0x61, 0x71, 0x75, 0x65, 0x20, 0x75,
    0x73, 0x65, 0x72, 0x20, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x14, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x14, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x14, 0x11, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x14, 0x18,
    0x19,
];

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
