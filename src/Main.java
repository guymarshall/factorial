import java.math.BigDecimal;
import java.math.BigInteger;
import java.text.DecimalFormat;
import java.text.DecimalFormatSymbols;
import java.util.Locale;
import java.util.Scanner;
import java.util.stream.IntStream;

public class Main {
    public static String formatScientific(BigInteger number) {
        BigDecimal numberDecimal = new BigDecimal(number);
        DecimalFormat decimalFormat = new DecimalFormat("0.00E0", DecimalFormatSymbols.getInstance(Locale.ROOT));
        return decimalFormat.format(numberDecimal);
    }

    public static BigInteger factorial(int number) {
        return IntStream.rangeClosed(2, number)
                .parallel()
                .mapToObj(BigInteger::valueOf)
                .reduce(BigInteger.ONE, BigInteger::multiply);
    }

    public static void main(String[] args) {
        System.out.print("Enter number: ");
        Scanner scanner = new Scanner(System.in);
        int number = scanner.nextInt();

        long startTime = System.nanoTime();
        BigInteger factorial = factorial(number);

        System.out.println("Formatting...");
        String factorialFormatted = formatScientific(factorial);
        System.out.printf("%d! = %s%n", number, factorialFormatted);

        long endTime = System.nanoTime();
        System.out.printf("Time: %dms%n", (endTime - startTime) / 1000000);
    }
}