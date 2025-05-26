import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int km = scan.nextInt();
    double l = scan.nextDouble();

    System.out.printf("%.3f km/l\n", km/l);
  }
}