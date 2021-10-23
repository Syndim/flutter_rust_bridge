use std::fs;
use std::io::Write;
use std::process::Command;

use log::{debug, warn};

pub fn bindgen_rust_to_dart(
    rust_crate_dir: &str,
    c_output_path: &str,
    dart_output_path: &str,
    dart_class_name: &str,
    c_struct_names: Vec<String>,
) {
    cbindgen(rust_crate_dir, c_output_path, c_struct_names);
    ffigen(c_output_path, dart_output_path, dart_class_name);
}

fn execute_command(arg: &str, current_dir: Option<&str>) {
    let mut cmd = if cfg!(target_os = "windows") {
        let mut cmd = Command::new("cmd");
        cmd.arg("/C");
        cmd
    } else {
        let mut cmd = Command::new("sh");
        cmd.arg("-c");
        cmd
    };

    cmd.arg(arg);

    if let Some(current_dir) = current_dir {
        cmd.current_dir(current_dir);
    }

    debug!("execute command: arg={:?} cmd={:?}", arg, cmd);

    let result = cmd.output().unwrap();

    if result.status.success() {
        debug!("command={:?} output={:?}", cmd, result);
        if String::from_utf8_lossy(&result.stdout).contains("fatal error") {
            warn!( "See keywords such as `error` in command output. Maybe there is a problem? command={:?} output={:?}", cmd, result);
        }
    } else {
        warn!("command={:?} output={:?}", cmd, result);
        panic!("command execution failed. command={:?}", cmd);
    }
}

fn cbindgen(rust_crate_dir: &str, c_output_path: &str, c_struct_names: Vec<String>) {
    debug!(
        "execute cbindgen rust_crate_dir={} c_output_path={}",
        rust_crate_dir, c_output_path
    );

    let config = format!(
        r#"
language = "C"

# do NOT include "stdarg.h", see #108 and #53
sys_includes = ["stdbool.h", "stdint.h", "stdlib.h"]
no_includes = true

[export]
include = [{}]
"#,
        c_struct_names
            .iter()
            .map(|name| format!("\"{}\"", name))
            .collect::<Vec<_>>()
            .join(", ")
    );
    debug!("cbindgen config: {}", config);

    let mut config_file = tempfile::NamedTempFile::new().unwrap();
    config_file.write_all(config.as_bytes()).unwrap();
    debug!("cbindgen config_file: {:?}", config_file);

    execute_command(
        &format!(
            "cbindgen -v --config \"{}\" --output \"{}\"",
            config_file.path().to_str().unwrap(),
            c_output_path,
        ),
        Some(fs::canonicalize(rust_crate_dir).unwrap().to_str().unwrap()),
    );
}

fn ffigen(c_path: &str, dart_path: &str, dart_class_name: &str) {
    debug!("execute ffigen c_path={} dart_path={}", c_path, dart_path);
    let config = format!(
        "
        output: '{}'
        name: '{}'
        description: 'generated by flutter_rust_bridge'
        headers:
          entry-points:
            - '{}'
          include-directives:
            - '{}'
        comments: false
        preamble: |
          // ignore_for_file: camel_case_types, non_constant_identifier_names, avoid_positional_boolean_parameters, annotate_overrides
        ",
        dart_path, dart_class_name, c_path, c_path,
    );
    debug!("ffigen config: {}", config);

    let mut config_file = tempfile::NamedTempFile::new().unwrap();
    config_file.write_all(config.as_bytes()).unwrap();
    debug!("ffigen config_file: {:?}", config_file);

    // NOTE please install ffigen globally first: `dart pub global activate ffigen`
    execute_command(
        &format!(
            "dart pub global run ffigen --config \"{}\"",
            config_file.path().to_str().unwrap()
        ),
        None,
    );
}

pub fn format_rust(path: &str) {
    debug!("execute format_rust path={}", path);
    execute_command(&format!("rustfmt \"{}\"", path), None);
}

pub fn format_dart(path: &str, line_length: i32) {
    debug!(
        "execute format_dart path={} line_length={}",
        path, line_length
    );
    execute_command(
        &format!(
            "dart format {} --line-length {}",
            path,
            &line_length.to_string(),
        ),
        None,
    );
}
