using System;

class Program {
    static void Main() {
        int t = int.Parse(Console.ReadLine());

        for (int i = 0; i < t; i++) {
            string[] values = Console.ReadLine().Split();
            int r1 = int.Parse(values[0]);
            int r2 = int.Parse(values[1]);

            Console.WriteLine(r1 + r2);
        }
    }
}
