"""
`vidyut.prakriya` contains code for generating Sanskrit words by using Paninian
rules. You can use it like so:

```python
from vidyut.prakriya import Ashtadhyayi, Dhatupatha

d = Dhatupatha("/path/to/vidyut-data/prakriya/dhatupatha.tsv")
bhu = d["01.0001"]

prakriyas = a.derive_tinantas(
    dhatu=dhatu,
    prayoga=Prayoga.Kartari,
    purusha=Purusha.Prathama,
    vacana=Vacana.Eka,
    lakara=Lakara.Lat,
)
assert len(prakriyas) == 1
assert prakriyas[0].text == "Bavati"
```
"""
from vidyut.vidyut import prakriya as __mod

Ashtadhyayi = __mod.Ashtadhyayi
Dhatupatha = __mod.Dhatupatha

Krt = __mod.Krt
Lakara = __mod.Lakara
Prayoga = __mod.Prayoga
Purusha = __mod.Purusha
Sanadi = __mod.Sanadi
Vacana = __mod.Vacana
