import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    String[] values = reader.readLine()
        .split(" ");

    int x = Integer.parseInt(values[0]);
    int y = Integer.parseInt(values[1]);

    if (x < 0 || y < 0 || x > 432 || y > 468) {
      System.out.println("fora");
    } else {
      System.out.println("dentro");
    }
  }
}
