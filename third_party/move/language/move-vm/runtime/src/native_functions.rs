// Copyright (c) The Diem Core Contributors
// Copyright (c) The Move Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::data_cache::TransactionCache;
use crate::{
    interpreter::Interpreter, loader::Resolver, native_extensions::NativeContextExtensions,
};
use move_binary_format::errors::{
    ExecutionState, Location, PartialVMError, PartialVMResult, VMResult,
};
use move_binary_format::file_format::AbilitySet;
use move_binary_format::CompiledModule;
use move_core_types::{
    account_address::AccountAddress,
    gas_algebra::{InternalGas, NumBytes},
    identifier::Identifier,
    language_storage::TypeTag,
    value::MoveTypeLayout,
    vm_status::{StatusCode, StatusType},
};
use move_vm_types::{
    loaded_data::runtime_types::Type, natives::function::NativeResult, values::Value,
};
use std::{
    collections::{HashMap, VecDeque},
    fmt::Write,
    sync::Arc,
};

pub type UnboxedNativeFunction = dyn Fn(&mut NativeContext, Vec<Type>, VecDeque<Value>) -> PartialVMResult<NativeResult>
    + Send
    + Sync
    + 'static;

pub type NativeFunction = Arc<UnboxedNativeFunction>;

pub type NativeFunctionTable = Vec<(AccountAddress, Identifier, Identifier, NativeFunction)>;

pub fn make_table(
    addr: AccountAddress,
    elems: &[(&str, &str, NativeFunction)],
) -> NativeFunctionTable {
    make_table_from_iter(addr, elems.iter().cloned())
}

pub fn make_table_from_iter<S: Into<Box<str>>>(
    addr: AccountAddress,
    elems: impl IntoIterator<Item = (S, S, NativeFunction)>,
) -> NativeFunctionTable {
    elems
        .into_iter()
        .map(|(module_name, func_name, func)| {
            (
                addr,
                Identifier::new(module_name).unwrap(),
                Identifier::new(func_name).unwrap(),
                func,
            )
        })
        .collect()
}

pub(crate) struct NativeFunctions(
    HashMap<AccountAddress, HashMap<String, HashMap<String, NativeFunction>>>,
);

impl NativeFunctions {
    pub fn resolve(
        &self,
        addr: &AccountAddress,
        module_name: &str,
        func_name: &str,
    ) -> Option<NativeFunction> {
        self.0.get(addr)?.get(module_name)?.get(func_name).cloned()
    }

    pub fn new<I>(natives: I) -> PartialVMResult<Self>
    where
        I: IntoIterator<Item = (AccountAddress, Identifier, Identifier, NativeFunction)>,
    {
        let mut map = HashMap::new();
        for (addr, module_name, func_name, func) in natives.into_iter() {
            let modules = map.entry(addr).or_insert_with(HashMap::new);
            let funcs = modules
                .entry(module_name.into_string())
                .or_insert_with(HashMap::new);

            if funcs.insert(func_name.into_string(), func).is_some() {
                return Err(PartialVMError::new(StatusCode::DUPLICATE_NATIVE_FUNCTION));
            }
        }
        Ok(Self(map))
    }
}

pub struct NativeContext<'a, 'b> {
    interpreter: &'a mut Interpreter,
    data_store: &'a mut dyn TransactionCache,
    resolver: &'a Resolver<'a>,
    extensions: &'a mut NativeContextExtensions<'b>,
    gas_balance: InternalGas,
}

impl<'a, 'b> NativeContext<'a, 'b> {
    pub(crate) fn new(
        interpreter: &'a mut Interpreter,
        data_store: &'a mut dyn TransactionCache,
        resolver: &'a Resolver<'a>,
        extensions: &'a mut NativeContextExtensions<'b>,
        gas_balance: InternalGas,
    ) -> Self {
        Self {
            interpreter,
            data_store,
            resolver,
            extensions,
            gas_balance,
        }
    }
}

impl<'a, 'b> NativeContext<'a, 'b> {
    pub fn print_stack_trace<B: Write>(&self, buf: &mut B) -> PartialVMResult<()> {
        self.interpreter
            .debug_print_stack_trace(buf, self.resolver.loader())
    }

    pub fn exists_at(
        &mut self,
        address: AccountAddress,
        type_: &Type,
    ) -> VMResult<(bool, Option<NumBytes>)> {
        let (value, num_bytes) = self
            .data_store
            .load_resource(self.resolver.loader(), address, type_)
            .map_err(|err| err.finish(Location::Undefined))?;
        let exists = value
            .exists()
            .map_err(|err| err.finish(Location::Undefined))?;
        Ok((exists, num_bytes))
    }

    pub fn save_event(
        &mut self,
        guid: Vec<u8>,
        seq_num: u64,
        ty: Type,
        val: Value,
    ) -> PartialVMResult<bool> {
        match self
            .data_store
            .emit_event(self.resolver.loader(), guid, seq_num, ty, val)
        {
            Ok(()) => Ok(true),
            Err(e) if e.major_status().status_type() == StatusType::InvariantViolation => Err(e),
            Err(_) => Ok(false),
        }
    }

    pub fn events(&self) -> &Vec<(Vec<u8>, u64, Type, MoveTypeLayout, Value)> {
        self.data_store.events()
    }

    pub fn load_type(&self, type_tag: &TypeTag) -> VMResult<Type> {
        self.resolver.loader().load_type(type_tag, self.data_store)
    }

    pub fn get_type_layout(&self, type_tag: &TypeTag) -> VMResult<MoveTypeLayout> {
        self.resolver
            .loader()
            .get_type_layout(type_tag, self.data_store)
    }

    pub fn get_fully_annotated_type_layout(&self, type_tag: &TypeTag) -> VMResult<MoveTypeLayout> {
        self.resolver
            .loader()
            .get_fully_annotated_type_layout(type_tag, self.data_store)
    }

    pub fn type_to_type_tag(&self, ty: &Type) -> PartialVMResult<TypeTag> {
        self.resolver.loader().type_to_type_tag(ty)
    }

    pub fn type_to_type_layout(&self, ty: &Type) -> PartialVMResult<Option<MoveTypeLayout>> {
        match self.resolver.type_to_type_layout(ty) {
            Ok(ty_layout) => Ok(Some(ty_layout)),
            Err(e) if e.major_status().status_type() == StatusType::InvariantViolation => Err(e),
            Err(_) => Ok(None),
        }
    }

    pub fn type_to_fully_annotated_layout(
        &self,
        ty: &Type,
    ) -> PartialVMResult<Option<MoveTypeLayout>> {
        match self.resolver.type_to_fully_annotated_layout(ty) {
            Ok(ty_layout) => Ok(Some(ty_layout)),
            Err(e) if e.major_status().status_type() == StatusType::InvariantViolation => Err(e),
            Err(_) => Ok(None),
        }
    }

    pub fn extensions(&self) -> &NativeContextExtensions<'b> {
        self.extensions
    }

    pub fn extensions_mut(&mut self) -> &mut NativeContextExtensions<'b> {
        self.extensions
    }

    /// Get count stack frames, including the one of the called native function. This
    /// allows a native function to reflect about its caller.
    pub fn stack_frames(&self, count: usize) -> ExecutionState {
        self.interpreter.get_stack_frames(count)
    }

    pub fn gas_balance(&self) -> InternalGas {
        self.gas_balance
    }

    pub fn verify_module_bundle_for_publication(
        &self,
        modules: &[CompiledModule],
    ) -> PartialVMResult<()> {
        self.resolver
            .loader()
            .verify_module_bundle_for_publication(modules, self.data_store)
            .map_err(|e| e.to_partial())
    }

    pub fn abilities(&self, ty: &Type) -> PartialVMResult<AbilitySet> {
        self.resolver.loader().abilities(ty)
    }
}
