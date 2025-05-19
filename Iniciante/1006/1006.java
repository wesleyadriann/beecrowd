import java.util.Locale;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);
    scan.useLocale(Locale.US);

    double a = scan.nextDouble();
    double b = scan.nextDouble();
    double c = scan.nextDouble();

    String result = String.format(Locale.US, "%.1f",  ((a * 2) + (b * 3) + (c * 5)) / 10);

    System.out.println("MEDIA = " + result);
  }
}