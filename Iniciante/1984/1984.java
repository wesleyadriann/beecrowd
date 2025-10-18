import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String value = new StringBuilder(scan.nextLine().trim())
        .reverse()
        .toString();

    System.out.println(value);
  }
}
