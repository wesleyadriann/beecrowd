import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    double valor = scanner.nextDouble();

    int notas100 = (int) (valor / 100);
    valor %= 100;

    int notas50 = (int) (valor / 50);
    valor %= 50;

    int notas20 = (int) valor / 20;
    valor %= 20;

    int notas10 = (int) valor / 10;
    valor %= 10;

    int notas5 = (int) valor / 5;
    valor %= 5;

    int notas2 = (int) valor / 2;
    valor = valor % 2 * 10;

    int moeda1 = (int) valor / 10;
    valor %= 10;

    int moeda50 = (int) valor / 5;
    valor %= 5;

    int moeda25 = (int) (valor / 2.5);
    valor %= 2.5;

    int moeda10 = (int) valor;
    valor %= 1;

    int moeda5 = (int) (valor / 0.5);
    valor %= 0.5;

    int moeda01 = (int) (valor / 0.1);

    System.out.printf("NOTAS:\n%d nota(s) de R$ 100.00\n", notas100);
    System.out.printf("%d nota(s) de R$ 50.00\n", notas50);
    System.out.printf("%d nota(s) de R$ 20.00\n", notas20);
    System.out.printf("%d nota(s) de R$ 10.00\n", notas10);
    System.out.printf("%d nota(s) de R$ 5.00\n", notas5);
    System.out.printf("%d nota(s) de R$ 2.00\n", notas2);

    System.out.printf("MOEDAS:\n%d moeda(s) de R$ 1.00\n", moeda1);
    System.out.printf("%d moeda(s) de R$ 0.50\n", moeda50);
    System.out.printf("%d moeda(s) de R$ 0.25\n", moeda25);
    System.out.printf("%d moeda(s) de R$ 0.10\n", moeda10);
    System.out.printf("%d moeda(s) de R$ 0.05\n", moeda5);
    System.out.printf("%d moeda(s) de R$ 0.01\n", moeda01);

  }
}
