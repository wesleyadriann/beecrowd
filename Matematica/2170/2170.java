import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    int line = 1;
    while (scanner.hasNextLine()) {
      String[] values = scanner.nextLine().split(" ");

      double x = Double.parseDouble(values[0]);
      double y = Double.parseDouble(values[1]);

      double result = (y / x - 1) * 100;

      System.out.printf("Projeto %d:\n", line);
      System.out.printf("Percentual dos juros da aplicacao: %.2f %%\n\n", result);

      line++;
    }
  }
}
