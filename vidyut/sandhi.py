"""
`vidyut.sandhi` contains various utilities for working with sandhi changes
between words. It is fast, simple, and appropriate for most use cases. You can
use it like so:

```python
from vidyut.sandhi import Splitter

splitter = Splitter("/path/to/vidyut-data/sandhi-rules.csv")
for split in splitter.split("ityapi", 2):
    print(split.first, split.second, split.is_valid)
```
"""
from vidyut.vidyut import sandhi as __mod

Splitter = __mod.Splitter
Split = __mod.Split
