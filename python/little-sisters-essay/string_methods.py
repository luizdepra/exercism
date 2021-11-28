def capitalize_title(title):
    """Capitalize the title of the paper.

    :param title: str title string that needs title casing
    :return:  str title string in title case (first letters capitalized)
    """

    return title.title()


def check_sentence_ending(sentence):
    """Check if each sentence ends with a period.

    :param sentence: str a sentence to check.
    :return:  bool True if punctuated correctly with period, False otherwise.
    """

    return sentence.endswith(".")


def clean_up_spacing(sentence):
    """Clean up spacing.

    :param sentence: str a sentence to clean of leading and trailing space characters.
    :return: str a sentence that has been cleaned of leading and trailing space characters.
    """

    return sentence.strip()


def replace_word_choice(sentence, old_word, new_word):
    """Replace words with a synonym.

    :param sentence: str a sentence to replace words in.
    :param old_word: str word to replace
    :param new_word: str replacement word
    :return:  str input sentence with new words in place of old words
    """

    return sentence.replace(old_word, new_word)
