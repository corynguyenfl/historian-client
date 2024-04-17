// SPDX-FileCopyrightText: 2024 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");

    std::fs::create_dir_all("src/generated").unwrap();
    std::fs::create_dir_all("src/generated/io/cloudevents/v1").unwrap();

    std::fs::write("src/generated/io/mod.rs", "pub mod cloudevents;").unwrap();

    std::fs::write("src/generated/io/cloudevents/mod.rs", "pub mod v1;").unwrap();

    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .protoc_arg("--experimental_allow_proto3_optional")
        .out_dir("src/generated")
        .compile(&["protos/historian.proto"], &["protos"])
        .unwrap_or_else(|e| panic!("Failed to compile protos: {}", e));

    std::fs::copy(
        "src/generated/io.cloudevents.v1.rs",
        "src/generated/io/cloudevents/v1/mod.rs",
    )
    .unwrap();
    // delete src/generated/io.cloudevents.v1.rs
    std::fs::remove_file("src/generated/io.cloudevents.v1.rs").unwrap();

    // write this content to src/generated/mod.rs
    /*
       pub mod commonmodule;
       pub mod breakermodule;
       pub mod capbankmodule;
       pub mod essmodule;
       pub mod generationmodule;
       pub mod loadmodule;
       pub mod metermodule;
       pub mod solarmodule;
       pub mod switchmodule;
       pub mod reclosermodule;
       pub mod regulatormodule;
       pub mod resourcemodule;
       pub mod io;
       pub mod historian;
    */
    let mut mod_rs_content = String::new();
    mod_rs_content.push_str("pub mod commonmodule;\n");
    mod_rs_content.push_str("pub mod breakermodule;\n");
    mod_rs_content.push_str("pub mod capbankmodule;\n");
    mod_rs_content.push_str("pub mod essmodule;\n");
    mod_rs_content.push_str("pub mod generationmodule;\n");
    mod_rs_content.push_str("pub mod loadmodule;\n");
    mod_rs_content.push_str("pub mod metermodule;\n");
    mod_rs_content.push_str("pub mod solarmodule;\n");
    mod_rs_content.push_str("pub mod switchmodule;\n");
    mod_rs_content.push_str("pub mod reclosermodule;\n");
    mod_rs_content.push_str("pub mod regulatormodule;\n");
    mod_rs_content.push_str("pub mod resourcemodule;\n");
    mod_rs_content.push_str("pub mod io;\n");
    mod_rs_content.push_str("pub mod historian;\n");

    std::fs::write("src/generated/mod.rs", mod_rs_content).unwrap();
}
