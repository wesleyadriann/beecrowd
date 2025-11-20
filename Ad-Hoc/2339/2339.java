import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader streamReader = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(streamReader);

    String[] values = reader.readLine().split(" ");

    int c = Integer.parseInt(values[0]);
    int p = Integer.parseInt(values[1]);
    int f = Integer.parseInt(values[2]);

    if (c * f <= p) {
      System.out.println("S");
    } else {
      System.out.println("N");
    }
  }
}
