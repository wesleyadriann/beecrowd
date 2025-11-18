import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scanner = new Scanner(System.in);

    boolean line0 = true;
    while (scanner.hasNextLine()) {
      int n = Integer.parseInt(scanner.nextLine());

      if (n == 0) {
        break;
      }

      int longest = 0;
      List<String> words = new ArrayList<>();
      for (int i = 0; i < n; i++) {
        String word = scanner.nextLine().trim();

        if (word.length() > longest) {
          longest = word.length();
        }

        words.add(word);
      }

      if (line0)
        line0 = false;
      else
        System.out.println();

      for (String word : words) {
        String formated = String.format("%" + longest + "s", word);
        System.out.println(formated);
      }
    }
  }
}
