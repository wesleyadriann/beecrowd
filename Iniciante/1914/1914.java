import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    int qt = Integer.parseInt(reader.readLine());

    for (int i = 0; i < qt; i++) {
      String[] players = reader.readLine().split(" ");
      String[] values = reader.readLine().split(" ");

      boolean isEven = (Integer.parseInt(values[0]) + Integer.parseInt(values[1])) % 2 == 0;

      if (isEven) {
        if (players[1].equals("PAR")) {
          System.out.println(players[0]);
          continue;
        }

        System.out.println(players[2]);

      } else {
        if (players[1].equals("IMPAR")) {
          System.out.println(players[0]);
          continue;
        }

        System.out.println(players[2]);
      }
    }
  }
}
