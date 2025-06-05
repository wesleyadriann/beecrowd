import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int d = scan.nextInt();

    int a = d / 364;
    d %= 365;

    int m = d / 30;
    d %= 30;
    
    System.out.println(a + " ano(s)");
    System.out.println(m + " mes(es)");
    System.out.println(d+ " dia(s)");
  }
}