import java.util.Locale;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);
    scan.useLocale(Locale.US);

    scan.nextLine();
    double a = scan.nextDouble();
    double b = scan.nextDouble();

    String result = String.format(Locale.US,"TOTAL = R$ %.2f", a + (b * 0.15));
    System.out.println(result);
  }
}