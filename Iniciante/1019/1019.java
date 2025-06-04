import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int n = scan.nextInt();

    int hours = n / 3600;
    n %= 3600;

    int minutes = n / 60;
    n %= 60;

    System.out.printf("%d:%d:%d\n", hours, minutes, n);
  }
}