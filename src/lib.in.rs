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

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum Operation {
    Equal,
    LessThan,
    GreaterThan,
    LessOrEqual,
    GreaterOrEqual,
    NotEqual,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FieldsetType {
    #[serde(rename="datetime")]
    Datetime,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum FieldType {
    #[serde(rename="date")]
    Date,
    #[serde(rename="time")]
    Time,
    #[serde(rename="day")]
    Day,
    #[serde(rename="month")]
    Month,
    #[serde(rename="year")]
    Year,
    #[serde(rename="tzHour")]
    TzHour,
    #[serde(rename="tzMinute")]
    TzMinute,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Condition {
    pub operation: Operation,
    pub field: u32,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Sentence {
    pub id: String,
    pub field: Option<u32>,
    pub fieldset: Option<Fieldset>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Multiplier {
    pub id: String,
    pub field: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Classifier {
    pub id: String,
    pub field: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Field {
    #[serde(rename="type")]
    pub field_type: FieldType,
    #[serde(rename="$value")]
    pub value: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Fieldset {
    #[serde(rename="type")]
    pub fieldset_type: FieldsetType,
    #[serde(rename="field")]
    pub fields: Vec<Field>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ParameterGroup {
    pub pgn: u32,
    pub field: Option<u32>,
    pub fieldset: Option<Fieldset>,
    pub multiplier: Option<Multiplier>,
    pub classifier: Option<Classifier>,
    pub conditions: Option<Vec<Condition>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Mapping {
    pub path: String,
    #[serde(rename="parameter_group")]
    pub parameter_groups: Option<Vec<ParameterGroup>>,
    #[serde(rename="sentence")]
    pub sentences: Option<Vec<Sentence>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Mappings {
    pub mapping: Vec<Mapping>,
}

// Signal K JSON Types
#[derive(Serialize, Deserialize)]
pub struct Source {
    pub pgn: u32,
    pub device: String,
    pub src: u8,
}

#[derive(Serialize, Deserialize)]
pub enum JsonValue {
    Integer(i64),
    Double(f64),
    String,
    Boolean(bool),
}

#[derive(Serialize, Deserialize)]
pub struct Value {
    pub path: String,
    pub value: JsonValue,
}

#[derive(Serialize, Deserialize)]
pub struct Update {
    pub source: Source,
    pub timestamp: DateTime<Utc>,
    pub values: Vec<Value>,
}

pub struct Delta {
    pub context: String,
    pub updates: Vec<Update>,
}
