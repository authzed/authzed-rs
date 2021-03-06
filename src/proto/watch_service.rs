// This file is generated by rust-protobuf 2.22.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `arrakisapi/api/watch_service.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_0;

#[derive(PartialEq,Clone,Default)]
pub struct WatchRequest {
    // message fields
    pub namespaces: ::protobuf::RepeatedField<::std::string::String>,
    pub start_revision: ::protobuf::SingularPtrField<super::core::Zookie>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WatchRequest {
    fn default() -> &'a WatchRequest {
        <WatchRequest as ::protobuf::Message>::default_instance()
    }
}

impl WatchRequest {
    pub fn new() -> WatchRequest {
        ::std::default::Default::default()
    }

    // repeated string namespaces = 1;


    pub fn get_namespaces(&self) -> &[::std::string::String] {
        &self.namespaces
    }
    pub fn clear_namespaces(&mut self) {
        self.namespaces.clear();
    }

    // Param is passed by value, moved
    pub fn set_namespaces(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.namespaces = v;
    }

    // Mutable pointer to the field.
    pub fn mut_namespaces(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.namespaces
    }

    // Take field
    pub fn take_namespaces(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.namespaces, ::protobuf::RepeatedField::new())
    }

    // .Zookie start_revision = 2;


    pub fn get_start_revision(&self) -> &super::core::Zookie {
        self.start_revision.as_ref().unwrap_or_else(|| <super::core::Zookie as ::protobuf::Message>::default_instance())
    }
    pub fn clear_start_revision(&mut self) {
        self.start_revision.clear();
    }

    pub fn has_start_revision(&self) -> bool {
        self.start_revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_revision(&mut self, v: super::core::Zookie) {
        self.start_revision = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_revision(&mut self) -> &mut super::core::Zookie {
        if self.start_revision.is_none() {
            self.start_revision.set_default();
        }
        self.start_revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_revision(&mut self) -> super::core::Zookie {
        self.start_revision.take().unwrap_or_else(|| super::core::Zookie::new())
    }
}

impl ::protobuf::Message for WatchRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.start_revision {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.namespaces)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_revision)?;
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
        for value in &self.namespaces {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        if let Some(ref v) = self.start_revision.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.namespaces {
            os.write_string(1, &v)?;
        };
        if let Some(ref v) = self.start_revision.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> WatchRequest {
        WatchRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "namespaces",
                |m: &WatchRequest| { &m.namespaces },
                |m: &mut WatchRequest| { &mut m.namespaces },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::core::Zookie>>(
                "start_revision",
                |m: &WatchRequest| { &m.start_revision },
                |m: &mut WatchRequest| { &mut m.start_revision },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WatchRequest>(
                "WatchRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WatchRequest {
        static instance: ::protobuf::rt::LazyV2<WatchRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WatchRequest::new)
    }
}

impl ::protobuf::Clear for WatchRequest {
    fn clear(&mut self) {
        self.namespaces.clear();
        self.start_revision.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WatchRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WatchResponse {
    // message fields
    pub updates: ::protobuf::RepeatedField<super::core::RelationTupleUpdate>,
    pub end_revision: ::protobuf::SingularPtrField<super::core::Zookie>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a WatchResponse {
    fn default() -> &'a WatchResponse {
        <WatchResponse as ::protobuf::Message>::default_instance()
    }
}

impl WatchResponse {
    pub fn new() -> WatchResponse {
        ::std::default::Default::default()
    }

    // repeated .RelationTupleUpdate updates = 1;


    pub fn get_updates(&self) -> &[super::core::RelationTupleUpdate] {
        &self.updates
    }
    pub fn clear_updates(&mut self) {
        self.updates.clear();
    }

    // Param is passed by value, moved
    pub fn set_updates(&mut self, v: ::protobuf::RepeatedField<super::core::RelationTupleUpdate>) {
        self.updates = v;
    }

    // Mutable pointer to the field.
    pub fn mut_updates(&mut self) -> &mut ::protobuf::RepeatedField<super::core::RelationTupleUpdate> {
        &mut self.updates
    }

    // Take field
    pub fn take_updates(&mut self) -> ::protobuf::RepeatedField<super::core::RelationTupleUpdate> {
        ::std::mem::replace(&mut self.updates, ::protobuf::RepeatedField::new())
    }

    // .Zookie end_revision = 2;


    pub fn get_end_revision(&self) -> &super::core::Zookie {
        self.end_revision.as_ref().unwrap_or_else(|| <super::core::Zookie as ::protobuf::Message>::default_instance())
    }
    pub fn clear_end_revision(&mut self) {
        self.end_revision.clear();
    }

    pub fn has_end_revision(&self) -> bool {
        self.end_revision.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_revision(&mut self, v: super::core::Zookie) {
        self.end_revision = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_revision(&mut self) -> &mut super::core::Zookie {
        if self.end_revision.is_none() {
            self.end_revision.set_default();
        }
        self.end_revision.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_revision(&mut self) -> super::core::Zookie {
        self.end_revision.take().unwrap_or_else(|| super::core::Zookie::new())
    }
}

impl ::protobuf::Message for WatchResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.updates {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.end_revision {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.updates)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.end_revision)?;
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
        for value in &self.updates {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(ref v) = self.end_revision.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.updates {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(ref v) = self.end_revision.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> WatchResponse {
        WatchResponse::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::core::RelationTupleUpdate>>(
                "updates",
                |m: &WatchResponse| { &m.updates },
                |m: &mut WatchResponse| { &mut m.updates },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::core::Zookie>>(
                "end_revision",
                |m: &WatchResponse| { &m.end_revision },
                |m: &mut WatchResponse| { &mut m.end_revision },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<WatchResponse>(
                "WatchResponse",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static WatchResponse {
        static instance: ::protobuf::rt::LazyV2<WatchResponse> = ::protobuf::rt::LazyV2::INIT;
        instance.get(WatchResponse::new)
    }
}

impl ::protobuf::Clear for WatchResponse {
    fn clear(&mut self) {
        self.updates.clear();
        self.end_revision.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WatchResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WatchResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\"arrakisapi/api/watch_service.proto\x1a\x19arrakisapi/api/core.proto\
    \"^\n\x0cWatchRequest\x12\x1e\n\nnamespaces\x18\x01\x20\x03(\tR\nnamespa\
    ces\x12.\n\x0estart_revision\x18\x02\x20\x01(\x0b2\x07.ZookieR\rstartRev\
    ision\"k\n\rWatchResponse\x12.\n\x07updates\x18\x01\x20\x03(\x0b2\x14.Re\
    lationTupleUpdateR\x07updates\x12*\n\x0cend_revision\x18\x02\x20\x01(\
    \x0b2\x07.ZookieR\x0bendRevision2:\n\x0cWatchService\x12*\n\x05Watch\x12\
    \r.WatchRequest\x1a\x0e.WatchResponse\"\00\x01B&Z$github.com/petricorp/c\
    ode/arrakisapiJ\xd8\x06\n\x06\x12\x04\0\0\x19\x01\n\x08\n\x01\x0c\x12\
    \x03\0\0\x12\n\x08\n\x01\x08\x12\x03\x02\0;\n\t\n\x02\x08\x0b\x12\x03\
    \x02\0;\n\t\n\x02\x03\0\x12\x03\x04\0#\n\n\n\x02\x06\0\x12\x04\x06\0\x08\
    \x01\n\n\n\x03\x06\0\x01\x12\x03\x06\x08\x14\n\x0b\n\x04\x06\0\x02\0\x12\
    \x03\x07\x02:\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x07\x06\x0b\n\x0c\n\
    \x05\x06\0\x02\0\x02\x12\x03\x07\x0c\x18\n\x0c\n\x05\x06\0\x02\0\x06\x12\
    \x03\x07\"(\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x07)6\n\n\n\x02\x04\0\
    \x12\x04\n\0\x0f\x01\n\n\n\x03\x04\0\x01\x12\x03\n\x08\x14\nv\n\x04\x04\
    \0\x02\0\x12\x03\r\x02!\x1ai\x20A\x20watch\x20request\x20specifies\x20on\
    e\x20or\x20more\x20namespaces\x20and\x20a\x20zookie\n\x20representing\
    \x20the\x20time\x20to\x20start\x20watching.\n\n\x0c\n\x05\x04\0\x02\0\
    \x04\x12\x03\r\x02\n\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\r\x0b\x11\n\x0c\
    \n\x05\x04\0\x02\0\x01\x12\x03\r\x12\x1c\n\x0c\n\x05\x04\0\x02\0\x03\x12\
    \x03\r\x1f\x20\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x0e\x02\x1c\n\x0c\n\x05\
    \x04\0\x02\x01\x06\x12\x03\x0e\x02\x08\n\x0c\n\x05\x04\0\x02\x01\x01\x12\
    \x03\x0e\t\x17\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0e\x1a\x1b\n\n\n\
    \x02\x04\x01\x12\x04\x11\0\x19\x01\n\n\n\x03\x04\x01\x01\x12\x03\x11\x08\
    \x15\n\xc0\x02\n\x04\x04\x01\x02\0\x12\x03\x17\x02+\x1a\xb2\x02\x20A\x20\
    watch\x20response\x20contains\x20all\x20tuple\x20modification\x20events\
    \x20in\x20ascending\n\x20timestamp\x20order,\x20from\x20the\x20requested\
    \x20start\x20timestamp\x20to\x20a\x20timestamp\n\x20encoded\x20in\x20a\
    \x20heartbeat\x20zookie\x20included\x20in\x20the\x20watch\x20response.\
    \x20The\x20client\n\x20can\x20use\x20the\x20heartbeat\x20zookie\x20to\
    \x20resume\x20watching\x20where\x20the\x20previous\x20watch\n\x20respons\
    e\x20left\x20off.\n\n\x0c\n\x05\x04\x01\x02\0\x04\x12\x03\x17\x02\n\n\
    \x0c\n\x05\x04\x01\x02\0\x06\x12\x03\x17\x0b\x1e\n\x0c\n\x05\x04\x01\x02\
    \0\x01\x12\x03\x17\x1f&\n\x0c\n\x05\x04\x01\x02\0\x03\x12\x03\x17)*\n\
    \x0b\n\x04\x04\x01\x02\x01\x12\x03\x18\x02\x1a\n\x0c\n\x05\x04\x01\x02\
    \x01\x06\x12\x03\x18\x02\x08\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x18\
    \t\x15\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x18\x18\x19b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
