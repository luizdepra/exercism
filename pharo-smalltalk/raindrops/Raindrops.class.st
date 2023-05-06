Class {
    #name: #Raindrops,
    #superclass : #Object,
  #category : #'Exercise@Raindrops'
}

{ #category : #'dunno' }
Raindrops >> convertNumber: number [
    ^ String streamContents: [ :stream |
        (number isDivisibleBy: 3)
            ifTrue: [ stream nextPutAll: 'Pling' ].
        (number isDivisibleBy: 5)
            ifTrue: [ stream nextPutAll: 'Plang' ].
        (number isDivisibleBy: 7)
            ifTrue: [ stream nextPutAll: 'Plong' ].
        stream isEmpty
            ifTrue: [ stream nextPutAll: number asString ] ]
]