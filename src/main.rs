#![crate_name = "learn_rust"]

//! Not very bad for start
//! - [x] it's easy
//! - [ ] it works
//!   - really?

/// test struct
#[allow(dead_code)]
struct TestStruct {
    /// test field
    test_field: String,
    /// pub field
    pub pub_field: String,
    /// not very pub pub
    pub(self) super_pub_field: u8,
}

/// pub struct
pub struct PubStruct(u8);

/// not very pub pub struct
#[allow(dead_code)]
pub(self) struct SelfPubStruct(u8);

/// main function as entry point
fn main() {
    println!("\nHell-0, world!");
    
    println!("31 = {}", PubStruct(31));
    println!("13 = {:?}", PubStruct(13));
}


impl core::fmt::Display for PubStruct {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str( "testing" )?;
        f.write_str( format!("<{}>", self.0).as_str() )?;
        Ok(())
    }
}

impl core::fmt::Debug for PubStruct {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str( "dbg\n" )?;
        f.write_str( format!("<{}>", self.0).as_str() )?;
        Ok(())
    }
}

