import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    final double PI = 3.14159;

    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine()
      .split(" ");

    double a = Double.parseDouble(values[0]);
    double b = Double.parseDouble(values[1]);
    double c = Double.parseDouble(values[2]);

    System.out.printf("TRIANGULO: %.3f\n", a * c / 2.0);
    System.out.printf("CIRCULO: %.3f\n", c * c * PI);
    System.out.printf("TRAPEZIO: %.3f\n", (a + b) * c / 2.0);
    System.out.printf("QUADRADO: %.3f\n", b *  b);
    System.out.printf("RETANGULO: %.3f\n", a * b);
  }
}