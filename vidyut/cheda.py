"""
`vidyut.cheda` contains code for segmenting Sanskrit expressions. You can use
it like so:

```python
from vidyut.cheda import Chedaka

chedaka = Chedaka("/path/to/vidyut-data")
print(chedaka.run("rAjA yajate"))
```
"""
from vidyut.vidyut import Chedaka
