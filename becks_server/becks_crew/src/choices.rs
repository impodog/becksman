use num_enum::{IntoPrimitive, TryFromPrimitive};
use serde::{Deserialize, Serialize};

#[derive(
    Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Social {
    #[default]
    Student,
    Teacher,
}

#[derive(
    Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Gender {
    #[default]
    Male,
    Female,
}

#[derive(
    Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Clothes {
    S,
    #[default]
    M,
    L,
    XL,
    XXL,
    XXXL,
}

#[derive(
    Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Hand {
    Left,
    #[default]
    Right,
}

#[derive(
    Default, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive, Serialize, Deserialize,
)]
#[repr(u8)]
pub enum Hold {
    #[default]
    Verti,
    Horiz,
}
