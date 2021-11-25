package com.marshall.guy;

import java.math.BigInteger;
import java.util.Scanner;

public class Main {
    public static BigInteger calculateFactorial(int number) {
        BigInteger result = BigInteger.ONE;
        BigInteger counter = BigInteger.TWO;
        BigInteger userInput = new BigInteger(Integer.toString(number));

        if (number > 1) {
            while (counter.compareTo(userInput) < 1) { // counter <= number
                result = result.multiply(counter);
                counter = counter.add(BigInteger.ONE);
            }
        }
        return result;
    }

    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        System.out.print("Enter an integer: ");
        int userInput = scanner.nextInt();
        long startTime = System.nanoTime();

        System.out.println(calculateFactorial(userInput) + "\n");
        long stopTime = System.nanoTime();
        double timeTaken = (stopTime - startTime) / 1_000_000_000.0;
        System.out.println(userInput + " factorial took " + timeTaken + " seconds to calculate!");
    }
}