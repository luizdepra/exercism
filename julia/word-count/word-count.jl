function wordcount(sentence)
    words = split(lowercase(sentence), r"('\B|\B'|[^\w'])+"; keepempty=false)

    word_counts = Dict{String, Int}()
    for word in words
        word = lowercase(word)
        word_counts[word] = get(word_counts, word, 0) + 1
    end

    return word_counts
end
