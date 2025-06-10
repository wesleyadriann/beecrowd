import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] input = scan.nextLine().split(" ");

    int a = Integer.parseInt(input[0]);
    int b = Integer.parseInt(input[1]);
    int c = Integer.parseInt(input[2]);
    int d = Integer.parseInt(input[3]);

    Boolean b_gt_c = b > c;
    Boolean d_gt_a = d > a;
    Boolean cd_gt_ab = c + d > a + b;
    Boolean cd_gt_0 = c > -1 && d > -1;
    Boolean a_odd = a % 2 == 0;

    if (b_gt_c && d_gt_a && cd_gt_ab && cd_gt_0 && a_odd) {
      System.out.println("Valores aceitos");
    } else {
      System.out.println("Valores nao aceitos");
    }
  }
}