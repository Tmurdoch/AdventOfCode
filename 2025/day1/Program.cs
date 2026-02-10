using System.IO;
using System;

class TestClass
{

    public static (int, int) Wrap(int number, int current, char left_right)
    {
        int count = 0;
        for (int i = number; i > 0; i--)
        {

            if (current == 0) count++;
            
            //Console.WriteLine($"i: {i}, current: {current}");
            if (left_right == 'L')
            {
                if (current == 0)
                {
                    current = 99;
                }
                else
                {
                    current = current - 1;

                }
            }
            else // right
            {
                if (current == 99)
                {
                    current = 0;
                }
                else
                {
                    current = current + 1;
                }

            }

        }
        return (count, current);
    }

    static void Main(string[] args)
    {
        String? line;
        try
        {
            StreamReader sr = new StreamReader("C:\\Users\\kewl2\\Projects\\AdventOfCode\\2025\\day1\\input.txt");
            line = sr.ReadLine();
            int count = 0;
            int current = 50;
            while (line != null)
            {

                Console.WriteLine($"before {current}");
                //get the number
                char left_right = line[0];
                int number = int.Parse(line[1..]);
                (int ret_count, int ret_current) = Wrap(number, current, left_right);

                Console.WriteLine($"left or right: {left_right}, old_current: {current}, num: {number}, new current: {ret_current}, count: {ret_count}");
                count += ret_count;
                current = ret_current;


                line = sr.ReadLine();

            }

            (int test1, int test2) = Wrap(1000, 50, 'R');
            Console.WriteLine($" new current: {test2}, count: {test1}");

            System.Diagnostics.Debug.Assert(Wrap(48, 52, 'R') == (0, 0));
            System.Diagnostics.Debug.Assert(Wrap(51, 48, 'R') == (0, 99));
            System.Diagnostics.Debug.Assert(Wrap(1, 0, 'L') == (1, 99));
            System.Diagnostics.Debug.Assert(Wrap(2, 1, 'L') == (1, 99));
            System.Diagnostics.Debug.Assert(Wrap(1, 99, 'R') == (0, 0));
            System.Diagnostics.Debug.Assert(Wrap(250, 50, 'R') == (2, 0));
            System.Diagnostics.Debug.Assert(Wrap(249, 50, 'R') == (2, 99));
            System.Diagnostics.Debug.Assert(Wrap(249, 50, 'L') == (2, 1));
            System.Diagnostics.Debug.Assert(Wrap(1, 0, 'R') == (1, 1));
            System.Diagnostics.Debug.Assert(Wrap(5, 0, 'L') == (1, 95));





            System.Diagnostics.Debug.Assert(Wrap(1000, 50, 'R') == (10, 50));




            Console.WriteLine($"coutn: {test1}, current: {test2}");


            Console.WriteLine($"hit 0 #{count} times");

        }
        catch (Exception e)
        {
            Console.WriteLine("Exception: " + e.Message);
        }


    }
}

