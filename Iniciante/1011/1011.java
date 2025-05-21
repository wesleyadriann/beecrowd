import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    final double PI = 3.14159;

    Scanner scan = new Scanner(System.in);

    double r = scan.nextDouble();

    String result = String.format("VOLUME = %.3f", (4.0/3.0) * PI * Math.pow(r, 3.0));
    System.out.println(result);
  }
}