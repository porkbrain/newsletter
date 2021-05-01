import re


def from_str(w):
    """
    Given a string it converts it to a list of floats.
    """
    assert isinstance(w, str), "Input must be string"
    assert len(w) != 0, "Input cannot be empty"

    features = [
        float(len(w)),
        float(w.islower()),
        float(w.isupper()),
        float(has_letters(w)),
        float(has_letters_only(w)),
        float(has_digits(w)),
        float(has_digits_only(w)),
        float(is_alphanumeric_or_dash_or_underscore(w)),
        float(ends_with_digit(w)),
        float(has_letters_which_end_with_two_digits(w)),
        float(is_in_english_dictionary(w)),
    ]

    for i in range(0, len(w)):
        features.append(map_char_to_feature(w[i]))

    return features


def has_letters(w):
    return bool(re.search("[a-zA-Z]", w))


def has_letters_only(w):
    return bool(re.search("^[a-zA-Z]+$", w))


def has_digits(w):
    return bool(re.search("\d", w))


def has_digits_only(w):
    return bool(re.search("^\d+$", w))


def is_alphanumeric_or_dash_or_underscore(w):
    return bool(re.search("^[a-zA-Z0-9-_]+$", w))


def ends_with_digit(w):
    return ord(w[-1]) in range(ord("0"), ord("9") + 1)


def has_letters_which_end_with_two_digits(w):
    return bool(re.search("^[a-zA-Z]+\d\d$", w))


def load_dict_en():
    """
    Loads dictionary from file. This should happen once on program boot.
    """
    print("Loading dict")
    dict_en = open("data/dictionary.en.txt", "r")

    words = set()

    for word in dict_en.readlines():
        words.add(word.strip().lower())

    return words


dict_en = load_dict_en()


def is_in_english_dictionary(w):
    return w.lower() in dict_en


def map_char_to_feature(c):
    c = ord(c)
    if c in range(ord("a"), ord("z") + 1):
        return 1.0
    if c in range(ord("A"), ord("Z") + 1):
        return 2.0
    if c in range(ord("0"), ord("9") + 1):
        return 3.0
    if c == ord("-"):
        return 4.0

    return 5.0


"""
Uncomment last line to run tests
"""


def test_module():
    assert has_letters("$_THERE")
    assert has_letters("there_123")
    assert not has_letters("123(*&^")

    assert has_letters_only("THERE")
    assert has_letters_only("there")
    assert not has_letters_only("there_123")

    assert has_digits("yes_123")
    assert not has_digits("no:(")

    assert has_digits_only("80082")
    assert not has_digits_only("no_123")

    assert is_alphanumeric_or_dash_or_underscore("123")
    assert is_alphanumeric_or_dash_or_underscore("1234abc")
    assert is_alphanumeric_or_dash_or_underscore("at_123")
    assert is_alphanumeric_or_dash_or_underscore("-123-_onlyleTTErs")
    assert not is_alphanumeric_or_dash_or_underscore("")
    assert not is_alphanumeric_or_dash_or_underscore("1a$")

    assert ends_with_digit("123")
    assert ends_with_digit("at_129")
    assert ends_with_digit("at_120")
    assert not ends_with_digit("1234abc")

    assert has_letters_which_end_with_two_digits("OFF20")
    assert not has_letters_which_end_with_two_digits("123hehe12")
    assert not has_letters_which_end_with_two_digits("23")
    assert not has_letters_which_end_with_two_digits("123")
    assert not has_letters_which_end_with_two_digits("123hehe1")

    assert is_in_english_dictionary("dog")
    assert is_in_english_dictionary("DOG")
    assert not is_in_english_dictionary("d4wg")
    assert not is_in_english_dictionary("nepravdepodobne")

    assert map_char_to_feature("a") == 1.0
    assert map_char_to_feature("z") == 1.0
    assert map_char_to_feature("A") == 2.0
    assert map_char_to_feature("Z") == 2.0
    assert map_char_to_feature("0") == 3.0
    assert map_char_to_feature("1") == 3.0
    assert map_char_to_feature("9") == 3.0
    assert map_char_to_feature("-") == 4.0
    assert map_char_to_feature("$") == 5.0
    assert map_char_to_feature("%") == 5.0

    assert from_str("OK20") == [
        4.0,
        0.0,
        1.0,
        1.0,
        0.0,
        1.0,
        0.0,
        1.0,
        1.0,
        1.0,
        0.0,
        2.0,
        2.0,
        3.0,
        3.0,
    ]


# uncomment temporarily when working with module
# test_module()
