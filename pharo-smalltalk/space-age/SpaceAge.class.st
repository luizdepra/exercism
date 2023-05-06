Class {
    #name: #SpaceAge,
    #superclass : #Object,
    #category : #'Exercise@SpaceAge'
}

{ #category : #'dunno' }
SpaceAge class >> planetMapping [
    | earthOrbitalPeriod |
    earthOrbitalPeriod := 31557600.
    ^ {
        'Earth' -> earthOrbitalPeriod.
		'Mercury' -> (earthOrbitalPeriod * 0.2408467).
        'Venus' -> (earthOrbitalPeriod * 0.61519726).
        'Mars' -> (earthOrbitalPeriod * 1.8808158).
        'Jupiter' -> (earthOrbitalPeriod * 11.862615).
        'Saturn' -> (earthOrbitalPeriod * 29.447498).
        'Uranus' -> (earthOrbitalPeriod * 84.01684).
        'Neptune' -> (earthOrbitalPeriod * 164.79132).
    } asDictionary
]


{ #category : #'dunno' }
SpaceAge >> ageOnPlanet: planet at: age [
    ^ age / self class planetMapping at: planet
]