// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use super::auth_validator::BuiltinAuthValidator;
use crate::addresses::KANARI_FRAMEWORK_ADDRESS;
use anyhow::Result;
use move_core_types::{
    account_address::AccountAddress, ident_str, identifier::IdentStr, value::MoveValue,
};
use moveos_types::{
    module_binding::{ModuleBinding, MoveFunctionCaller},
    moveos_std::tx_context::TxContext,
    state::MoveStructType,
    transaction::FunctionCall,
};

pub const MODULE_NAME: &IdentStr = ident_str!("session_validator");

pub struct SessionValidator {}

impl SessionValidator {
    pub fn auth_validator_id() -> u64 {
        BuiltinAuthValidator::Session.flag().into()
    }
}

impl MoveStructType for SessionValidator {
    const ADDRESS: AccountAddress = KANARI_FRAMEWORK_ADDRESS;
    const MODULE_NAME: &'static IdentStr = MODULE_NAME;
    const STRUCT_NAME: &'static IdentStr = ident_str!("SessionValidator");
}

/// Rust bindings for Kanari Framework session_validator module
pub struct SessionValidatorModule<'a> {
    caller: &'a dyn MoveFunctionCaller,
}

impl<'a> SessionValidatorModule<'a> {
    const VALIDATE_FUNCTION_NAME: &'static IdentStr = ident_str!("validate");

    pub fn validate(&self, ctx: &TxContext, payload: Vec<u8>) -> Result<()> {
        let auth_validator_call = FunctionCall::new(
            Self::function_id(Self::VALIDATE_FUNCTION_NAME),
            vec![],
            vec![MoveValue::vector_u8(payload).simple_serialize().unwrap()],
        );
        self.caller
            .call_function(ctx, auth_validator_call)?
            .into_result()
            .map(|values| {
                debug_assert!(values.is_empty(), "should not have return values");
            })?;
        Ok(())
    }
}

impl<'a> ModuleBinding<'a> for SessionValidatorModule<'a> {
    const MODULE_NAME: &'static IdentStr = MODULE_NAME;
    const MODULE_ADDRESS: AccountAddress = KANARI_FRAMEWORK_ADDRESS;

    fn new(caller: &'a impl MoveFunctionCaller) -> Self
    where
        Self: Sized,
    {
        Self { caller }
    }
}
