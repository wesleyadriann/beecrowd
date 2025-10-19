import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    while (scan.hasNextLine()) {
      int value = Integer.parseInt(scan.nextLine());
      System.out.println(value - 1);
    }
  }
}
