import java.math.BigInteger;
import java.text.DecimalFormat;
import java.text.DecimalFormatSymbols;
import java.text.NumberFormat;
import java.util.Locale;
import java.util.Scanner;

public class Main {
    public static String formatScientific(BigInteger number) {
        if (number.compareTo(BigInteger.valueOf(1000)) <= 0) {
            return number.toString();
        }

        NumberFormat formatter = new DecimalFormat("0.######E0", DecimalFormatSymbols.getInstance(Locale.ROOT));
        return formatter.format(number);
    }

    public static BigInteger calculateFactorial(int number) {
        BigInteger factorial = BigInteger.ONE;

        for (BigInteger i = BigInteger.ONE; i.compareTo(BigInteger.valueOf(number)) <= 0; i = i.add(BigInteger.ONE)) {
            factorial = factorial.multiply(i);
        }

        return factorial;

        /*
        (2..=number)
            .into_par_iter()
            .map(BigInt::from)
            .reduce_with(|a: BigInt, b: BigInt| a * b)
            .unwrap_or(BigInt::from(1))
        */
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