import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    int n = Integer.parseInt(reader.readLine());

    for (int i = 0; i < n; i++) {
      String[] values = reader.readLine().split(" ");

      int h = Integer.parseInt(values[0]);
      int m = Integer.parseInt(values[1]);
      String o = values[2].equals("0") ? "A porta fechou!" : "A porta abriu!";

      System.out.printf("%02d:%02d - %s\n", h, m, o);
    }

  }
}
