(ns interest-is-interesting)

(defn interest-rate
  "Calculate the interest rate based on the provided balance."
  [balance]
  (cond (< balance 0M) -3.213
        (< balance 1000M) 0.5
        (< balance 5000M) 1.621
        :else 2.475))

(defn annual-balance-update
  "Calculate the annual balance update."
  [balance]
  (+ balance (* balance 0.01M (bigdec (Math/abs (interest-rate balance))))))

(defn amount-to-donate
  "Calculate how much money to donate to charities base on the balance and tax-free percentage."
  [balance tax-free-percentage]
  (if (< balance 0) 0 (int (* balance 0.02M tax-free-percentage))))
