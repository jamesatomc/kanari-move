// Copyright (c) Kanari Network
// SPDX-License-Identifier: Apache-2.0

pub mod event;
pub mod field;
pub mod state;
pub mod transaction;

pub trait Filter<T> {
    fn matches(&self, item: &T) -> bool;
}
