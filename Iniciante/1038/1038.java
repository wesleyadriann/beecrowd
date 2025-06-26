import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    int code = Integer.parseInt(values[0]);
    int qtd = Integer.parseInt(values[1]);

    double item_value = 0.0;

    switch (code) {
      case 1:
        item_value = 4;
        break;
      case 2:
        item_value = 4.5;
        break;
      case 3:
        item_value = 5;
        break;
      case 4:
        item_value = 2;
        break;
      default:
        item_value = 1.5;
        break;
    }

    System.out.printf("Total: R$ %.2f\n", item_value * qtd);
  }
}
