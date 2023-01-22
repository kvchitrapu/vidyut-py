import pytest

from vidyut.prakriya import (
    Ashtadhyayi,
    Dhatupatha,
    Krt,
    Lakara,
    Prayoga,
    Pratipadika,
    Linga,
    Vibhakti,
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


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "Bavati"),
        ("08.0010", "karoti|kurute"),
    ],
)
def test_basic_kartari_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUyate"),
        ("08.0010", "kriyate"),
    ],
)
def test_basic_karmani_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Karmani,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "buBUzati"),
        # ("08.0010", "cikIrzati"),
    ],
)
def test_sannanta_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.San,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BAvayati|BAvayate"),
        ("08.0010", "kArayati|kArayate"),
    ],
)
def test_nijanta_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Nic,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "boBUyate"),
        # ("08.0010", "cekrIyate"),
    ],
)
def test_yananta_tinantas(code, expected):
    dhatu = d[code]
    prakriyas = a.derive_tinantas(
        dhatu=dhatu,
        prayoga=Prayoga.Kartari,
        purusha=Purusha.Prathama,
        vacana=Vacana.Eka,
        lakara=Lakara.Lat,
        sanadi=Sanadi.Yan,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


@pytest.mark.parametrize(
    "code,expected",
    [
        ("01.0001", "BUtvA"),
        ("08.0010", "kftvA"),
    ],
)
def test_krdantas(code, expected):
    dhatu = d[code]
    prakriyas = a.derive_krdantas(
        dhatu=dhatu,
        krt=Krt.ktvA,
    )
    expected = set(expected.split("|"))
    actual = {x.text for x in prakriyas}
    assert expected == actual


def test_subantas():
    prakriyas = a.derive_subantas(
        pratipadika=Pratipadika(text="deva"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.Prathama,
        vacana=Vacana.Eka,
    )
    expected = {"devaH"}
    actual = {x.text for x in prakriyas}
    assert expected == actual
