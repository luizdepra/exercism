Class {
    #name: #Isogram,
    #superclass : #Object,
	#category : #'Exercise@Isogram'
}

{ #category : #'dunno' }
Isogram >> isIsogramPhrase: phrase [
    | lowered |
    lowered := phrase asLowercase onlyLetters.
    ^ lowered asSet size = lowered size.
]