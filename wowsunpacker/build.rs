// build.rs
use std::{path::Path, process::Command};

fn main() {
    // build the visual studio solution with msbuild by using the script
    let mut env = r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Auxiliary\Build\vcvarsamd64_x86.bat";
    // fallback to 2019
    if !Path::new(env).exists() {
        env = r"C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvarsamd64_x86.bat";
    }
    if !Path::new(env).exists() {
        panic!("Visual Studio not found");
    }

    let mut msbuild = Command::new(env);
    msbuild.args([
        "&&",
        "msbuild",
        "..\\paramsunpack\\wowsunpack.sln",
        "/p:Configuration=Release",
        "/p:Platform=Any CPU",
    ]);
    println!("{:?}", msbuild);
    msbuild.status().expect("Failed to build paramsunpack");

    // make sure the DLL is under bin/release
    let dll_path = Path::new("../paramsunpack/wowsunpack/bin/Release");
    if !dll_path.exists() {
        panic!("Release folder not found");
    }

    if !dll_path.join("x64/HenryQuan.WoWsUnpack.dll").exists() {
        panic!("DLL not found");
    }

    // copy all DLLs to the target folder
    let output_path: &Path;
    if cfg!(debug_assertions) {
        output_path = Path::new("target/debug");
    } else {
        output_path = Path::new("target/release");
    }

    if !output_path.exists() {
        panic!("Debug folder not found");
    }

    std::fs::copy(
        dll_path.join("x64/HenryQuan.WoWsUnpack.dll"),
        output_path.join("HenryQuan.WoWsUnpack.dll"),
    )
    .expect("Failed to copy DLL");

    // dependencies
    const DEPENDENCIES: [&str; 6] = [
        "Newtonsoft.Json.dll",
        "Razorvine.Pickle.dll",
        "System.Buffers.dll",
        "System.Memory.dll",
        "System.Numerics.Vectors.dll",
        "System.Runtime.CompilerServices.Unsafe.dll",
    ];

    for dependency in DEPENDENCIES.iter() {
        std::fs::copy(dll_path.join(dependency), output_path.join(dependency))
            .expect("Failed to copy dependency");
    }
}
