import java.math.BigInteger;
import java.text.DecimalFormat;
import java.text.DecimalFormatSymbols;
import java.text.NumberFormat;
import java.util.Locale;
import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static String formatScientific(BigInteger number) {
        if (number.compareTo(BigInteger.valueOf(1000)) <= 0) {
            return number.toString();
        }

        NumberFormat formatter = new DecimalFormat("0.######E0", DecimalFormatSymbols.getInstance(Locale.ROOT));
        return formatter.format(number);
    }

    public static BigInteger calculateFactorial(int number) {
        return IntStream.rangeClosed(2, number)
                .parallel()
                .mapToObj(BigInteger::valueOf)
                .reduce(BigInteger.ONE, BigInteger::multiply);
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.println("Enter number:");
        int number = scanner.nextInt();

        BigInteger factorial = calculateFactorial(number);

        System.out.println("Formatting...");
        String factorialFormatted = formatScientific(factorial);
        System.out.printf("%d! = %s%n", number, factorialFormatted);
    }
}