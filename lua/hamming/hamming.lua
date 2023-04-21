local Hamming = {}

function Hamming.compute(seq_a,seq_b)
    if #seq_a ~= #seq_b then
        return -1
    end

    local distance = 0

    for i = 1, #seq_a do
        local a_char = seq_a:sub(i, i)
        local b_char = seq_b:sub(i, i)

        if a_char ~= b_char then
            distance = distance + 1
        end
    end

    return distance
end

return Hamming
