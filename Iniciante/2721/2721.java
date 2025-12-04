import java.util.Scanner;

public class Main {
  final static String[] NAMES = {
      "Dasher",
      "Dancer",
      "Prancer",
      "Vixen",
      "Comet",
      "Cupid",
      "Donner",
      "Blitzen",
      "Rudolph"
  };

  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    String input = scanner.nextLine();

    int total = 0;

    for (int i = 0; i < input.length(); i++) {
      char character = input.charAt(i);
      if (character == ' ')
        continue;
      total += character - '0';
    }

    int index = total % 9;
    if (index == 0)
      index = 9;

    System.out.println(NAMES[index - 1]);
  }
}
