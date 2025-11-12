import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    String[] values = scan.nextLine().split(" ");

    int h1 = Integer.parseInt(values[0]);
    int m1 = Integer.parseInt(values[1]);

    int h2 = Integer.parseInt(values[2]);
    int m2 = Integer.parseInt(values[3]);

    int t1InMinutes = h1 * 60 + m1;
    int t2InMinutes = h2 * 60 + m2;

    int diff = t2InMinutes - t1InMinutes;

    if (t1InMinutes >= t2InMinutes) {
      diff = 24 * 60 + diff;
    }

    int hours = diff / 60;
    int minutes = diff % 60;

    System.out.printf("O JOGO DUROU %d HORA(S) E %d MINUTO(S)\n", hours, minutes);
  }
}
