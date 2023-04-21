local function is_prime(primes, number)
    for i = 1, #primes do
        if number % primes[i] == 0 then
            return false
        end
    end

    return true
end

return function(nth)
    assert(nth >= 1, "Invalid input")

    if nth == 1 then
        return 2
    end

    local count = 1
    local number = 1
    local primes = { 2 }
    while count < nth do
        number = number + 2
        if is_prime(primes, number) then
            table.insert(primes, number)
            count = count + 1
        end
    end

    return number
end
