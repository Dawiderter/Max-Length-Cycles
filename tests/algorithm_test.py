from math import factorial
from typing import Dict, List
import unittest

import matplotlib.pyplot as plt

import src.algorithm as al


def check_if_permutation(permutation : List[int]) -> bool:
    n = len(permutation)
    already_seen = [False for i in range(n)]
    max_item = -1
    for item in permutation:
        if (not 0 <= item < n) or already_seen[item] :
            return False
        max_item = max(item, max_item)
        already_seen[item] = True
    return max_item == n-1


class TestPermuting(unittest.TestCase):
    def test_check(self):
        positive_test_set = [[0,3,2,4,1], [5,4,1,0,2,3], [0]]
        negative_test_set = [[1,2,3,1], [0,1,2,4], [0,2,-1]]

        for test in positive_test_set:
            self.assertTrue(check_if_permutation(test))
        for test in negative_test_set:
            self.assertFalse(check_if_permutation(test))

    def test_perm_correctness(self):
        for i in range(1000):
            per = al.generateSequence(i)
            for i in range(10):
                per = al.permute(per)
                self.assertTrue(check_if_permutation(per))

    def test_perm_distribution(self):
        return
        count : Dict[str, int] = {}
        n = 5
        amount = 10_000
        perm = al.generateSequence(n)
        for i in range(amount):
            perm = al.permute(perm)
            repres = str(perm)
            if repres in count:
                count[repres] += 1
            else:
                count[repres] = 0
        x_number = factorial(n)
        x_range = x_number/1000
        x_ranges = [x_range*i for i in range(0,1001)]
        values = list(count.values())
        print(f"Standard deviation: {np.std(values)}")
        x_line = 0.5 + np.arange(x_range)
        plt.subplot(1, 2, 1)
        plt.bar(x_line, values, width=1, edgecolor="white", linewidth=0.7)
        plt.subplot(1, 2, 2)
        plt.bar(x_line, np.random.uniform(), width=1, edgecolor="white", linewidth=0.7)
        plt.show()

class TestCycles(unittest.TestCase):
    def test_identity(self):
        data = [0,1,2,3,4]
        result = [[0],[1],[2],[3],[4]]
        max_length = 1
        self.assertEqual(result, al.find_cycles(data))
        self.assertEqual(max_length, al.max_length_cycle(data))
    def test_cyclic(self):
        data = [1,2,3,4,0]
        result = [[0,1,2,3,4]]
        max_length = 5
        self.assertEqual(result, al.find_cycles(data))
        self.assertEqual(max_length, al.max_length_cycle(data))
    def test_small_cycles(self):
        data = [1,0,3,2,4]
        result = [[0,1], [2,3], [4]]
        max_length = 2
        self.assertEqual(result, al.find_cycles(data))
        self.assertEqual(max_length, al.max_length_cycle(data))
    def test_medium_cycles(self):
        data = [3,4,1,0,2]
        result = [[0,3], [1,4,2]]
        max_length = 3
        self.assertEqual(result, al.find_cycles(data))
        self.assertEqual(max_length, al.max_length_cycle(data))
    def test_long_cycles(self):
        data = [4,1,3,0,2]
        result = [[0,4,2,3], [1]]
        max_length = 4
        self.assertEqual(result, al.find_cycles(data))
        self.assertEqual(max_length, al.max_length_cycle(data))
    
    def test_average_cycle_number(self):
        averages = []

        for n in range(101):
            sum = 0
            perm = al.generateSequence(n)
            sample = 3_000
            for _ in range(sample):
                perm = al.permute(perm)
                sum += al.number_of_cycles(perm)
            averages.append(sum/(sample))
        
        plt.plot(range(101), averages)
        plt.show()

    def test_max_cycle_length(self):
        averages = []

        for n in range(101):
            sum = 0
            perm = al.generateSequence(n)
            sample = 3_000
            for _ in range(sample):
                perm = al.permute(perm)
                sum += al.max_length_cycle(perm)
            averages.append(sum/(sample))
        
        plt.plot(range(101), averages)
        plt.show()

             

if __name__ == '__main__':
    unittest.main()