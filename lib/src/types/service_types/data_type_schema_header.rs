// OPCUA for Rust
// SPDX-License-Identifier: MPL-2.0
// Copyright (C) 2017-2022 Adam Lock
//
// This file was autogenerated from Opc.Ua.Types.bsd by tools/schema/gen_types.js
//
// DO NOT EDIT THIS FILE
#![allow(unused_attributes)]
use std::io::{Read, Write};
#[allow(unused_imports)]
use crate::types::{
    encoding::*,
    basic_types::*,
    service_types::impls::MessageInfo,
    node_ids::ObjectId,
    string::UAString,
    service_types::StructureDescription,
    service_types::EnumDescription,
    service_types::SimpleTypeDescription,
};

#[derive(Debug, Clone, PartialEq)]
pub struct DataTypeSchemaHeader {
    pub namespaces: Option<Vec<UAString>>,
    pub structure_data_types: Option<Vec<StructureDescription>>,
    pub enum_data_types: Option<Vec<EnumDescription>>,
    pub simple_data_types: Option<Vec<SimpleTypeDescription>>,
}

impl MessageInfo for DataTypeSchemaHeader {
    fn object_id(&self) -> ObjectId {
        ObjectId::DataTypeSchemaHeader_Encoding_DefaultBinary
    }
}

impl BinaryEncoder<DataTypeSchemaHeader> for DataTypeSchemaHeader {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += byte_len_array(&self.namespaces);
        size += byte_len_array(&self.structure_data_types);
        size += byte_len_array(&self.enum_data_types);
        size += byte_len_array(&self.simple_data_types);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += write_array(stream, &self.namespaces)?;
        size += write_array(stream, &self.structure_data_types)?;
        size += write_array(stream, &self.enum_data_types)?;
        size += write_array(stream, &self.simple_data_types)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S, decoding_options: &DecodingOptions) -> EncodingResult<Self> {
        let namespaces: Option<Vec<UAString>> = read_array(stream, decoding_options)?;
        let structure_data_types: Option<Vec<StructureDescription>> = read_array(stream, decoding_options)?;
        let enum_data_types: Option<Vec<EnumDescription>> = read_array(stream, decoding_options)?;
        let simple_data_types: Option<Vec<SimpleTypeDescription>> = read_array(stream, decoding_options)?;
        Ok(DataTypeSchemaHeader {
            namespaces,
            structure_data_types,
            enum_data_types,
            simple_data_types,
        })
    }
}