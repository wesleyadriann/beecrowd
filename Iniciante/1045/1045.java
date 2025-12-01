import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStreamReader;
import java.util.Arrays;
import java.util.Collections;

public class Main {
  public static void main(String[] args) throws IOException {
    InputStreamReader stream = new InputStreamReader(System.in);
    BufferedReader reader = new BufferedReader(stream);

    String[] values = reader.readLine().split(" ");

    Double[] parsed = {
        Double.parseDouble(values[0]),
        Double.parseDouble(values[1]),
        Double.parseDouble(values[2])
    };
    Collections.sort(Arrays.asList(parsed), Collections.reverseOrder());

    double a = parsed[0];
    double b = parsed[1];
    double c = parsed[2];

    if (a >= b + c) {
      System.out.println("NAO FORMA TRIANGULO");
      return;
    }

    double pow_a = a * a;
    double pow_b = b * b;
    double pow_c = c * c;

    if (pow_a == pow_b + pow_c) {
      System.out.println("TRIANGULO RETANGULO");
    } else if (pow_a > pow_b + pow_c) {
      System.out.println("TRIANGULO OBTUSANGULO");
    } else if (pow_a < pow_b + pow_c) {
      System.out.println("TRIANGULO ACUTANGULO");
    }

    if (a == b && b == c) {
      System.out.println("TRIANGULO EQUILATERO");
    } else if (a == b || b == c || c == a) {
      System.out.println("TRIANGULO ISOSCELES");
    }
  }
}
