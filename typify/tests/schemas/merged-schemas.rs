#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[doc = "ButNotThat"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"not\": {"]
#[doc = "    \"required\": ["]
#[doc = "      \"that\""]
#[doc = "    ]"]
#[doc = "  },"]
#[doc = "  \"properties\": {"]
#[doc = "    \"that\": {},"]
#[doc = "    \"this\": {}"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ButNotThat {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub this: Option<serde_json::Value>,
}
impl From<&ButNotThat> for ButNotThat {
    fn from(value: &ButNotThat) -> Self {
        value.clone()
    }
}
#[doc = "JsonResponseBase"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"result\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonResponseBase {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}
impl From<&JsonResponseBase> for JsonResponseBase {
    fn from(value: &JsonResponseBase) -> Self {
        value.clone()
    }
}
#[doc = "JsonSuccess"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/components/schemas/JsonSuccessBase\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"properties\": {"]
#[doc = "        \"msg\": {},"]
#[doc = "        \"result\": {}"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct JsonSuccess {
    pub msg: String,
    pub result: JsonSuccessResult,
}
impl From<&JsonSuccess> for JsonSuccess {
    fn from(value: &JsonSuccess) -> Self {
        value.clone()
    }
}
#[doc = "x"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"x\","]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/components/schemas/JsonResponseBase\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"required\": ["]
#[doc = "        \"msg\","]
#[doc = "        \"result\""]
#[doc = "      ],"]
#[doc = "      \"properties\": {"]
#[doc = "        \"msg\": {"]
#[doc = "          \"type\": \"string\""]
#[doc = "        },"]
#[doc = "        \"result\": {"]
#[doc = "          \"enum\": ["]
#[doc = "            \"success\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JsonSuccessBase {
    pub msg: String,
    pub result: JsonSuccessBaseResult,
}
impl From<&JsonSuccessBase> for JsonSuccessBase {
    fn from(value: &JsonSuccessBase) -> Self {
        value.clone()
    }
}
#[doc = "JsonSuccessBaseResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"success\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JsonSuccessBaseResult {
    #[serde(rename = "success")]
    Success,
}
impl From<&JsonSuccessBaseResult> for JsonSuccessBaseResult {
    fn from(value: &JsonSuccessBaseResult) -> Self {
        value.clone()
    }
}
impl ToString for JsonSuccessBaseResult {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "success".to_string(),
        }
    }
}
impl std::str::FromStr for JsonSuccessBaseResult {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "success" => Ok(Self::Success),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for JsonSuccessBaseResult {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for JsonSuccessBaseResult {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for JsonSuccessBaseResult {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "JsonSuccessResult"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"success\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum JsonSuccessResult {
    #[serde(rename = "success")]
    Success,
}
impl From<&JsonSuccessResult> for JsonSuccessResult {
    fn from(value: &JsonSuccessResult) -> Self {
        value.clone()
    }
}
impl ToString for JsonSuccessResult {
    fn to_string(&self) -> String {
        match *self {
            Self::Success => "success".to_string(),
        }
    }
}
impl std::str::FromStr for JsonSuccessResult {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "success" => Ok(Self::Success),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for JsonSuccessResult {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for JsonSuccessResult {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for JsonSuccessResult {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "NarrowNumber"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"integer\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"minimum\": 1.0"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NarrowNumber(pub std::num::NonZeroU64);
impl std::ops::Deref for NarrowNumber {
    type Target = std::num::NonZeroU64;
    fn deref(&self) -> &std::num::NonZeroU64 {
        &self.0
    }
}
impl From<NarrowNumber> for std::num::NonZeroU64 {
    fn from(value: NarrowNumber) -> Self {
        value.0
    }
}
impl From<&NarrowNumber> for NarrowNumber {
    fn from(value: &NarrowNumber) -> Self {
        value.clone()
    }
}
impl From<std::num::NonZeroU64> for NarrowNumber {
    fn from(value: std::num::NonZeroU64) -> Self {
        Self(value)
    }
}
impl std::str::FromStr for NarrowNumber {
    type Err = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.parse()?))
    }
}
impl std::convert::TryFrom<&str> for NarrowNumber {
    type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for NarrowNumber {
    type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for NarrowNumber {
    type Error = <std::num::NonZeroU64 as std::str::FromStr>::Err;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}
impl ToString for NarrowNumber {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[doc = "TrimFat"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"not\": {"]
#[doc = "    \"anyOf\": ["]
#[doc = "      {"]
#[doc = "        \"required\": ["]
#[doc = "          \"b\""]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      {"]
#[doc = "        \"required\": ["]
#[doc = "          \"c\""]
#[doc = "        ]"]
#[doc = "      }"]
#[doc = "    ]"]
#[doc = "  },"]
#[doc = "  \"required\": ["]
#[doc = "    \"a\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"a\": {},"]
#[doc = "    \"b\": {},"]
#[doc = "    \"c\": {}"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrimFat {
    pub a: serde_json::Value,
}
impl From<&TrimFat> for TrimFat {
    fn from(value: &TrimFat) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable1"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"string\","]
#[doc = "      \"enum\": ["]
#[doc = "        \"foo\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"bar\": {}"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Unsatisfiable1 {}
impl From<&Unsatisfiable1> for Unsatisfiable1 {
    fn from(value: &Unsatisfiable1) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable2"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"foo\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"type\": \"string\","]
#[doc = "          \"enum\": ["]
#[doc = "            \"bar\""]
#[doc = "          ]"]
#[doc = "        }"]
#[doc = "      },"]
#[doc = "      \"additionalProperties\": false"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Unsatisfiable2 {}
impl From<&Unsatisfiable2> for Unsatisfiable2 {
    fn from(value: &Unsatisfiable2) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"allOf\": ["]
#[doc = "    {"]
#[doc = "      \"$ref\": \"#/definitions/unsatisfiable-3-a\""]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"properties\": {"]
#[doc = "        \"action\": {"]
#[doc = "          \"$ref\": \"#/definitions/unsatisfiable-3-b\""]
#[doc = "        }"]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsatisfiable3 {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Unsatisfiable3Action>,
}
impl From<&Unsatisfiable3> for Unsatisfiable3 {
    fn from(value: &Unsatisfiable3) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3A"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"action\": {"]
#[doc = "      \"allOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/unsatisfiable-3-c\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Unsatisfiable3A {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<Unsatisfiable3C>,
}
impl From<&Unsatisfiable3A> for Unsatisfiable3A {
    fn from(value: &Unsatisfiable3A) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3Action"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "false"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub enum Unsatisfiable3Action {}
impl From<&Unsatisfiable3Action> for Unsatisfiable3Action {
    fn from(value: &Unsatisfiable3Action) -> Self {
        value.clone()
    }
}
#[doc = "Unsatisfiable3B"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"bar\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Unsatisfiable3B {
    #[serde(rename = "bar")]
    Bar,
}
impl From<&Unsatisfiable3B> for Unsatisfiable3B {
    fn from(value: &Unsatisfiable3B) -> Self {
        value.clone()
    }
}
impl ToString for Unsatisfiable3B {
    fn to_string(&self) -> String {
        match *self {
            Self::Bar => "bar".to_string(),
        }
    }
}
impl std::str::FromStr for Unsatisfiable3B {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "bar" => Ok(Self::Bar),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Unsatisfiable3B {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Unsatisfiable3B {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Unsatisfiable3B {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "Unsatisfiable3C"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"foo\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum Unsatisfiable3C {
    #[serde(rename = "foo")]
    Foo,
}
impl From<&Unsatisfiable3C> for Unsatisfiable3C {
    fn from(value: &Unsatisfiable3C) -> Self {
        value.clone()
    }
}
impl ToString for Unsatisfiable3C {
    fn to_string(&self) -> String {
        match *self {
            Self::Foo => "foo".to_string(),
        }
    }
}
impl std::str::FromStr for Unsatisfiable3C {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "foo" => Ok(Self::Foo),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for Unsatisfiable3C {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for Unsatisfiable3C {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for Unsatisfiable3C {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
#[doc = "WeirdEnum"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"oneOf\": ["]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"patterns\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-either\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-regex\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"pattern\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-either\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-regex\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"patterns\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"patterns\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-regex\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"pattern-either\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    {"]
#[doc = "      \"not\": {"]
#[doc = "        \"anyOf\": ["]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"patterns\""]
#[doc = "            ]"]
#[doc = "          },"]
#[doc = "          {"]
#[doc = "            \"required\": ["]
#[doc = "              \"pattern-either\""]
#[doc = "            ]"]
#[doc = "          }"]
#[doc = "        ]"]
#[doc = "      },"]
#[doc = "      \"required\": ["]
#[doc = "        \"pattern-regex\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"pattern\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pattern-either\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"pattern-regex\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"patterns\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum WeirdEnum {
    Variant0 {
        pattern: String,
    },
    Variant1 {
        patterns: String,
    },
    Variant2 {
        #[serde(rename = "pattern-either")]
        pattern_either: String,
    },
    Variant3 {
        #[serde(rename = "pattern-regex")]
        pattern_regex: String,
    },
}
impl From<&WeirdEnum> for WeirdEnum {
    fn from(value: &WeirdEnum) -> Self {
        value.clone()
    }
}
fn main() {}
