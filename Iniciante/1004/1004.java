import java.util.Scanner;

public class Main {
  public static void main(String[] arg) {
    Scanner scan = new Scanner(System.in);

    int a = scan.nextInt();
    int b = scan.nextInt();

    System.out.println("PROD = " + (a * b));
  }
}