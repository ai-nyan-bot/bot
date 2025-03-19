// Copyright (c) nyanbot.com 2025.
// This file is licensed under the AGPL-3.0-or-later.

use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};

#[derive(Eq, Hash, Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Partition(pub i32);

impl Partition {
    pub const ONE: Partition = Partition(1);
    pub const TWO: Partition = Partition(2);
    pub const THREE: Partition = Partition(3);
    pub const FOUR: Partition = Partition(4);
    pub const FIVE: Partition = Partition(5);
    pub const SIX: Partition = Partition(6);
    pub const SEVEN: Partition = Partition(7);
    pub const EIGHT: Partition = Partition(8);
}

impl Partition {
    pub fn enumerate() -> Vec<Partition> {
        vec![
            1.into(),
            2.into(),
            3.into(),
            4.into(),
            5.into(),
            6.into(),
            7.into(),
            8.into(),
        ]
    }
}

impl Display for Partition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl AsRef<Partition> for Partition {
    fn as_ref(&self) -> &Partition {
        self
    }
}

impl PartialEq<i32> for Partition {
    fn eq(&self, other: &i32) -> bool {
        self.0 == *other
    }
}

impl From<i32> for Partition {
    fn from(value: i32) -> Self {
        Self(value)
    }
}
