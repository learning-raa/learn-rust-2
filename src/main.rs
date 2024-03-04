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

/// struct
#[derive(Debug)]
struct SomeStruct(u8);

/// not very pub pub struct
#[allow(dead_code)]
pub(self) struct SelfPubStruct(u8);


#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

/// main function as entry point
fn main() {
    println!("\nHell-0, world!");
    
    println!("31 = {}", PubStruct(31));
    println!("13 = {:?}", PubStruct(13));
    println!("?? --> {:?}", SomeStruct(3));

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        println!( "{}", color );
    }
}


impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "RGB ({red}, {green}, {blue}) 0x{red:0>2X}{green:0>2X}{blue:0>2X}", 
               red=self.red, green=self.green, blue=self.blue )
    }
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
        write!(f, "<{}>\n", self.0)
    }
}

