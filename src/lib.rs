/* Copyright 2016 Tim Mathews (tim@signalk.org)
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 *     Unless required by applicable law or agreed to in writing, software
 *     distributed under the License is distributed on an "AS IS" BASIS,
 *     WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *     See the License for the specific language governing permissions and
 *     limitations under the License.
 */

extern crate serde_xml;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use serde_xml::{from_str, Error};

include!(concat!(env!("OUT_DIR"), "/lib.rs"));

impl Mappings {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Mappings, Error> {
        let mut file = File::open(path).expect("Could not open file");
        let mut buf = String::new();

        try!(file.read_to_string(&mut buf));

        from_str::<Mappings>(&buf)
    }
}

#[test]
fn test_from_file() {
    let m = Mappings::from_file("tests/samples/valid_two_mappings.xml");

    let expect = Mappings {
        mapping: vec![
            Mapping {
                path: "some/path".to_string(),
                parameter_groups: vec![
                    ParameterGroup {
                        pgn: 1,
                        field: 1,
                    },
                ],
            },
            Mapping {
                path: "some/other/path".to_string(),
                parameter_groups: vec![
                    ParameterGroup {
                        pgn: 2,
                        field: 2,
                    },
                ],
            },
        ],
    };

    match m {
        Ok(val) => { assert_eq!(val, expect) },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        }
    }
}
