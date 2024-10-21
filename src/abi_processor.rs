//! This module provides functionality for processing Ethereum events and generating
//! valid Solidity struct and event definitions.
//!
//! While `event.full_signature()` is available in alloy, it doesn't create struct definitions for
//! complex types (tuples) used in events. This module fills that gap by:
//!
//! 1. Extracting and generating Solidity struct definitions from event inputs
//! 2. Handling nested struct definitions
//! 3. Producing valid Solidity event definitions that reference these structs
use alloy_json_abi::{Event, EventParam, InternalType};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone)]
pub struct SolStruct {
    pub name: String,
    pub fields: Vec<SolField>,
}

#[derive(Debug, Clone)]
pub struct SolField {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct SolEvent {
    pub name: String,
    pub params: Vec<SolEventParam>,
}

#[derive(Debug, Clone)]
pub struct SolEventParam {
    pub name: String,
    pub ty: String,
    pub indexed: bool,
}

impl fmt::Display for SolStruct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "struct {} {{", self.name)?;
        for field in &self.fields {
            writeln!(f, "    {} {};", field.ty, field.name)?;
        }
        write!(f, "}}")
    }
}

impl fmt::Display for SolEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let params: Vec<String> = self
            .params
            .iter()
            .map(|param| {
                format!(
                    "{}{} {}",
                    param.ty,
                    if param.indexed { " indexed" } else { "" },
                    param.name
                )
            })
            .collect();
        write!(f, "event {}({});", self.name, params.join(", "))
    }
}

fn resolve_type(ty: &str, internal_type: &Option<InternalType>) -> String {
    match internal_type {
        Some(InternalType::AddressPayable(_)) => "address payable".to_string(),
        Some(InternalType::Contract(name)) => name.clone(),
        Some(InternalType::Enum { contract: Some(contract), ty: enum_type }) => {
            format!("{}.{}", contract, enum_type)
        }
        Some(InternalType::Enum { contract: None, ty: enum_type }) => enum_type.clone(),
        Some(InternalType::Struct { contract: Some(contract), ty: struct_type }) => {
            format!("{}.{}", contract, struct_type)
        }
        Some(InternalType::Struct { contract: None, ty: struct_type }) => struct_type.clone(),
        Some(InternalType::Other { contract: Some(contract), ty: other_type }) => {
            format!("{}.{}", contract, other_type)
        }
        Some(InternalType::Other { contract: None, ty: other_type }) => other_type.clone(),
        None => ty.to_string(),
    }
}

pub fn process_events(events: &[Event]) -> (Vec<SolStruct>, Vec<SolEvent>) {
    let mut structs = HashMap::new();
    let mut processed_events = Vec::new();

    for event in events {
        collect_structs(&event.inputs, &mut structs);
        let sol_event = SolEvent {
            name: event.name.clone(),
            params: event
                .inputs
                .iter()
                .map(|input| {
                    let ty = if input.ty == "tuple" {
                        resolve_type(&input.ty, &input.internal_type)
                    } else {
                        input.ty.clone()
                    };
                    SolEventParam { name: input.name.clone(), ty, indexed: input.indexed }
                })
                .collect(),
        };
        processed_events.push(sol_event);
    }

    (structs.into_values().collect(), processed_events)
}

fn collect_structs(params: &[EventParam], structs: &mut HashMap<String, SolStruct>) {
    for param in params {
        if param.ty == "tuple" {
            if let Some(InternalType::Struct { ty: struct_name, .. }) = &param.internal_type {
                if !structs.contains_key(struct_name) {
                    let sol_struct = SolStruct {
                        name: struct_name.clone(),
                        fields: param
                            .components
                            .iter()
                            .map(|comp| SolField {
                                name: comp.name.clone(),
                                ty: resolve_type(&comp.ty, &comp.internal_type),
                            })
                            .collect(),
                    };
                    structs.insert(struct_name.clone(), sol_struct);
                }
            }
        }
    }
}
