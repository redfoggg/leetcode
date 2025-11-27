def Solution:
    def romanToInt(self, s: str) -> int:
        total = 0
        roman = {"I": 1
                 "V": 5
                 "X": 10
                 "L": 50
                 "C": 100
                 "D": 500
                 "M": 1000}
        for a, b in zip(s, s[1:]) # same as having s[i], s[i + 1]
            if roman[a] < roman[b]: # let's take IV a = 1 and b = 5, a < b, so res = res - 1 = -1, since there is no new iteration, goes to return which will be -1 + roman["V"], which is 5, returning 4 as it should.
                res -= roman[a]
            else:
                res += roman[a]
        return res + roman[s[-1]]
