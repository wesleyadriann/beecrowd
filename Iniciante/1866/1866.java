import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int c = scan.nextInt();

    for (int i = 0; i < c; i++) {
      int n = scan.nextInt();

      if (n % 2 == 0) {
        System.out.println("0");
      } else {
        System.out.println("1");
      }
    }
  }
}
