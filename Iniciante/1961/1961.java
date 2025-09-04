import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int p = Integer.parseInt(scan.nextLine().split(" ")[0]);

    String[] values = scan.nextLine().trim().split(" ");

    boolean win = true;
    int prev = Integer.parseInt(values[0]);

    for (int i = 1; i < values.length; i++) {
      int value = Integer.parseInt(values[i]);

      int min = prev - p;
      int max = prev + p;
      if (value < min || value > max) {
        win = false;
        break;
      }

      prev = value;
    }

    if (win) {
      System.out.println("YOU WIN");
    } else {
      System.out.println("GAME OVER");
    }
  }
}
