use serde::{Deserialize, Deserializer};
use serde::de::Unexpected;

#[allow(dead_code, clippy::redundant_pub_crate)]
pub(crate) fn check_success_errorId<'de, D: Deserializer<'de>>(de: D) -> Result<(), D::Error> {
    let num = <u8 as Deserialize<'de>>::deserialize(de)?;
    
    #[allow(clippy::if_not_else)]
    if num == 0 {
        Ok(())
    } else {
        Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(u64::from(num)),
            &"0",
        ))
    }
}

#[allow(dead_code, clippy::redundant_pub_crate)]
pub(crate) fn check_failure_errorId<'de, D: Deserializer<'de>>(de: D) -> Result<u8, D::Error> {
    let num = <u8 as Deserialize<'de>>::deserialize(de)?;
    
    #[allow(clippy::if_not_else)]
    if num != 0 {
        Ok(num)
    } else {
        Err(serde::de::Error::invalid_value(
            Unexpected::Unsigned(u64::from(num)),
            &"not 0",
        ))
    }
}

#[allow(dead_code, clippy::redundant_pub_crate)]
pub(crate) fn check_processing_status<'de, D: Deserializer<'de>>(de: D) -> Result<(), D::Error> {
    let num = <&'de str as Deserialize<'de>>::deserialize(de)?;
    
    #[allow(clippy::if_not_else)]
    if num == "processing" {
        Ok(())
    } else {
        Err(serde::de::Error::invalid_value(
            Unexpected::Str(&num),
            &"processing",
        ))
    }
}