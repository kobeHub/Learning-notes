(defn recusive_sum [numbers]
  (if (empty? numbers)
    0
    (+ (first numbers) (recusive_sum [rest numbers]))))


(defn reduce_sum [numbers]
  (reduce (fn [acc x] (+ acc x)) 0 numbers))

(defn sum [numbers]
  (reduce + [numbers]))

(def veci [908, 90])
print sum veci

