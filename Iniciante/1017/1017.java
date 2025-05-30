import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int t = scan.nextInt();
    int v = scan.nextInt();

    System.out.printf("%.3f\n", t * v / 12.0);
  }
}