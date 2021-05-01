import re


def normalize_phrase(phrase):
    """
    Given a string, it strips or transforms it so that it can be used with a
    vectorizer.
    """

    assert isinstance(phrase, str)

    phrase = phrase.lower()
    phrase = re.sub(r"\$", " USD ", phrase)
    phrase = re.sub(r"£", " GBP ", phrase)
    phrase = re.sub(r"€", " EUR ", phrase)
    phrase = re.sub(r"%", " PERCENT ", phrase)
    phrase = re.sub(r"\W", " ", phrase)
    phrase = re.sub(r"\s+[a-z]\s+", " ", phrase)
    phrase = re.sub(r"\^[a-z]\s+", " ", phrase)
    phrase = re.sub(r"\s+", " ", phrase)
    phrase = re.sub(r"^b\s+", "", phrase)

    return phrase
