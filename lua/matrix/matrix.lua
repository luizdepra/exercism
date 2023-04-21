return function(input)
    local matrix = {}

    for line in input:gmatch('[%d ]+') do
        local row = {}

        for value in line:gmatch('%d+') do
            local number = tonumber(value)

            table.insert(row, number)
        end

        table.insert(matrix, row)
    end

    matrix.row = function(index)
        return matrix[index]
    end

    matrix.column = function(index)
        local col = {}

        for i = 1, #matrix do
            table.insert(col, matrix[i][index])
        end

        return col
    end

    return matrix
end
