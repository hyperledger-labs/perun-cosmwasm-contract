//  Copyright 2021 PolyCrypt GmbH
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//    http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

use perun_cosmwasm::types::*;
use serde_generate::{golang, CodeGeneratorConfig, Encoding};
use serde_reflection::{Tracer, TracerConfig};
use std::{
    env::current_dir,
    fs::{create_dir_all, remove_file, write},
    path::Path,
    string::String,
};

fn main() {
    let mut dir = current_dir().unwrap();
    dir.push("go-encoding");
    create_dir_all(&dir).unwrap();
    let mut tracer = Tracer::new(TracerConfig::default());

    tracer.trace_simple_type::<State>().unwrap();
    tracer.trace_simple_type::<Params>().unwrap();
    tracer.trace_simple_type::<Withdrawal>().unwrap();
    tracer.trace_simple_type::<Funding>().unwrap();
    tracer.trace_simple_type::<NativeBalance>().unwrap();
    let reg = tracer.registry().unwrap();

    let mut source = Vec::new();
    let config =
        CodeGeneratorConfig::new("encoding".to_string()).with_encodings(vec![Encoding::Bcs]);

    let gen = golang::CodeGenerator::new(&config);
    gen.output(&mut source, &reg).unwrap();

    write_file(source, "encoding.go".into(), &dir);
}

fn write_file(data: Vec<u8>, name: String, dir: &Path) {
    let path = dir.join(name);
    let _ = remove_file(&path);
    write(&path, data).expect("Could not write data to file");
}
