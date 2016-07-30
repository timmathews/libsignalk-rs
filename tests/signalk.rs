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

extern crate serde;
extern crate serde_xml;
extern crate libsignalk;

use libsignalk::{ Mapping, ParameterGroup };

use serde_xml::from_str;

#[test]
fn read_xml() {
    let xml = "
        <mapping>
            <path>some/path</path>
            <parameter_group>
                <pgn>123456</pgn>
                <field>1</field>
            </parameter_group>
        </mapping>
    ";

    let mut expect = Mapping {
        path: "some/path".to_string(),
        parameter_groups: Vec::new(),
    };

    expect.parameter_groups.push(ParameterGroup {
        pgn: 123456,
        field: 1,
    });

    let v = from_str::<Mapping>(xml);

    match v {
        Ok(val) => {
            assert_eq!(val, expect);
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}
