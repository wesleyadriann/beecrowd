import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int size = Integer.parseInt(scan.nextLine());
    String[] values = scan.nextLine().split(" ");

    int small = Integer.parseInt(values[0]);
    int smallIndex = 0;
    for (int i = 1; i < size; i++) {
      int value = Integer.parseInt(values[i]);
      if (value < small) {
        small = value;
        smallIndex = i;
      }
    }

    System.out.println(smallIndex + 1);
  }
}
