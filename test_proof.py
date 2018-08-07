"""
File: proof.py
Author: Me
Email: elastic7327@email.com
Github: https://github.com/elastic7327
Description: for pycon2018
"""


import pytest
import example # rust implemented!

from hashlib import sha256


def valid_proof(last_proof, proof, difficulty):
    guess = "{}{}".format(last_proof, proof)
    res = sha256(guess.encode()).hexdigest()
    print(res)
    return "0"*difficulty == res[:difficulty]


def simple_proof_job(difficulty):

    proof = 0

    while (valid_proof(0, proof, difficulty) == False):
        proof += 1

    __import__('ipdb').set_trace()

@pytest.mark.skip(reason="skip it for a moment")
def test_rust_implemented_proof_job_benchmark(benchmark):
    benchmark(example.example, 4)


def test_pure_python_proof_job(benchmark):
    benchmark(simple_proof_job, 2)
