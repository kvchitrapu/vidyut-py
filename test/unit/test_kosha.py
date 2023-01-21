import tempfile

import pytest


from vidyut.kosha import (
    Builder,
    Kosha,
    Purusha,
    Vacana,
    Pada,
    PadaPrayoga,
    Pratipadika,
    PartOfSpeech,
    Lakara,
    Linga,
    Vibhakti,
    Dhatu,
)


@pytest.fixture(scope="session")
def kosha():
    """Create a sample Kosha."""
    gacchati_tin = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    gacchati_sup = Pada.make_subanta(
        pratipadika=Pratipadika(text="gam"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    with tempfile.TemporaryDirectory() as tempdir:
        b = Builder(tempdir)
        b.insert("gacCati", gacchati_tin)
        b.insert("gacCati", gacchati_sup)
        b.finish()

        return Kosha(tempdir)


def test_dhatu():
    gam = Dhatu(text="gam")
    assert gam.text == "gam"


def test_dhatu__eq():
    gam = Dhatu(text="gam")
    assert gam == gam
    assert gam != Dhatu(text="BU")


def test_dhatu__repr():
    dhatu = Dhatu(text="gam")
    assert repr(dhatu) == "Dhatu(text='gam')"
    assert str(dhatu) == "gam"


def test_pratipadika():
    deva = Pratipadika(text="deva")
    assert deva.text == "deva"


def test_pratipadika__eq():
    deva = Pratipadika(text="deva")
    assert deva == deva
    assert deva != Pratipadika(text="svarga")


def test_pratipadika__repr():
    p = Pratipadika(text="deva")
    assert repr(p) == "Pratipadika(text='deva')"
    assert str(p) == "deva"


def test_pada__make_tinanta():
    p = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    assert p.dhatu == Dhatu(text="gam")
    assert p.purusha == Purusha.Prathama
    assert p.lakara == Lakara.Lat
    assert p.vacana == Vacana.Eka
    assert p.pada_prayoga == PadaPrayoga.Parasmaipada

    assert p.linga is None
    assert p.vibhakti is None


def test_pada__make_subanta():
    p = Pada.make_subanta(
        pratipadika=Pratipadika(text="deva"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    assert p.pratipadika == Pratipadika(text="deva")
    assert p.linga == Linga.Pum
    assert p.vacana == Vacana.Eka
    assert p.vibhakti == Vibhakti.V7
    assert not p.is_purvapada

    assert p.purusha is None
    assert p.lakara is None


def test_pada__make_avyaya():
    p = Pada.make_avyaya(
        pratipadika=Pratipadika(text="ca"),
    )

    assert p.pratipadika == Pratipadika(text="ca")
    assert p.linga is None
    assert p.vacana is None
    assert p.vibhakti is None
    assert p.purusha is None
    assert p.lakara is None
    assert not p.is_purvapada


def test_end_to_end():
    gacchati_tin = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    gacchati_sup = Pada.make_subanta(
        pratipadika=Pratipadika(text="gam"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    with tempfile.TemporaryDirectory() as tempdir:
        b = Builder(tempdir)
        b.insert("gacCati", gacchati_tin)
        b.insert("gacCati", gacchati_sup)
        b.finish()

        kosha = Kosha(tempdir)


def test_kosha__contains(kosha):
    assert "Bavati" not in kosha
    assert "gacCati" in kosha


def test_kosha__contains_prefix(kosha):
    for prefix in ["", "g", "ga", "gac", "gacC", "gacCa", "gacCat", "gacCati"]:
        assert kosha.contains_prefix(prefix)

    assert not kosha.contains_prefix("gacCati2")


def test_kosha__get_all(kosha):
    gacchati_tin = Pada.make_tinanta(
        dhatu=Dhatu(text="gam"),
        purusha=Purusha.Prathama,
        lakara=Lakara.Lat,
        vacana=Vacana.Eka,
        pada_prayoga=PadaPrayoga.Parasmaipada,
    )

    gacchati_sup = Pada.make_subanta(
        pratipadika=Pratipadika(text="gam"),
        linga=Linga.Pum,
        vibhakti=Vibhakti.V7,
        vacana=Vacana.Eka,
    )

    assert len(kosha.get_all("Bavati")) == 0
    assert len(kosha.get_all("gacCati")) == 2

    [tin, sup] = kosha.get_all("gacCati")
    assert tin.text == "gacCati"
    assert tin.info == gacchati_tin

    assert sup.text == "gacCati"
    assert sup.info == gacchati_sup
