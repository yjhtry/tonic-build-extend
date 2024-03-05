# Tonic-build extend

This is a simple example of how to use extended functions of tonic-build.

## Usage

```rust
use tonic_build_extend::BuilderExt;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .types_attributes(
            &[
                "Struct1",
                "Struct2",
            ],
            &[
                "#[derive(derive_builder::Builder)]",
                "#[builder(setter(into), default)]",
            ],
        )
        .fields_attributes(
            &["Struct.fieldName1", "Struct.fieldName2"],
            &["start", "end"],
            &["#[builder(setter(strip_option))]"],
        )
        .compile(&["./protos/pb.proto"], &["protos"])
        .unwrap();

    Command::new("cargo")
        .args(["fmt"])
        .output()
        .expect("Failed to run cargo fmt");

}

```
