from treehouse.search import binary_search, binary_search_recursion, linear_search

numbers = [0, 5, 7, 10, 15]


def test_linear_search():
    assert linear_search(numbers, 0) == 0
    assert linear_search(numbers, 15) == 4
    assert linear_search(numbers, 5) == 1
    assert linear_search(numbers, 6) is None


def test_binary_search():
    assert binary_search(numbers, 0) == 0
    assert binary_search(numbers, 15) == 4
    assert binary_search(numbers, 5) == 1
    assert binary_search(numbers, 6) is None


def test_binary_search_recursion():
    assert binary_search_recursion(numbers, 0, 0, 4) == 0
    assert binary_search_recursion(numbers, 15, 0, 4) == 4
    assert binary_search_recursion(numbers, 5, 0, 4) == 1
    assert binary_search_recursion(numbers, 11, 0, 4) is None
    assert binary_search_recursion(numbers, 4, 0, 4) is None
    assert binary_search_recursion(numbers, 8, 0, 4) is None
