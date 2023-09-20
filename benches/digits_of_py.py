def make_pi():
    q, r, t, k, m, x = 1, 0, 1, 1, 3, 3

    j = 0
    while 1:
        if 4 * q + r - t < m * t:
            yield m
            q, r, t, k, m, x = 10*q, 10*(r-m*t), t, k, (10*(3*q+r))//t - 10*m, x
        else:
            q, r, t, k, m, x = q*k, (2*q+r)*x, t*x, k+1, (q*(7*k+2)+r*x)//(t*x), x+2
        j += 1


my_array = []

gen = make_pi()
for i in range(10000):
    my_array.append(str(next(gen)))

my_array = my_array[:1] + ['.'] + my_array[1:]


with open("digits_of_pi.txt", "w") as f:
    f.write("".join(my_array))