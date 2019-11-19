file=${1:-"src/schema.rs"}

npm install -g quicktype

url=https://vega.github.io/schema/vega-lite/v3.4.0.json
escaped_url=`echo $url | sed 's#/#\\\/#g'`

echo '-- generating file from schema'
quicktype \
	--src $url \
	--src-lang schema \
	--out $file \
	--top-level Vegalite \
	--density dense	\
	--visibility public \
	--derive-debug

echo '-- remove extra comments'
sed -i '/^\/\/[^\/]*$/d' $file

echo '-- inserting license and lint'
cat << EOF > tmp_schema.rs
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(missing_docs, clippy::large_enum_variant)]

EOF
cat $file >> tmp_schema.rs
mv tmp_schema.rs $file

echo '-- fix serde import'
sed -i 's/extern crate serde_json;/use serde::{Deserialize, Serialize};/' $file

echo '-- set fields that have special meaning for null'
sed -i 's/use serde::/use crate::removable_value::RemovableValue;\nuse serde::/' $file
python3 scripts/if_null_fields.py $file tmp_schema.rs && mv tmp_schema.rs $file

echo '-- custom changes'
python3 scripts/custom_changes.py $file tmp_schema.rs && mv tmp_schema.rs $file

echo '-- skip serializing None by default'
sed -i 's/pub \(\w*\): Option/#[serde(skip_serializing_if = "Option::is_none")] pub \1: Option/' $file

echo '-- fix Box usage'
sed -i 's/Option<Box>/Option<DefBox>/' $file
sed -i 's/enum Box {/enum DefBox {/' $file

echo '-- make everything clonable and default'
sed -i 's/#\[derive(Debug, Serialize, Deserialize)\]/#[derive(Debug, Clone, Serialize, Deserialize)]/' $file
sed -i 's/pub struct/#[derive(Default)] pub struct/' $file

echo '-- setup builder'
sed -i 's/use serde::/use derive_builder::Builder;\nuse serde::/' $file
sed -i 's/pub struct/#[derive(Builder)] pub struct/' $file
sed -i 's/pub struct/#[builder(setter(into, strip_option))] pub struct/' $file
sed -i 's/pub \(\w*\): Option/#[builder(default)] pub \1: Option/' $file
sed -i "s/#\[builder(default)\] pub schema: Option/#[builder(default = \"Some(\\\\\"$escaped_url\\\\\".to_string())\")] pub schema: Option/" $file

sed -i 's/pub \(\w*\): \([^<]*\),$/#[serde(skip_serializing_if = "Option::is_none")] #[builder(default)] pub \1: Option<\2>,/' $file

echo '-- simplification'
sed -i 's/pub enum InlineDataset /#[allow(unused)]enum UnusedInlineDataset /' $file
sed -i 's/<InlineDataset>/<serde_json::value::Value>/' $file
sed -i 's/BoxPlotDefClass/MarkDefClass/g' $file
#sed -i 's/BoxPlotDefExtent/MarkDefExtent/' $file
sed -i 's/Enum(BoxPlot)/Enum(Mark)/' $file
sed -i 's/<BoxPlot>/<Mark>/' $file
sed -i 's/pub enum BoxPlot /pub enum Mark /' $file

sed -i 's/pub \(\w*\): Box<Option<\(\S*\)>>/#[serde(skip_serializing_if = "Option::is_none")] #[builder(default)] pub \1: Option<\2>/' $file
sed -i 's/pub filter: Option<Box<PurpleLogicalOperandPredicate>>,/pub filter: Option<PurpleLogicalOperandPredicate>,/' $file

cargo fmt -- $file

