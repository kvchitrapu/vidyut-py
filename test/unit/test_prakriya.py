from vidyut.prakriya import (
    Ashtadhyayi,
    Dhatupatha,
    Krt,
    Lakara,
    Prayoga,
    Purusha,
    Sanadi,
    Vacana,
)


a = Ashtadhyayi()
# Path is relative to the project root.
d = Dhatupatha("test/data/test-dhatupatha.tsv")


def test_dhatupatha():
    bhu = d["01.0001"]
    assert bhu.upadesha == "BU"

    kr = d["08.0010"]
    assert kr.upadesha == "qukf\\Y"


def test_basic_kartari_tinantas():
    bhu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "Bavati"


def test_basic_karmani_tinantas():
    bhu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Karmani,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "BUyate"


def test_sannanta_tinantas():
    bhu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.San,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "buBUzati"


def test_nijanta_tinantas():
    bhu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Nic,
    )
    assert len(prakriyas) == 2
    results = {x.text for x in prakriyas}
    assert results == {"BAvayati", "BAvayate"}


def test_yananta_tinantas():
    bhu = d["01.0001"]
    prakriyas = a.derive_tinantas(
        dhatu=bhu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Yan,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "boBUyate"


def test_krdantas():
    bhu = d["01.0001"]
    prakriyas = a.derive_krdantas(
        dhatu=bhu,
        krt=Krt.ktvA,
    )
    assert len(prakriyas) == 1
    assert prakriyas[0].text == "BUtvA"
