import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Collections;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    boolean first = true;
    while (true) {
      int n = Integer.parseInt(reader.readLine());

      if (n == 0)
        break;

      if (first)
        first = false;
      else
        System.out.println();

      ArrayList<String> white_p = new ArrayList<>();
      ArrayList<String> white_m = new ArrayList<>();
      ArrayList<String> white_g = new ArrayList<>();
      ArrayList<String> red_p = new ArrayList<>();
      ArrayList<String> red_m = new ArrayList<>();
      ArrayList<String> red_g = new ArrayList<>();

      for (int i = 0; i < n; i++) {

        String name = reader.readLine();
        String[] type = reader.readLine().split(" ");

        String color = type[0];
        String size = type[1];

        if (color.equals("branco")) {
          switch (size) {
            case "P":
              white_p.add(name);
              break;
            case "M":
              white_m.add(name);
              break;
            default:
              white_g.add(name);
              break;
          }
          continue;
        }

        switch (size) {
          case "P":
            red_p.add(name);
            break;
          case "M":
            red_m.add(name);
            break;
          default:
            red_g.add(name);
            break;
        }
      }

      Collections.sort(white_p);
      white_p.forEach(person -> {
        System.out.println("branco P " + person);
      });

      Collections.sort(white_m);
      white_m.forEach(person -> {
        System.out.println("branco M " + person);
      });

      Collections.sort(white_g);
      white_g.forEach(person -> {
        System.out.println("branco G " + person);
      });

      Collections.sort(red_p);
      red_p.forEach(person -> {
        System.out.println("vermelho P " + person);
      });

      Collections.sort(red_m);
      red_m.forEach(person -> {
        System.out.println("vermelho M " + person);
      });

      Collections.sort(red_g);
      red_g.forEach(person -> {
        System.out.println("vermelho G " + person);
      });
    }
  }
}
