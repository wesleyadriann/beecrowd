import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    String line1 = reader.readLine();
    String line2 = reader.readLine();
    String line3 = reader.readLine();

    String result;
    if (line1.equals("vertebrado")) {
      if (line2.equals("ave")) {

        if (line3.equals("carnivoro")) {
          result = "aguia";
        } else {
          result = "pomba";
        }

      } else if (line3.equals("onivoro")) {
        result = "homem";
      } else {
        result = "vaca";
      }

    } else if (line2.equals("inseto")) {
      if (line3.equals("hematofago")) {
        result = "pulga";
      } else {
        result = "lagarta";
      }
    } else if (line3.equals("hematofago")) {
      result = "sanguessuga";
    } else {
      result = "minhoca";
    }

    System.out.println(result);
  }
}
