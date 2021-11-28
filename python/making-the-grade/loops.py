def round_scores(student_scores):
    """Rounding Scores,

    :param student_scores: list of student exam scores as float or int.
    :return: list of student scores *rounded* to nearest integer value.
    """

    return [round(s) for s in student_scores]


def count_failed_students(student_scores):
    """Non-Passing Students.

    :param student_scores: list of integer student scores.
    :return: integer count of student scores at or below 40.
    """

    return len([s for s in student_scores if s <= 40])


def above_threshold(student_scores, threshold):
    """The best.

    :param student_scores: list of integer scores
    :param threshold :  integer
    :return: list of integer scores that are at or above the "best" threshold.
    """

    return [s for s in student_scores if s >= threshold]


def letter_grades(highest):
    """Calculating Letter Grades.

    :param highest: integer of highest exam score.
    :return: list of integer lower threshold scores for each D-A letter grade interval.
             For example, where the highest score is 100, and failing is <= 40,
             The result would be [41, 56, 71, 86]:

             41 <= "D" <= 55
             56 <= "C" <= 70
             71 <= "B" <= 85
             86 <= "A" <= 100
    """

    step = (highest - 40) // 4

    return [41 + step * n for n in range(0, 4)]


def student_ranking(student_scores, student_names):
    """Matching Names to Scores.

    :param student_scores: list of scores in descending order.
    :param student_names: list of names in descending order by exam score.
    :return: list of strings in format ["<rank>. <student name>: <score>"].
    """

    return [
        f"{i+1}. {name}: {score}"
        for i, (name, score) in enumerate(zip(student_names, student_scores))
    ]


def perfect_score(student_info):
    """A perfect score.

    :param student_info: list of [<student name>, <score>] lists
    :return: first `[<student name>, 100]` or `[]` if no student score of 100 is found.
    """

    return next(iter([s for s in student_info if s[1] == 100]), [])
