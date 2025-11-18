import java.util.Scanner;
import java.util.regex.Matcher;
import java.util.regex.Pattern;

public class Main {
  public static void main(String[] args) {
    final Pattern fullPattern = Pattern.compile("[a-zA-Z0-9]{6,32}+");
    final Pattern numberPattern = Pattern.compile(".*[0-9].*");
    final Pattern upperPattern = Pattern.compile(".*[A-Z].*");
    final Pattern lowerPattern = Pattern.compile(".*[a-z].*");

    Scanner scanner = new Scanner(System.in);

    while (scanner.hasNextLine()) {
      String pass = scanner.nextLine().trim();
      Matcher fullMacher = fullPattern.matcher(pass);

      if (!fullMacher.matches()) {
        System.out.println("Senha invalida.");
        continue;
      }

      Matcher numberMatcher = numberPattern.matcher(pass);
      Matcher upperMatcher = upperPattern.matcher(pass);
      Matcher lowerMatcher = lowerPattern.matcher(pass);

      if (!numberMatcher.matches() || !upperMatcher.matches() || !lowerMatcher.matches()) {
        System.out.println("Senha invalida.");
        continue;
      }

      System.out.println("Senha valida.");
    }
  }
}
