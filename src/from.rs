use std::convert::TryFrom;

pub struct FromEventNumber(i32);

impl TryFrom<i32> for FromEventNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(FromEventNumber(value))
        } else {
            Err(())
        }
    }
}
