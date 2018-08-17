import string
import random
import doubles

# Python ZIP version
def count_doubles(val):
    total = 0
    # there is an improved version later on this post
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_rust(benchmark):   #  <-- Benchmark the Rust version
    benchmark(doubles.count_doubles, val)
