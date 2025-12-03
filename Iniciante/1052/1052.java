import java.util.Scanner;

public class Main {
  public static void main(String[] args) {
    Scanner scan = new Scanner(System.in);

    int value = Integer.parseInt(scan.nextLine());

    String result;

    switch (value) {
      case 1:
        result = "January";
        break;
      case 2:
        result = "February";
        break;
      case 3:
        result = "March";
        break;
      case 4:
        result = "April";
        break;
      case 5:
        result = "May";
        break;
      case 6:
        result = "June";
        break;
      case 7:
        result = "July";
        break;
      case 8:
        result = "August";
        break;
      case 9:
        result = "September";
        break;
      case 10:
        result = "October";
        break;
      case 11:
        result = "November";
        break;
      default:
        result = "December";
        break;
    }

    System.out.println(result);
  }
}
