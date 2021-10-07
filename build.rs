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

	let mut build = Build::new();

	build
		.cpp(true)
		.include("src/yoga")
		// https://github.com/facebook/yoga/blob/8a95fbe87874aec9fcd1cabbee8952755949431d/tools/build_defs/oss/yoga_defs.bzl#L53-64
		.flag("-fno-omit-frame-pointer")
		.flag("-fexceptions")
		.flag("-fvisibility=hidden")
		.flag("-ffunction-sections")
		.flag("-fdata-sections")
		.flag("-Wall")
		.flag("-Werror")
		.flag("-O2")
		.flag("-std=c++11")
		.flag("-DYG_ENABLE_EVENTS")
		// https://github.com/facebook/yoga/blob/8a95fbe87874aec9fcd1cabbee8952755949431d/tools/build_defs/oss/yoga_defs.bzl#L66-68
		.flag("-fPIC")
		// C++ Files
		.file("src/yoga/yoga/log.cpp")
		.file("src/yoga/yoga/Utils.cpp")
		.file("src/yoga/yoga/YGConfig.cpp")
		.file("src/yoga/yoga/YGEnums.cpp")
		.file("src/yoga/yoga/YGLayout.cpp")
		.file("src/yoga/yoga/YGNode.cpp")
		.file("src/yoga/yoga/YGNodePrint.cpp")
		.file("src/yoga/yoga/YGStyle.cpp")
		.file("src/yoga/yoga/YGValue.cpp")
		.file("src/yoga/yoga/Yoga.cpp")
		.file("src/yoga/yoga/event/event.cpp");

	match env::var("CARGO_CFG_TARGET_OS").as_deref() {
		Ok("android") => {
			build.cpp_link_stdlib("c++_static");
		}
		Ok("windows") => {
			build.cpp_link_stdlib("static=stdc++");
		}
		_ => {}
	}

	build.compile("libyoga.a");

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
		.clang_args(["-x", "c++-header"])
		.clang_args(args)
		.no_convert_floats()
		.enable_cxx_namespaces()
		.allowlist_type("YG.*")
		.allowlist_function("YG.*")
		.allowlist_var("YG.*")
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
