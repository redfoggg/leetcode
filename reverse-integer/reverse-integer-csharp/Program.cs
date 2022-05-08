public class Solution {
    public int Reverse(int x) 
    {
        int reverse = 0;
        int maxInt = int.MaxValue;
        int minInt = int.MinValue;
        while(x != 0)
        {
            var lastDigit = x % 10;

            if(reverse > maxInt/10 || (reverse == maxInt/10 && lastDigit > 7)) return 0;
            if(reverse < minInt/10 || (reverse == minInt/10 && lastDigit < -8)) return 0;
            
            reverse = (reverse * 10) + lastDigit;
            x /= 10;
        }

        return reverse;
    }
}
