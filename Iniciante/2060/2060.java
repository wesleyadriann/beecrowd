import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int n = Integer.parseInt(scan.nextLine());

    String[] values = scan.nextLine().split(" ");

    int result2 = 0;
    int result3 = 0;
    int result4 = 0;
    int result5 = 0;

    for (int i = 0; i < n; i++) {
      int value = Integer.parseInt(values[i]);

      if (value % 2 == 0) {
        result2++;
      }
      if (value % 3 == 0) {
        result3++;
      }
      if (value % 4 == 0) {
        result4++;
      }
      if (value % 5 == 0) {
        result5++;
      }
    }

    System.out.println(result2 + " Multiplo(s) de 2");
    System.out.println(result3 + " Multiplo(s) de 3");
    System.out.println(result4 + " Multiplo(s) de 4");
    System.out.println(result5 + " Multiplo(s) de 5");
  }
}
