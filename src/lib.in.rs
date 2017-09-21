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

#[derive(Serialize, Deserialize)]
pub enum Operation {
    Equal,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    NotEqual,
}

#[derive(Serialize, Deserialize)]
pub enum FieldsetType {
    Datetime,
}

#[derive(Serialize, Deserialize)]
pub enum FieldType {
    Date,
    Time,
    Day,
    Month,
    Year,
    TzHour,
    TzMinute,
}

#[derive(Serialize, Deserialize)]
pub struct Condition {
    pub operation: Operation,
    pub field: u32,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Sentence {
    pub id: String,
    pub field: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Multiplier {
    pub id: String,
    pub field: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Classifier {
    pub id: String,
    pub field: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Field {
    pub field_type: FieldType,
    pub value: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Fieldset {
    pub fieldset_type: FieldsetType,
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParameterGroup {
    pub pgn: u32,
    pub field: u32,
//    pub fieldset: Fieldset,
//    pub multiplier: Multiplier,
//    pub classifier: Classifier,
//    pub conditions: Vec<Condition>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParameterGroups {
    pub parameter_group: Vec<ParameterGroup>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Mapping {
    pub path: String,
    #[serde(rename="parameter_group")]
    pub parameter_groups: Vec<ParameterGroup>,
//    pub sentences: Vec<Sentence>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Mappings {
    pub mapping: Vec<Mapping>,
}