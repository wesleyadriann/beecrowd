import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int a = scan.nextInt();
    int b = scan.nextInt();

    scan.close();
    System.out.println("SOMA = " + (a + b));
  }
}