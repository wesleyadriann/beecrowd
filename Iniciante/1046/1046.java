import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    String[] values = scanner.nextLine().split(" ");

    int h1 = Integer.parseInt(values[0]);
    int h2 = Integer.parseInt(values[1]);

    if (h1 >= h2) {
      System.out.printf("O JOGO DUROU %d HORA(S)\n", 24 - (h1 - h2));
    } else {
      System.out.printf("O JOGO DUROU %d HORA(S)\n", h2 - h1);
    }

  }
}
