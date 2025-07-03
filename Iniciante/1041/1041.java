import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    double x = Double.parseDouble(values[0]);
    double y = Double.parseDouble(values[1]);

    if (x == 0.0 && y == 0.0) {
      System.out.println("Origem");
      return;
    } else if (x == 0.0) {
      System.out.println("Eixo Y");
      return;
    } else if (y == 0.0) {
      System.out.println("Eixo X");
      return;
    }

    int result = 1;

    if (x < 0.0 && y > 0.0) {
      result = 2;
    } else if (x < 0.0 && y < 0.0) {
      result = 3;
    } else if (x > 0.0 && y < 0.0) {
      result = 4;
    }

    System.out.println("Q" + result);
  }
}
