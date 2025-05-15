import java.util.Locale;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    double PI = 3.14159;

    Scanner scan = new Scanner(System.in);
    scan.useLocale(Locale.US);

    double n = scan.nextDouble();

    String result = String.format(Locale.US,"%.4f", PI * (n * n));

    scan.close();
    System.out.println("A=" + result);
  }
}