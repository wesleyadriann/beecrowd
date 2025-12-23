import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    String input = reader.readLine().trim();

    if (input.length() >= 10) {
      System.out.println("palavrao");
    } else {
      System.out.println("palavrinha");
    }
  }
}
