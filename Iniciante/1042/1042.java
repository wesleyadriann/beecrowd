import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    int a = Integer.parseInt(values[0]);
    int b = Integer.parseInt(values[1]);
    int c = Integer.parseInt(values[2]);

    int x = a, y = b, z = c;

    if (x > y) {
      int temp = x;
      x = y;
      y = temp;
    }

    if (y > z) {
      int temp = z;
      z = y;
      y = temp;
    }

    if (x > y) {
      int temp = x;
      x = y;
      y = temp;
    }

    System.out.printf("%d\n%d\n%d\n\n", x, y, z);
    System.out.printf("%d\n%d\n%d\n", a, b, c);
  }

}
