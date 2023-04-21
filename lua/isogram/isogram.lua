return function(input)
    local exist = {}

    input = input:lower()
    for char in input:gmatch('%a') do
        if exist[char] then
            return false
        end

        exist[char] = true
    end

    return true
end
