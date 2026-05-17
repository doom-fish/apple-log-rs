use std::env;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=swift-bridge");
    println!("cargo:rerun-if-env-changed=DOCS_RS");
    println!("cargo:rerun-if-env-changed=DEVELOPER_DIR");
    println!("cargo:rerun-if-env-changed=SDKROOT");

    if env::var("DOCS_RS").is_ok() {
        return;
    }

    let swift_dir = "swift-bridge";
    let out_dir = env::var("OUT_DIR").expect("OUT_DIR must be set by Cargo");
    let swift_build_dir = format!("{out_dir}/swift-build");

    if let Ok(output) = Command::new("swiftlint")
        .arg("lint")
        .current_dir(swift_dir)
        .output()
    {
        if !output.status.success() {
            eprintln!(
                "SwiftLint warnings:\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        }
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let swift_triple = match target_arch.as_str() {
        "x86_64" => "x86_64-apple-macosx",
        "aarch64" => "arm64-apple-macosx",
        other => {
            panic!("apple-log: unsupported target arch '{other}'. Expected x86_64 or aarch64.")
        }
    };

    let output = Command::new("swift")
        .args([
            "build",
            "-c",
            "release",
            "--triple",
            swift_triple,
            "--package-path",
            swift_dir,
            "--scratch-path",
            &swift_build_dir,
        ])
        .output()
        .expect("failed to build AppleLog Swift bridge");

    if !output.status.success() {
        eprintln!(
            "Swift build STDOUT:\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
        eprintln!(
            "Swift build STDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
        panic!(
            "apple-log: Swift bridge build failed with exit code {:?}",
            output.status.code()
        );
    }

    link_swift_bridge(&swift_build_dir);
}

fn link_swift_bridge(swift_build_dir: &str) {
    println!("cargo:rustc-link-search=native={swift_build_dir}/release");
    println!("cargo:rustc-link-lib=static=AppleLogBridge");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=OSLog");
    println!("cargo:rustc-link-arg=-Wl,-rpath,/usr/lib/swift");

    match Command::new("xcode-select").arg("-p").output() {
        Ok(output) if output.status.success() => {
            let xcode_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let legacy = format!(
                "{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift-5.5/macosx"
            );
            println!("cargo:rustc-link-arg=-Wl,-rpath,{legacy}");
            let current =
                format!("{xcode_path}/Toolchains/XcodeDefault.xctoolchain/usr/lib/swift/macosx");
            println!("cargo:rustc-link-arg=-Wl,-rpath,{current}");
            println!("cargo:rustc-link-search=native={current}");
            for compatibility_lib in [
                "swiftCompatibility50",
                "swiftCompatibility51",
                "swiftCompatibility56",
                "swiftCompatibilityConcurrency",
                "swiftCompatibilityDynamicReplacements",
                "swiftCompatibilityPacks",
            ] {
                println!("cargo:rustc-link-lib=static={compatibility_lib}");
            }
        }
        Ok(output) => {
            println!(
                "cargo:warning=`xcode-select -p` exited non-zero (status={:?}); Swift runtime rpaths were not added.",
                output.status.code()
            );
        }
        Err(err) => {
            println!(
                "cargo:warning=`xcode-select` could not be invoked ({err}); Swift runtime rpaths were not added."
            );
        }
    }
}
