Class {
    #name: #Acronym,
    #superclass : #Object,
	#category : #'Exercise@Acronym'
}

{ #category : #'dunno' }
Acronym >> abbreviatePhrase: phrase [
    | letters parts |
    parts := phrase substrings: ' -'.
    letters := parts collect: [:a | (a copyWithout: $_) first ].
    ^ (letters joinUsing: '') asUppercase
]