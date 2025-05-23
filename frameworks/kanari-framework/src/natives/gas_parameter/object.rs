// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::natives::gas_parameter::native::MUL;
use moveos_stdlib::natives::moveos_stdlib::object::GasParameters;

crate::natives::gas_parameter::native::define_gas_parameters_for_natives!(GasParameters, "object", [
    [.common.load_base, "common.load_base", 1000 * MUL],
    [.common.load_per_byte, "common.load_per_byte", 10 * MUL],
    [.common.load_failure, "common.load_failure", 5 * MUL],

    [.native_object_meta.base, "native_object_meta.base", 100 * MUL],

    [.native_borrow_object.base, "native_borrow_object.base", 500 * MUL],
    [.native_take_object.base, "native_take_object.base", 500 * MUL],
    [.native_transfer_object.base, "native_transfer_object.base", 500 * MUL],
    [.native_to_shared_object.base, "native_to_shared_object.base", 500 * MUL],
    [.native_to_frozen_object.base, "native_to_frozen_object.base", 500 * MUL],

    [.native_add_field.base, "native_add_field.base", 500 * MUL],
    [.native_add_field.per_byte_serialized, "native_add_field.per_byte_serialized", 10 * MUL],
    [.native_borrow_field.base, "native_borrow_field.base", 500 * MUL],
    [.native_borrow_field.per_byte_serialized, "native_borrow_field.per_byte_serialized", 10 * MUL],
    [.native_contains_field.base, "native_contains_field.base", 500 * MUL],
    [.native_contains_field.per_byte_serialized, "native_contains_field.per_byte_serialized", 10 * MUL],
    [.native_contains_field_with_value_type.base, "native_contains_field_with_value_type.base", 500 * MUL],
    [.native_contains_field_with_value_type.per_byte_serialized, "native_contains_field_with_value_type.per_byte_serialized", 10 * MUL],
    [.native_remove_field.base, "native_remove_field.base", 500 * MUL],
    [.native_remove_field.per_byte_serialized, "native_remove_field.per_byte_serialized", 10 * MUL],
]);
