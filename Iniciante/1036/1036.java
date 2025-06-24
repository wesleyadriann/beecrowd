import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    double a = Double.parseDouble(values[0]);
    double b = Double.parseDouble(values[1]);
    double c = Double.parseDouble(values[2]);

    double delta = (b * b) - (4 * a * c);

    if (a < 1 || delta < 0) {
      System.out.println("Impossivel calcular");
    } else {
      double r1 = (-1 * b + Math.sqrt(delta)) / (2 * a);
      double r2 = (-1 * b - Math.sqrt(delta)) / (2 * a);

      System.out.printf("R1 = %.5f\n", r1);
      System.out.printf("R2 = %.5f\n", r2);
    }
  }
}
