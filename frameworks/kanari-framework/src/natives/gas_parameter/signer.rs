// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::natives::gas_parameter::native::MUL;
use moveos_stdlib::natives::moveos_stdlib::signer::GasParameters;

crate::natives::gas_parameter::native::define_gas_parameters_for_natives!(GasParameters, "type_info", [
    [.module_signer.base, "module_signer.base", 1000 * MUL],
]);
