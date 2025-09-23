import java.util.Scanner;

public class Main {
  public static void main(String args[]) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    int bigger = 0;

    for (int i = 0; i < values.length; i++) {
      int value = Integer.parseInt(values[i]);
      if (value > bigger) {
        bigger = value;
      }
    }

    System.out.println(bigger);
  }
}
