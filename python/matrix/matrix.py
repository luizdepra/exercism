class Matrix:
    def __init__(self, matrix_string):
        self.matrix = []
        for row in matrix_string.splitlines():
            column = []
            for value in row.split():
                column.append(int(value))
            self.matrix.append(column)

        self.num_rows = len(self.matrix)
        self.num_columns = len(self.matrix[0])

    def row(self, index):
        if index <= 0 or index > self.num_rows:
            raise Exception("Invalid row index")

        return list(self.matrix[index - 1])


    def column(self, index):
        if index <= 0 or index > self.num_columns:
            raise Exception("Invalid column index")

        return [row[index - 1] for row in self.matrix]
