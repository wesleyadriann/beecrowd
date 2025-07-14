import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    int a = Integer.parseInt(values[0]);
    int b = Integer.parseInt(values[1]);

    boolean isMult = a % b == 0;

    if (b > a) {
      isMult = b % a == 0;
    }

    if (isMult) {
      System.out.println("Sao Multiplos");
    } else {
      System.out.println("Nao sao Multiplos");
    }
  }
}
