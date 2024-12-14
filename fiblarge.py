import sys

n = int(sys.argv[1])
a = 1
b = 1
for _ in range(n // 2):
    a += b
    b += a
sys.set_int_max_str_digits(3 * n)
print(b)
