use std::{
    cell::RefCell, collections::HashMap, fmt::Debug, path::PathBuf, str::FromStr, sync::Mutex,
};

use serde::Serialize;

lazy_static::lazy_static! {
    static ref TRACE: Mutex<DebugTrace> = Mutex::new(DebugTrace::new());
}

struct DebugTrace {
    data: HashMap<String, String>,
    index: usize,
    trace: Vec<String>,
}

impl DebugTrace {
    fn new() -> Self {
        std::fs::remove_dir_all("debug").unwrap();
        std::fs::create_dir("debug").unwrap();
        Self {
            data: HashMap::new(),
            index: 0,
            trace: Vec::new(),
        }
    }

    fn write(&mut self) {
        let out_path = format!("debug/{:05}.sir", self.index);
        self.index += 1;
        let mut contents = format!(
            r#"{{
  "trace": ["#
        );
        for line in &self.trace {
            contents.push_str("\n    ");
            contents.push_str(line);
            contents.push_str(",");
        }
        contents.push_str("\n  ]");
        for (name, datum) in &self.data {
            contents.push_str(&format!(
                r#",
  "{}": {}"#,
                name, datum
            ));
        }
        contents.push_str("}");
        std::fs::write(&out_path, contents).unwrap();
    }
}

pub fn enter(fn_name: &str, args: &impl Serialize) {
    let mut trace = TRACE.lock().unwrap();
    trace.trace.push(format!(
        r#"{{"event": "enter", "fn_name": "{}", "args": {}}}"#,
        fn_name,
        serde_json::to_string_pretty(args).unwrap()
    ));
    trace.write();
}

pub fn leave(fn_name: &str) {
    let mut trace = TRACE.lock().unwrap();
    trace
        .trace
        .push(format!(r#"{{"event": "leave", "fn_name": "{}"}}"#, fn_name,));
    trace.write();
}

pub fn put(label: &str, data: &impl Serialize) {
    let mut trace = TRACE.lock().unwrap();
    trace.data.insert(
        label.to_owned(),
        serde_json::to_string_pretty(data).unwrap(),
    );
}
