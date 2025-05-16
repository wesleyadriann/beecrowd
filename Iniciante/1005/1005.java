import java.util.Locale;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);
    scan.useLocale(Locale.US);

    double a = scan.nextDouble();
    double b = scan.nextDouble();
    scan.close();
    
    String result = String.format(Locale.US,"%.5f", ((a * 3.5) + (b * 7.5)) / 11.0);
    System.out.println("MEDIA = " + result);
  }
}