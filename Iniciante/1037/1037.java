import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    double n = scan.nextDouble();

    if (n < 0 || n > 100) {
      System.out.println("Fora de intervalo");
      return;
    }

    String lBracket = "[";

    if (n > 25) {
      lBracket = "(";
    }

    String interval = "75,100";
    if (n <= 25) {
      interval = "0,25";
    } else if (n <= 50) {
      interval = "25,50";
    } else if (n <= 75) {
      interval = "50,75";
    }

    System.out.println("Intervalo " + lBracket + interval + "]");

  }
}
