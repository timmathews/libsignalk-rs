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
