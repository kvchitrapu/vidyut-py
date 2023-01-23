import pytest

from vidyut.prakriya import (
    Pratipadika,
)


def test_pratipadika_new():
    p = Pratipadika(text="deva")
    assert repr(p) == "Pratipadika(text='deva')"


def test_pratipadika_new__fails_if_no_args():
    with pytest.raises(TypeError):
        p = Pratipadika()


def test_pratipadika_new__fails_if_unnamed_args():
    with pytest.raises(TypeError):
        p = Pratipadika("deva")


def test_pratipadika_eq():
    p1 = Pratipadika(text="deva")
    assert p1 == p1

    p2 = Pratipadika(text="deva")
    assert p1 == p2

    p2 = Pratipadika(text="eva")
    assert p1 != p2
