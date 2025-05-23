import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] line = scan.nextLine().split(" ");

    int a = Integer.parseInt(line[0]);
    int b = Integer.parseInt(line[1]);
    int c = Integer.parseInt(line[2]);

    int max_ab = (a + b + Math.abs(a - b)) / 2;
    int bigger = (max_ab + c + Math.abs(max_ab - c)) / 2;

    System.out.println(bigger + " eh o maior");
  }
}