import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    float n1 = Float.parseFloat(values[0]);
    float n2 = Float.parseFloat(values[1]);
    float n3 = Float.parseFloat(values[2]);
    float n4 = Float.parseFloat(values[3]);

    float median = ((n1 * 2) + (n2 * 3) + (n3 * 4) + (n4 * 1)) / 10.0f;

    System.out.printf("Media: %.1f%n", median);
    if (median >= 7.0) {
      System.out.println("Aluno aprovado.");
    } else if (median >= 5.0 && median < 7.0) {
      System.out.println("Aluno em exame.");
      Float n5 = Float.parseFloat(scan.nextLine());

      System.out.printf("Nota do exame: %.1f\n", n5);
      float final_median = (median + n5) / 2.0f;

      if (final_median >= 5.0) {
        System.out.println("Aluno aprovado.");
      } else {
        System.out.println("Aluno reprovado.");
      }

      System.out.printf("Media final: %.1f\n", final_median);
    } else {
      System.out.println("Aluno reprovado.");
    }
  }
}
