import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    float a = Float.parseFloat(values[0]);
    float b = Float.parseFloat(values[1]);
    float c = Float.parseFloat(values[2]);

    if (a < b + c && b < a + c && c < a + b) {
      System.out.printf("Perimetro = %.1f\n", a + b + c);
    } else {
      System.out.printf("Area = %.1f\n", (a + b) * c / 2);
    }
  }
}
