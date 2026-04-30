using System;
using System.Linq;

class Program
{
    static void Main()
    {
        string[] names = {
            "Dasher",
            "Dancer",
            "Prancer",
            "Vixen",
            "Comet",
            "Cupid",
            "Donner",
            "Blitzen",
            "Rudolph"
        };

        string input = Console.ReadLine();
        int total = input.Split().Sum(x => int.Parse(x));

        int index = total % 9;

        if (index == 0)
        {
            index = 9;
        }

        Console.WriteLine(names[index - 1]);
    }
}
