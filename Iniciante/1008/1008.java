import java.util.Locale;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);
    scan.useLocale(Locale.US);

    String employee = scan.nextLine();
    int hours = scan.nextInt();
    double salary = scan.nextDouble();

    System.out.println("NUMBER = " + employee);

    String result = String.format(Locale.US, "%.2f", salary * hours);
    System.out.println("SALARY = U$ " + result);
  }
}