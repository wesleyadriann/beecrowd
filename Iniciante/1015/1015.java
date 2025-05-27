import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] p1 = scan.nextLine().split(" ");
    String[] p2 = scan.nextLine().split(" ");

    double x1 = Double.parseDouble(p1[0]);
    double y1 = Double.parseDouble(p1[1]);

    double x2 = Double.parseDouble(p2[0]);
    double y2 = Double.parseDouble(p2[1]);

    double x = Math.pow(x2 - x1, 2);
    double y = Math.pow(y2 - y1, 2);

    System.out.printf("%.4f\n", Math.sqrt(x + y));

  }
}