"""
`vidyut.kosha` contains code for compactly storing Sanskrit words. You can use
it like so:

```python
from vidyut.kosha import Kosha

kosha = Kosha("/path/to/vidyut-data/kosha")
print(kosha.get("gacCati"))
```
"""
from vidyut.vidyut import kosha as __mod

Builder = __mod.Builder
Kosha = __mod.Kosha

Dhatu = __mod.Dhatu
Lakara = __mod.Lakara
Linga = __mod.Linga
PadaPrayoga = __mod.PadaPrayoga
Pratipadika = __mod.Pratipadika
Pada = __mod.Pada
PartOfSpeech = __mod.PartOfSpeech
Purusha = __mod.Purusha
Vacana = __mod.Vacana
Vibhakti = __mod.Vibhakti
