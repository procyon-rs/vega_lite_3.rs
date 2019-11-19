import sys
import re

change_next_field = False

with open(sys.argv[1], 'r', encoding="utf-8", errors='replace') as f:
    with open(sys.argv[2], 'w+', encoding="utf-8", errors='replace') as t:
        for line in f:
            if "/// " in line and 'If `null`,' in line:
                change_next_field = True
            if "/// " in line and 'If set to `null`,' in line:
                change_next_field = True
            if "/// " in line and 'Set to null to disable' in line:
                change_next_field = True
            if "/// " in line and 'Set to `null`' in line:
                change_next_field = True
            if "/// " in line and ', or `null`' in line:
                change_next_field = True
            if "/// " in line and '`null` indicating no sort.' in line:
                change_next_field = True

            if change_next_field and re.search("^[^/]*pub \w*: Option<", line):
                t.write(re.sub(r"pub (\w*): Option<",
                    r'#[serde(default, skip_serializing_if = "RemovableValue::is_default")] #[builder(default)] pub \1: RemovableValue<',
                    line
                ))
                change_next_field = False
            else:
                t.write(line)

