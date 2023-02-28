// Copyright (C) 2023 Greenbone Networks GmbH
//
// SPDX-License-Identifier: GPL-2.0-or-later

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]
mod built_in_functions;
mod naslvalue;
use built_in_functions::array;
use built_in_functions::cryptography;
use built_in_functions::description;
mod error;
use built_in_functions::frame_forgery;
use built_in_functions::function;
use built_in_functions::hostname;
use built_in_functions::misc;
use built_in_functions::string;

use error::FunctionError;

mod assign;
mod call;
mod context;
mod declare;
mod helper;
mod include;
mod interpreter;
mod loader;
mod logger;
mod lookup_keys;
mod loop_extension;
mod operator;

pub use context::Context;
pub use context::ContextType;
pub use context::DefaultContext;
pub use context::Register;
pub use error::InterpretError;
pub use error::InterpretErrorKind;
pub use interpreter::Interpreter;
pub use loader::*;
pub use logger::{DefaultLogger, Mode, NaslLogger};
pub use naslvalue::NaslValue;

// Is a type definition for built-in functions
pub(crate) type NaslFunction<'a, K> =
    fn(&Register, &Context<K>) -> Result<NaslValue, FunctionError>;
pub(crate) fn lookup<K>(function_name: &str) -> Option<NaslFunction<K>>
where
    K: AsRef<str>,
{
    description::lookup(function_name)
        .or_else(|| hostname::lookup(function_name))
        .or_else(|| misc::lookup(function_name))
        .or_else(|| string::lookup(function_name))
        .or_else(|| array::lookup(function_name))
        .or_else(|| function::lookup(function_name))
        .or_else(|| cryptography::lookup(function_name))
        .or_else(|| frame_forgery::lookup(function_name))
}
