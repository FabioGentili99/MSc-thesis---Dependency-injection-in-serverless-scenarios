
import static java.lang.System.exit;


public class Hello {
    public static void main(String[] args) {
        // Check if an argument is provided
        if (args.length < 1) {
            System.out.println("Usage: java Main <your_argument>");
            return;
        }
        // Read the first argument (args[0])
        String arg = args[0];
        hello(arg);
        exit(0);
    }

    public static void hello(String arg) {
            System.out.println("hello " + arg);
    }
}