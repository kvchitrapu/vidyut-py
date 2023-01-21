from dataclasses import dataclass
from typing import List, Dict

# vidyut-cheda
# ------------

@dataclass
class Token:
    text: str
    lemma: str
    info: Dict[str, str]

class Chedaka:
    def run(self, slp1_text: str) -> List[Token]:
        pass

# vidyut-kosha
# ------------

class Kosha:
    pass

# vidyut-prakriya
# ---------------

class Ashtadhyayi:
    pass

class Dhatupatha:
    pass
