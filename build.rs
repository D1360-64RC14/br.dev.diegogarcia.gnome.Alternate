use std::{
    path::Path,
    process::{Command, Output},
};

use walkdir::WalkDir;

const PATH_DATA_UI: &str = "data/ui";
const PATH_DATA_GRESOURCE: &str = "data";

fn main() {
    build_ui();
    build_gresource();
}

fn build_ui() {
    let ui_path = Path::new(PATH_DATA_UI);
    println!("cargo:rerun-if-changed={}", PATH_DATA_UI);

    let blueprint_paths = WalkDir::new(ui_path) //
        .into_iter()
        .filter_map(|item| match item {
            Ok(item) if item.path().is_file() => match item.path().to_str() {
                Some(path_str) if path_str.ends_with(".blp") => Some(path_str.to_string()),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();

    if blueprint_paths.is_empty() {
        eprintln!("no blueprint files found!");
        return;
    }

    let blueprint_compiler_output = Command::new("blueprint-compiler") //
        .arg("batch-compile")
        .arg("data/ui/compiled")
        .arg("data/ui")
        .args(blueprint_paths)
        .output()
        .expect("unable to run blueprint-compiler");

    if blueprint_compiler_output.status.success() {
        evaluate_output("blueprint-compiler succeeded", blueprint_compiler_output);
    } else {
        evaluate_output("blueprint-compiler failed", blueprint_compiler_output);
    };
}

fn build_gresource() {
    let gresource_path = Path::new(PATH_DATA_GRESOURCE);
    println!("cargo:rerun-if-changed={}", PATH_DATA_GRESOURCE);

    let resource_paths = WalkDir::new(gresource_path) //
        .into_iter()
        .filter_map(|item| match item {
            Ok(item) if item.path().is_file() => match item.path().to_str() {
                Some(path_str) if path_str.ends_with(".gresource.xml") => Some(path_str.to_owned()),
                _ => None,
            },
            _ => None,
        })
        .collect::<Vec<_>>();

    if resource_paths.is_empty() {
        eprintln!("no resource files found!");
        return;
    }

    for path in resource_paths {
        let glib_compile_resources_output = Command::new("glib-compile-resources") //
            .arg(path)
            .output()
            .expect("unable to run glib-compile-resources");

        if glib_compile_resources_output.status.success() {
            evaluate_output(
                "glib-compile-resources succeeded",
                glib_compile_resources_output,
            );
        } else {
            evaluate_output(
                "glib-compile-resources failed",
                glib_compile_resources_output,
            );
        }
    }
}

fn evaluate_output(msg: &str, output: Output) {
    eprintln!("{}", msg);

    eprint!(
        "{}",
        String::from_utf8_lossy(match output.status.success() {
            true => &output.stdout,
            false => &output.stderr,
        })
        .lines()
        .map(|line| format!("| {}", line))
        .collect::<Vec<String>>()
        .join("\n")
    );

    if !output.status.success() {
        std::process::exit(output.status.code().unwrap_or(0));
    }
}
