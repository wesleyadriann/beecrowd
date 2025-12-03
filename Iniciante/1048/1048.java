import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    Double value = Double.parseDouble(scanner.nextLine());

    Double adjust = 0.04;

    if (value <= 400) {
      adjust = 0.15;
    } else if (value <= 800) {
      adjust = 0.12;
    } else if (value <= 1200) {
      adjust = 0.1;
    } else if (value <= 2000) {
      adjust = 0.07;
    }

    System.out.printf("Novo salario: %.2f\n", value * (adjust + 1));
    System.out.printf("Reajuste ganho: %.2f\n", value * adjust);
    System.out.printf("Em percentual: %.0f %%\n", adjust * 100);
  }
}
