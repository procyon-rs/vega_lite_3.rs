import sys
import re

changes = [
    {
        'struct': 'SpecClass',
        'from': r'pub spec: Box<Option<SpecClass>>',
        'to': r'pub spec: Option<Box<SpecClass>>'
    },
    {
        'struct': 'ViewConfig',
        'from': r'pub stroke: Option<String>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub stroke: RemovableValue<String>'
    },
    {
        'struct': 'Encoding',
        'from': r'pub tooltip: Option<Tooltip>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub tooltip: RemovableValue<Tooltip>'
    },
    {
        'struct': 'BaseMarkConfig',
        'from': r'pub tooltip: Option<serde_json::Value>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub tooltip: RemovableValue<serde_json::Value>'
    },
    {
        'struct': 'LayerEncoding',
        'from': r'pub tooltip: Option<Tooltip>',
        'to': r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub tooltip: RemovableValue<Tooltip>'
    },
]

change_is_done = {}

with open(sys.argv[1], 'r', encoding="utf-8", errors='replace') as f:
    with open(sys.argv[2], 'w+', encoding="utf-8", errors='replace') as t:
        for line in f:
            rewrote = False
            for (change_id, change) in enumerate(changes):
                if "struct " + change["struct"] + " {" in line:
                    change_is_done[change_id] = False
                if change_id in change_is_done and not change_is_done[change_id] and re.search(change["from"], line):
                    print("rewrote " + change["struct"] + " field")
                    t.write(re.sub(change["from"], change["to"], line))
                    change_is_done[change_id] = True
                    rewrote = True
            if not rewrote:
                t.write(line)
