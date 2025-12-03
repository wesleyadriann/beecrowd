import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    Double salary = Double.parseDouble(scanner.nextLine());

    if (salary <= 2000) {
      System.out.println("Isento");
      return;
    }

    if (salary <= 3000) {
      Double tax = (salary - 2000) * 0.08;
      System.out.printf("R$ %.2f\n", tax);
      return;
    }

    if (salary <= 4500) {
      Double tax = (salary - 3000) * 0.18 + (1000 * 0.08);
      System.out.printf("R$ %.2f\n", tax);
      return;
    }

    Double tax = (salary - 4500) * 0.28 + (1500 * 0.18) + (1000 * 0.08);
    System.out.printf("R$ %.2f\n", tax);
  }
}
