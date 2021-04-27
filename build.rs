use grib_build;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();

    let input_path = Path::new("def").join("CCT").join("xml").join("C11.xml");
    let output_path = Path::new(&out_dir).join("cct11.rs");
    let parsed = grib_build::cct11::parse(input_path);
    let built = grib_build::cct11::rebuild(parsed);
    fs::write(
        &output_path,
        format!(
            "pub const COMMON_CODE_TABLE_11: &'static [&'static str] = &{:#?};",
            built
        ),
    )
    .unwrap();

    let input_path = Path::new("def").join("CCT").join("xml").join("C00.xml");
    let output_path = Path::new(&out_dir).join("cct00.rs");
    let parsed = grib_build::cct00::parse(input_path);
    let built = grib_build::cct00::rebuild(parsed);
    fs::write(
        &output_path,
        format!(
            "pub const COMMON_CODE_TABLE_00: &'static [&'static str] = &{:#?};",
            built
        ),
    )
    .unwrap();

    let input_file_names = [
        "def/GRIB2/GRIB2_CodeFlag_0_0_CodeTable_en.csv",
        "def/GRIB2/GRIB2_CodeFlag_1_2_CodeTable_en.csv",
        "def/GRIB2/GRIB2_CodeFlag_1_3_CodeTable_en.csv",
        "def/GRIB2/GRIB2_CodeFlag_1_4_CodeTable_en.csv",
        "def/GRIB2/GRIB2_CodeFlag_3_1_CodeTable_en.csv",
        "def/GRIB2/GRIB2_CodeFlag_4_0_CodeTable_en.csv",
        "def/GRIB2/GRIB2_CodeFlag_5_0_CodeTable_en.csv",
    ];
    let mut db = grib_build::grib2_codeflag_csv::CodeDB::new();
    let output_path = Path::new(&out_dir).join("grib2_codeflag.rs");
    for file_name in &input_file_names {
        let path = PathBuf::from(file_name);
        db.load(path).unwrap();
    }
    fs::write(&output_path, format!("{}", db)).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=def/CCT/xml/C00.xml");
    println!("cargo:rerun-if-changed=def/CCT/xml/C11.xml");
    for file_name in &input_file_names {
        println!("cargo:rerun-if-changed={}", file_name);
    }
}
