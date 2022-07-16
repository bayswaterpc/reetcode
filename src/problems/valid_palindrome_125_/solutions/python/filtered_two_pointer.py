class Solution:
    def isPalindrome(self, s: str) -> bool:
        f = ""
        for c in s:
            if c.isalnum():
                f += c.lower()
        print(f)
        l, r = 0, len(f)-1
        while l < r:
            if f[l] != f[r]:
                return False
            l += 1
            r -= 1
            
        return True
        
        