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

extern crate serde_xml_rs;
extern crate libsignalk;

use libsignalk::*;

use serde_xml_rs::from_str;

#[test]
fn read_xml() {
    let xml = "
        <mapping>
            <path>some/path</path>
            <parameter_group>
                <pgn>123456</pgn>
                <field>1</field>
            </parameter_group>
            <parameter_group>
                <pgn>234567</pgn>
                <field>2</field>
            </parameter_group>
            <sentence>
                <id>Foo</id>
                <field>1</field>
            </sentence>
        </mapping>
    ";

    let expect = Mapping {
        path: "some/path".to_string(),
        parameter_groups: Some(vec![
            ParameterGroup {
                pgn: 123456,
                field: Some(1),
                fieldset: None,
                multiplier: None,
                classifier: None,
                conditions: None,
            },
            ParameterGroup {
                pgn: 234567,
                field: Some(2),
                fieldset: None,
                multiplier: None,
                classifier: None,
                conditions: None,
            },
        ]),
        sentences: Some(vec![
            Sentence {
                id: "Foo".to_string(),
                field: Some(1),
                fieldset: None,
            },
        ]),
    };

    let v = from_str::<Mapping>(xml);

    match v {
        Ok(val) => {
            assert_eq!(val, expect)
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}

#[test]
fn read_mappings() {
    let xml = r##"
        <mappings>
            <mapping>
                <path>some/path</path>
                <parameter_group>
                    <pgn>1</pgn>
                    <fieldset type="datetime">
                        <field type="date">2</field>
                        <field type="time">3</field>
                    </fieldset>
                </parameter_group>
            </mapping>
            <mapping>
                <path>some/other/path</path>
                <parameter_group>
                    <pgn>2</pgn>
                    <field>2</field>
                </parameter_group>
            </mapping>
        </mappings>
    "##;

    let expect = Mappings {
        mapping: vec![
            Mapping {
                path: "some/path".to_string(),
                parameter_groups: Some(vec![
                    ParameterGroup {
                        pgn: 1,
                        field: None,
                        fieldset: Some(Fieldset {
                            fieldset_type: FieldsetType::Datetime,
                            fields: vec![
                                Field {
                                    field_type: FieldType::Date,
                                    value: 2,
                                },
                                Field {
                                    field_type: FieldType::Time,
                                    value: 3,
                                },
                            ],
                        }),
                        multiplier: None,
                        classifier: None,
                        conditions: None,
                    },
                ]),
                sentences: None,
            },
            Mapping {
                path: "some/other/path".to_string(),
                parameter_groups: Some(vec![
                    ParameterGroup {
                        pgn: 2,
                        field: Some(2),
                        fieldset: None,
                        multiplier: None,
                        classifier: None,
                        conditions: None,
                    },
                ]),
                sentences: None,
            },
        ],
    };

    let v = from_str::<Mappings>(xml);

    match v {
        Ok(val) => {
            assert_eq!(val, expect)
        },
        Err(err) => {
            println!("{}", err);
            assert!(false)
        },
    }
}
