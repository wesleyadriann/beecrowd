import java.util.Locale;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] item1 = scan.nextLine()
      .split(" ");
    String[] item2 = scan.nextLine()
      .split(" ");

    int item1Qtd = Integer.parseInt(item1[1]);
    double item1Value = Double.parseDouble(item1[2]);

    int item2Qtd = Integer.parseInt(item2[1]);
    double item2Value = Double.parseDouble(item2[2]);

    String result = String.format(
      Locale.US,
      "VALOR A PAGAR: R$ %.2f",
      (item1Qtd * item1Value) + (item2Qtd * item2Value)
    );

    System.out.println(result);
  }
}