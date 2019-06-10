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
//use vega_lite::Spec;
use crate::Vegalite;
use serde_json;
use showata::ContentInfo;
use showata::Showable;
use failure::Error;

impl Showable for Vegalite {
    fn to_content_info(&self) -> Result<ContentInfo, Error> {
        // vega3-extension: .vg, .vl, .vg.json, .vl.json, .vega, .vegalite
        // mime-types: 	application/vnd.vega.v3+json, application/vnd.vegalite.v2+json
        let content = serde_json::to_string(self)?;
        Ok(ContentInfo{
            content,
            mime_type: "application/vnd.vegalite.v3+json".into(),
        })
    }

    // TODO for html use [vega/vega-embed: Publish Vega visualizations as embedded web components with interactive parameters.](https://github.com/vega/vega-embed)
    // TODO add an Config parameter (with config for to json str, config for embed)
    fn to_html_page(&self) -> Result<String, Error> {
        let dod = self.to_content_info()?;
        let content = VEGA_EMBED_HTML_TEMPLATE.replace("{{ spec_as_json }}", &dod.content);
        Ok(content.into())
    }

}

const VEGA_EMBED_HTML_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <!-- Import Vega 5 & Vega-Lite 3 (does not have to be from CDN) -->
  <script src="https://cdn.jsdelivr.net/npm/vega@5"></script>
  <script src="https://cdn.jsdelivr.net/npm/vega-lite@3"></script>
  <!-- Import vega-embed -->
  <script src="https://cdn.jsdelivr.net/npm/vega-embed@4"></script>
</head>
<body>

<div id="vis"></div>

<script type="text/javascript">
  var spec = {{ spec_as_json }};
  vegaEmbed('#vis', spec).then(function(result) {
    // Access the Vega view instance (https://vega.github.io/vega/docs/api/view/) as result.view
  }).catch(console.error);
</script>
</body>
</html>
"#;
