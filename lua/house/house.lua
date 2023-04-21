local house = {}

local build_component = function(sub, post)
    return { subject = sub, postmodifier = post }
end

local components = {
    build_component('house', 'Jack built'),
    build_component('malt', 'lay in'),
    build_component('rat', 'ate'),
    build_component('cat', 'killed'),
    build_component('dog', 'worried'),
    build_component('cow with the crumpled horn', 'tossed'),
    build_component('maiden all forlorn', 'milked'),
    build_component('man all tattered and torn', 'kissed'),
    build_component('priest all shaven and shorn', 'married'),
    build_component('rooster that crowed in the morn', 'woke'),
    build_component('farmer sowing his corn', 'kept'),
    build_component('horse and the hound and the horn', 'belonged to'),
}

house.verse = function(which)
    local phrase = 'This is'
    for i = which, 1, -1 do
        phrase = phrase .. ' the ' .. components[i].subject
        if i ~= 1 then
            phrase = phrase .. '\nthat ' .. components[i].postmodifier
        else
            phrase = phrase .. ' that ' .. components[i].postmodifier .. '.'
        end
    end

    return phrase
end

house.recite = function()
    local song = ''
    for i = 1, #components do
        song = song .. house.verse(i) .. '\n'
    end
    song = song:sub(1, -2)

    return song
end

return house
