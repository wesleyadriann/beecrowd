import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    int n = Integer.parseInt(reader.readLine());

    int result = 0;

    for (int i = 0; i < n; i++) {
      String[] values = reader.readLine().split(" ");

      int l = Integer.parseInt(values[0]);
      int c = Integer.parseInt(values[1]);

      if (l > c) {
        result += c;
      }
    }

    System.out.println(result);
  }
}
