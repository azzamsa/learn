from typing import Optional


def linear_search(numbers: list[int], target: int) -> Optional[int]:
    """Search a target from the input using linear search algorithm

    :param numbers: a collection with comparable items (not required to be sorted)
    :param target: item value to search
    :return: index of found item or None if item is not found
    """
    for index, value in enumerate(numbers):
        if value == target:
            return index
    return None


def binary_search(numbers: list[int], target: int) -> Optional[int]:
    """Search a target from the input using binary search algorithm

    :param numbers: a collection with comparable items (must be sorted)
    :param target: item value to search
    :return: index of found item or None if item is not found
    """
    first_index = 0
    last_index = len(numbers) - 1

    while first_index <= last_index:
        mid_index = (last_index + first_index) // 2

        # best case
        item = numbers[mid_index]
        if item == target:
            return mid_index

        if item < target:
            # discard all numbers BEFORE mid_index
            first_index = mid_index + 1
        else:
            # discard all numbers AFTER mid_index
            last_index = mid_index - 1

    return None


def binary_search_recursion(
    numbers: list[int], target: int, first_index: int, last_index: int
) -> Optional[int]:
    """Search a target from the input using binary search algorithm

    Here we need to avoid changing the original `numbers` such
    `return binary_search_recursion(numbers[first_index:], target)`.
    It will change the length of the `numbers` as input and will return the wrong
    index in the last iteration.

    The steps for non-existing target is that the function keep changing it's first and
    last index. When both in the same position and the mid value does not equal the target,
    the last index decremented. Because the current mid value is greater that the target.
    Then in this condition (last index < first index) the recursion stopped.

    The last condition for non-existing target will always be the same. Given the
    `number = [0, 5, 7, 10, 15]`. Both 4 and 11 as the target will always be resulted
    in similar step. In the former case, the first and the last index will be in (1, 1).
    Thus resulting in (1,0) because 5 is greater than the target. In the latter case,
    the both the index will be in (4, 4). Then it will become (4, 3), because 15 is
    greater than the target.

    Illustrations:
    Note: index: (first, mid, last)

    1. list: [0, 5, 7, 10, 15]   ; index: (0, 2, 4)  ; target: 4
    2. list: [0, 5, *,  *,  *]   ; index: (0, 0, 1)
    3. list: [*, 5, *,  *,  *]   ; index: (1, 1, 1)
    4. stopped                   ; index: (1, *, 0)

    1. list: [0, 5, 7, 10, 15]   ; index: (0, 2, 4)  ; target: 11
    2. list: [*, *, *, 10, 15]   ; index: (3, 3, 4)
    3. list: [*, *, *,  *, 15]   ; index: (4, 4, 4)
    4. stopped                   ; index: (4, *, 3)

    1. list: [0, 5, 7, 10, 15]   ; index: (0, 2, 4)  ; target: 8
    2. list: [*, *, *, 10, 15]   ; index: (3, 3, 4)
    3. stopped                   ; index: (3, *, 2)

    :param numbers: a collection with comparable items (must be sorted)
    :param target: item value to search
    :return: index of found item or None if item is not found
    """
    # otherwise the recursion runs forever
    if last_index < first_index:
        return None

    mid_index = (last_index + first_index) // 2

    # best case
    item = numbers[mid_index]
    if item == target:
        return mid_index

    if item < target:
        # discard all numbers BEFORE mid_index
        first_index = mid_index + 1
        return binary_search_recursion(
            numbers,
            target,
            first_index,
            last_index,
        )
    else:
        # discard all numbers AFTER mid_index
        last_index = mid_index - 1
        return binary_search_recursion(numbers, target, first_index, last_index)
