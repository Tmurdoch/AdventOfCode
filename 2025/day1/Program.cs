using System.IO;
using System;

class TestClass {

  public static int Wrap(int value) {
    int max = 100;
    int min = 0;

    if (value == 0) return min;
    int range = max - min;

    int calc = ((((value - min) % range) + range) % range) + min;
    if (value < 0) {
      Console.WriteLine($"less than 0, {value}, calc: {calc}");
    }
    return calc;
    
  } 

  static void Main(string[] args) {
    String? line;
    try {
      StreamReader sr = new StreamReader("C:\\Users\\kewl2\\Projects\\AdventOfCode\\2025\\day1\\input.txt");
      line = sr.ReadLine();
      int count = 0; 
      int current = 50;
      while (line != null) {

        Console.WriteLine($"before {current}");
        //get the number
        char left_right = line[0]; 
        int number = int.Parse(line[1..]);
        if (left_right == 'R') {
          current = Wrap(current + number);
        }
        else if (left_right == 'L') {
          current = Wrap(current - number);
        } 

        if (current == 0) {
          count++;
        }

        Console.WriteLine($"left or right: {left_right}, num: {number}, new current: {current}");

        line = sr.ReadLine();

      }
      Console.WriteLine($"hit 0 #{count} times");

    }
    catch (Exception e) {
      Console.WriteLine("Exception: " + e.Message);
    }


  }
}

