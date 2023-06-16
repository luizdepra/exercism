(ns bird-watcher)

(def last-week
  [0, 2, 5, 3, 7, 8, 4])

(defn today
  [birds]
  (last birds))

(defn inc-bird
  [birds]
  (update birds (- (count birds) 1) inc))

(defn day-without-birds?
  [birds]
  (not (every? pos? birds)))

(defn n-days-count
  [birds n]
  (reduce + (take n birds)))

(defn busy-days
  [birds]
  (count (filter #(>= % 5) birds)))

(defn odd-week?
  [birds]
  (every? #(not= (first %) (last %)) (partition 2 1 birds)))
