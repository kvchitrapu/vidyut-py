import os
import pytest
from pathlib import Path

from vidyut.kosha import Kosha, PartOfSpeech


@pytest.fixture(scope="module")
def kosha() -> Kosha:
    path = os.environ["VIDYUT_DATA_DIR"]
    assert path
    return Kosha(Path(path) / "kosha")


@pytest.mark.parametrize(
    "word",
    [
        # Basic verb
        "Bavati",
        # Vacana
        "Bavatas",
        "Bavanti",
        # Purusha
        "Bavasi",
        "BavAmi",
        # Lakaras
        "Bavati",
        "baBUva",
        "BavitA",
        "Bavizyati",
        "Bavatu",
        "aBavat",
        "Bavet",
        "BUyAt",
        "aBUt",
        "aBavizyat",
        # Nijantas
        "BAvayati",
        "BAvayitA",
        "BAvayizyati",
        "BAvayatu",
        "aBAvayat",
        "BAvayet",
        # sannantas
        "buBUzati",
        "buBUzitA",
        # Upasargas
        "praBavati",
        "praBAvayati",
        "prabuBUzati",
    ],
)
def test_contains_tinanta(kosha, word):
    entries = kosha.get_all(word)
    assert any(e.info.pos == PartOfSpeech.Tinanta for e in entries)


@pytest.mark.parametrize(
    "word",
    [
        # Basic subanta
        "Svetas",
        # Vacana
        "SvetO",
        "SvetAs",
        # Vibhakti
        "Svetam",
        "Svetena",
        "SvetAya",
        "SvetAt",
        "Svetasya",
        "Svete",
        "Sveta",
        # Linga
        "SvetA",
        "SvetAni",
        # Vowel stems
        "senayA",
        "agninA",
        "nadyA",
        "guruRA",
        "vaDvA",
        "pitrA",
        "kartrA",
        "rAyA",
        "gavA",
        "nAvA",
        # Consonant stems
        "yoginA",
        "AtmanA",
        "BagavatA",
        "vAcA",
    ],
)
def test_contains_subanta(kosha, word):
    entries = kosha.get_all(word)
    assert any(e.info.pos == PartOfSpeech.Subanta for e in entries)


@pytest.mark.parametrize(
    "word",
    [
        # Basic avyayas
        "ca",
        "tu",
        "eva",
        "hi",
        "taTA",
        "iti",
    ],
)
def test_contains_avyaya(kosha, word):
    entries = kosha.get_all(word)
    assert any(e.info.pos == PartOfSpeech.Avyaya for e in entries)
