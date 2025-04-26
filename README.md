<p align="center">
    <img src=".github/assets/header.png" alt="Uzinfocom's {Vicardi}">
</p>

<p align="center">
    <h3 align="center">jCard (vCard in JSON format) serde serialization and deserialization.</h3>
</p>

<p align="center">
    <img align="center" src="https://img.shields.io/github/languages/top/uzinfocom-org/vicardi?style=flat&logo=nixos&logoColor=ffffff&labelColor=242424&color=242424" alt="Top Used Language">
    <a href="https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml"><img align="center" src="https://img.shields.io/github/actions/workflow/status/uzinfocom-org/vicardi/test.yml?style=flat&logo=github&logoColor=ffffff&labelColor=242424&color=242424" alt="Test CI"></a>
</p>

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

<p align="center">
    <img src=".github/assets/footer.png" alt="Uzinfocom's {Vicardi}">
</p>
