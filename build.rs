extern crate bindgen;
extern crate cc;

use bindgen::RustTarget;
use cc::Build;
use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn apple_sdk_root(sdk: &str) -> String {
	let sdk_path = Command::new("xcrun")
		.arg("--show-sdk-path")
		.arg("--sdk")
		.arg(sdk)
		.stderr(Stdio::inherit())
		.output()
		.unwrap()
		.stdout;

	String::from_utf8(sdk_path).unwrap().trim().to_owned()
}

fn main() {
	Command::new("git")
		.args(&["submodule", "init"])
		.status()
		.expect("Unable to initialize git submodules");
	Command::new("git")
		.args(&["submodule", "update"])
		.status()
		.expect("Unable to update the submodule repositories");

	Build::new()
		.cpp(true)
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/BUCK#L13
		.flag("-std=c++1y")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L49-L56
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-Wall")
		.flag("-O3")
		.flag("-ffast-math")
		// https://github.com/facebook/yoga/blob/c5f826de8306e5fbe5963f944c75add827e096c3/yoga_defs.bzl#L58-L60
		.flag("-fPIC")
		// C++ Files
		.file("src/yoga/yoga/Utils.cpp")
		.file("src/yoga/yoga/YGConfig.cpp")
		.file("src/yoga/yoga/YGEnums.cpp")
		.file("src/yoga/yoga/YGFloatOptional.cpp")
		.file("src/yoga/yoga/YGLayout.cpp")
		.file("src/yoga/yoga/YGNode.cpp")
		.file("src/yoga/yoga/YGNodePrint.cpp")
		.file("src/yoga/yoga/YGStyle.cpp")
		.file("src/yoga/yoga/Yoga.cpp")
		.compile("libyoga.a");

	let mut args = vec![];

	match env::var("CARGO_CFG_TARGET_ARCH")
		.ok()
		.zip(env::var("CARGO_CFG_TARGET_OS").ok())
	{
		Some((arch, os)) if arch == "x86_64" && os == "ios" => {
			args = vec!["-isysroot".to_owned(), apple_sdk_root("iphonesimulator")];
		}
		Some((_, os)) if os == "ios" => {
			args = vec!["-isysroot".to_owned(), apple_sdk_root("iphoneos")];
		}
		_ => {}
	}

	let bindings = bindgen::Builder::default()
		.rust_target(RustTarget::Stable_1_21)
		.clang_args(args)
		.no_convert_floats()
		.enable_cxx_namespaces()
		.whitelist_type("YG.*")
		.whitelist_function("YG.*")
		.whitelist_var("YG.*")
		.layout_tests(false)
		.rustfmt_bindings(false)
		.rustified_enum("YG.*")
		.header("src/yoga/yoga/Yoga.h")
		.generate()
		.expect("Unable to generate bindings");

	let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

	bindings
		.write_to_file(out_path.join("bindings.rs"))
		.expect("Unable to write bindings!");
}
