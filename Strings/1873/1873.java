import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    final String player1 = "rajesh";
    final String player2 = "sheldon";

    Scanner scanner = new Scanner(System.in);

    int c = Integer.parseInt(scanner.nextLine());

    for (int i = 0; i < c; i++) {
      String[] words = scanner.nextLine().split(" ");

      String player1Choice = words[0];
      String player2Choice = words[1];

      if (player1Choice.equals(player2Choice)) {
        System.out.println("empate");
        continue;
      }

      if (player1Choice.equals("pedra")) {
        if (player2Choice.equals("papel") || player2Choice.equals("spock")) {
          System.out.println(player2);
          continue;
        } else if (player2Choice.equals("lagarto") || player2Choice.equals("tesoura")) {
          System.out.println(player1);
          continue;
        }
      }

      if (player1Choice.equals("papel")) {
        if (player2Choice.equals("tesoura") || player2Choice.equals("lagarto")) {
          System.out.println(player2);
          continue;
        } else if (player2Choice.equals("pedra") || player2Choice.equals("spock")) {
          System.out.println(player1);
          continue;
        }
      }

      if (player1Choice.equals("tesoura")) {
        if (player2Choice.equals("spock") || player2Choice.equals("pedra")) {
          System.out.println(player2);
          continue;
        } else if (player2Choice.equals("papel") || player2Choice.equals("lagarto")) {
          System.out.println(player1);
          continue;
        }
      }

      if (player1Choice.equals("lagarto")) {
        if (player2Choice.equals("tesoura") || player2Choice.equals("pedra")) {
          System.out.println(player2);
          continue;
        } else if (player2Choice.equals("spock") || player2Choice.equals("papel")) {
          System.out.println(player1);
          continue;
        }
      }

      if (player1Choice.equals("spock")) {
        if (player2Choice.equals("lagarto") || player2Choice.equals("papel")) {
          System.out.println(player2);
          continue;
        } else if (player2Choice.equals("tesoura") || player2Choice.equals("pedra")) {
          System.out.println(player1);
          continue;
        }
      }

      System.out.println("empate");

    }
  }
}
