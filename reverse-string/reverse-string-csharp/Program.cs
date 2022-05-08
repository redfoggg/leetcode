// Time complexity O(n), Extra space O(1)
// Interview problems - Easy collection
public class Solution {
    public void ReverseString(char[] s) {
        int start = 0;
        int end = s.Length - 1;
        while(start < end)
        {
            s = Swap(s, start, end);
            start++;
            end--;
        }
    }
    
    static char[] Swap(char[] ch, int i, int j)
    {
        char temp = ch[i];
        ch[i] = ch[j];
        ch[j] = temp;
        return ch;
    }
}
