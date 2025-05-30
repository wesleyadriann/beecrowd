import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int x = scan.nextInt();

    double t = (x / 30.0) * 60.0;

    System.out.printf("%.0f minutos\n", t);
  }
}