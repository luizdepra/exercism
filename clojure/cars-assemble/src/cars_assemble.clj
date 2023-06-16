(ns cars-assemble)

(defn production-rate
  "Returns the assembly line's production rate per hour,
   taking into account its success rate"
  [speed]
  (* (* speed 221) (cond (= speed 10) 0.77
                         (= speed 9) 0.8
                         (> speed 4) 0.9
                         :else 1.0)))

(defn working-items
  "Calculates how many working cars are produced per minute"
  [speed]
  (int (/ (production-rate speed) 60)))
