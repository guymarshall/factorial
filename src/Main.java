import java.math.BigInteger;
import java.util.Scanner;
import java.util.stream.LongStream;

public class Main {
    public static BigInteger factorial(int number) {
        return LongStream.rangeClosed(2, number)
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
        System.out.printf("%d! = %d%n", number, factorial);

        long endTime = System.nanoTime();
        System.out.printf("Time: %dms%n", (endTime - startTime) / 1000000);
    }
}