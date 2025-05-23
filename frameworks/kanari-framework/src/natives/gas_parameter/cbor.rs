// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::natives::gas_parameter::native::MUL;
use moveos_stdlib::natives::moveos_stdlib::cbor::GasParameters;

crate::natives::gas_parameter::native::define_gas_parameters_for_natives!(GasParameters, "cbor", [
    [.to_bytes.base, "to_bytes.base", 1000 * MUL],
    [.from_bytes.base, "from_bytes.base", 20 * MUL],
]);
