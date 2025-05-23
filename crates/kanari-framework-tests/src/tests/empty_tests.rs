// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

use crate::binding_test;
use moveos_types::module_binding::MoveFunctionCaller;
use moveos_types::moveos_std::tx_context::TxContext;

#[tokio::test]
async fn test_empty() {
    let binding_test = binding_test::RustBindingTest::new().unwrap();
    let empty = binding_test.as_module_binding::<kanari_types::framework::empty::Empty>();
    let ctx = TxContext::random_for_testing_only();
    empty.empty(&ctx).unwrap();
}
