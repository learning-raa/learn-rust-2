fn main() {
    let nameof = NameOf {
        somebody: "a body".into(),
    };
    println!("dbg\n{:?}\n", nameof);
    let smbd = SomeBody {
        name: "a name".into(),
    };
    println!("dbg\n{:?}\n", smbd);
    println!("dbg\n{:?}\n", SomeBody::from(nameof.clone()));
    let tst: SomeBody = nameof.into();
    println!("dbg\n{:?}\n", tst);

    let try_from_invalid: Option<NameOf> = match NameOf::try_from(4) {
        Ok(val) => Some(val),
        Err(e) => {
            println!("error occured: <{}>", e.to_string());
            None
        }
    };
    let try_from_int = NameOf::try_from(42).unwrap();
    println!("dbg\n{:?}\n", try_from_int);

    println!(" to_string() -> {}", try_from_int.to_string());

    let from_string_invalid: Option<SomeBody> = match "4d2".parse::<SomeBody>() {
        Ok(val) => Some(val),
        Err(e) => {
            println!("another error occured: <{}>", e.to_string());
            None
        }
    };
    let from_string_tst = "42".parse::<SomeBody>().unwrap();
    println!(" parse() -> {}", from_string_tst.name);
}

#[derive(Debug, Clone)]
struct NameOf {
    somebody: String,
}
impl core::fmt::Display for NameOf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, " {{{}}}", self.somebody);
        Ok(())
    }
}
impl TryFrom<i32> for NameOf {
    type Error = String;

    fn try_from(src: i32) -> Result<Self, String> {
        if src != 42 {
            return Err("it's not 42".to_string());
        }
        Ok(Self {
            somebody: format!("from i32 = <{}>", src),
        })
    }
}

#[derive(Debug)]
struct SomeBody {
    name: String,
}
impl From<NameOf> for SomeBody {
    fn from(src: NameOf) -> Self {
        Self {
            name: format!("from <{}>", src.somebody),
        }
    }
}
impl core::str::FromStr for SomeBody {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, String> {
        if s != "42" {
            return Err("it's still NOT 42".into());
        }
        Ok(SomeBody {
            name: "42 is gooooood".into(),
        })
    }
}
