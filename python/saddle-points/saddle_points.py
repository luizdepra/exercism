def saddle_points(matrix):
    if len({len(row) for row in matrix}) > 1:
        raise ValueError('irregular matrix')

    row_max_list = list(map(max, matrix))
    col_min_list = list(map(min, list(zip(*matrix))))

    return [
        {'row': row + 1, 'column': col + 1}
        for row, row_max in enumerate(row_max_list)
        for col, col_min in enumerate(col_min_list)
        if row_max == col_min
    ]
