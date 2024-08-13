public class Example {
  private int sum;
  final long l = 32L;
  float f = 1.6f;

  Example() {
    System.err.println("ctor");
  }

  static void init() {
    var example = new Example();
  }

  public int example(int a, final int b) {
    int c = a + b;
    sum = c;
    return c;
  }

  protected String exampleStr() {
    return String.format("%f", 3.1 + f);
  }
}
