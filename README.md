<header>
<img src="https://raw.githubusercontent.com/uzinfocom-org/website/main/src/images/logo.svg" alt="logo" height="100" align="left">
<h1 style="display: inline">Vicardi</h1>

jCard (vCard in JSON format) serde serialization and deserialization.

[![GitHub top language](https://img.shields.io/github/languages/top/uzinfocom-org/vicardi?style=flat-square&logo=github)](https://github.com/uzinfocom-org/vicardi)
[![Chat](https://img.shields.io/badge/Chat-grey?style=flat-square&logo=telegram)](https://t.me/xinuxuz)
[![Test CI](https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml/badge.svg)](https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml)

</header>

## About

Our CCTLD-developed system uses the jCard format to communicate with ICANN services. Unfortunately, since there is no 
jCard serde crate available, we developed our own library and used this library to create an RDAP system that makes the
.uz TLD domains in Uzbekistan both fast and robust.

> [!NOTE]
> This library is developed according to the [RFC 7483](https://datatracker.ietf.org/doc/html/rfc7483) standard.
> 
> While the crate should be fully RFC compliant, please open an issue if you spot anything wrong.


## Using Vicardi

```rust
use vicardi::*;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let mut vcard = Vcard::default();
    vcard.push(Property::new_fn("John Doe", None));

    let json = json!([
        "vcard",
        [
            ["version", {}, "text", "4.0"],
            ["fn", {}, "text", "John Doe"]
        ]
    ]);

    let parsed: Vcard = serde_json::from_value(json.clone())?;

    assert_eq!(serde_json::to_value(&vcard)?, json);
    assert_eq!(parsed, vcard);
    Ok(())
}
```

See the [documentation](https://docs.rs/vicardi/latest/vicardi/) for more details.

## License
    
This library is distributed under the GPL-3.0 license. See the [LICENSE](./LICENSE) for more information!
